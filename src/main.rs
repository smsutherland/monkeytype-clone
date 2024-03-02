use color_eyre::{eyre::WrapErr as _, Result};
use compact_str::CompactString;
use crossterm::event::{self, Event, KeyEventKind};
use language::Word;
use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize as _},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Widget, Wrap,
    },
    Frame,
};
use std::time::Duration;

mod errors;
mod language;
mod tui;

fn main() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    App {
        text: language::generate_string(language::Language::English1k, 50).into(),
        ..App::default()
    }
    .run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}

#[derive(Debug, Default)]
struct App<'a> {
    exit: bool,
    text: Vec<Word<'a>>,
    typed: Vec<CompactString>,
    current_word: CompactString,
}

impl App<'_> {
    fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|f| self.render_frame(f))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
        // TODO: find a way to go from index to position in the paragraph and put the cursor there.
    }

    fn handle_events(&mut self) -> Result<()> {
        while let Ok(true) = event::poll(Duration::ZERO) {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) {
        match key_event.code {
            event::KeyCode::Esc => self.exit = true,
            event::KeyCode::Char(' ') => {
                let new_word =
                    std::mem::replace(&mut self.current_word, CompactString::with_capacity(0));
                self.typed.push(new_word);
                if self.typed.len() == 50 {
                    self.exit = true;
                }
            }
            event::KeyCode::Char(c) => self.current_word.push(c),
            event::KeyCode::Backspace => {
                self.current_word.pop();
            }
            _ => {}
        }
    }
}

impl Widget for &App<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Title::from(" MonkeyType but in Rust ".bold());
        let instructions = Title::from(vec![" ".into(), "<ESC>".bold().blue(), " Quit ".into()]);
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);
        Paragraph::new(create_text(&self.text, &self.typed, &self.current_word))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left)
            .block(block)
            .style((Color::default(), Color::Indexed(236)))
            .render(area, buf);
    }
}

fn create_text<'a>(
    text: &[&'a str],
    typed: &'a [CompactString],
    current_word: &'a str,
) -> Text<'a> {
    const STYLE_CORRECT: Style = Style::new().fg(Color::Green);
    const STYLE_INCORRECT: Style = Style::new().fg(Color::LightRed);
    const STYLE_EXTRA: Style = Style::new().fg(Color::Red);
    const STYLE_MISSING: Style = Style::new().fg(Color::Blue);

    let mut line = Line::default();

    for (i, typed) in typed.iter().enumerate() {
        let mut range_start = 0;
        let word = text[i];
        let mut correct = true;

        for ((i1, c1), c2) in word.char_indices().zip(typed.chars()) {
            if (c1 == c2) != correct {
                line.spans.push(Span::styled(
                    &word[range_start..i1],
                    if correct {
                        STYLE_CORRECT
                    } else {
                        STYLE_INCORRECT
                    },
                ));
                correct = !correct;
                range_start = i1;
            }
        }

        line.spans.push(Span::styled(
            &word[range_start..typed.len().min(word.len())],
            if correct {
                STYLE_CORRECT
            } else {
                STYLE_INCORRECT
            },
        ));

        if word.len() < typed.len() {
            line.spans
                .push(Span::styled(&typed[word.len()..], STYLE_EXTRA));
        }

        if word.len() > typed.len() {
            line.spans
                .push(Span::styled(&word[typed.len()..], STYLE_EXTRA));
        }

        line.spans.push(Span::raw(" "));
    }

    {
        let mut range_start = 0;
        let mut correct = true;
        let word = text[typed.len()];
        for ((i1, c1), c2) in word.char_indices().zip(current_word.chars()) {
            if (c1 == c2) != correct {
                line.spans.push(Span::styled(
                    &word[range_start..i1],
                    if correct {
                        STYLE_CORRECT
                    } else {
                        STYLE_INCORRECT
                    },
                ));
                correct = !correct;
                range_start = i1;
            }
        }
        line.spans.push(Span::styled(
            &word[range_start..current_word.len().min(word.len())],
            if correct {
                STYLE_CORRECT
            } else {
                STYLE_INCORRECT
            },
        ));

        if current_word.len() < word.len() {
            line.spans
                .push(Span::styled(&word[current_word.len()..], STYLE_MISSING));
        }
        if current_word.len() > word.len() {
            line.spans
                .push(Span::styled(&current_word[word.len()..], STYLE_EXTRA));
        }

        line.spans.push(Span::raw(" "));
    }

    for &word in &text[typed.len() + 1..] {
        line.spans.push(Span::styled(word, STYLE_MISSING));
        line.spans.push(Span::raw(" "));
    }

    Text::from(line)
}
