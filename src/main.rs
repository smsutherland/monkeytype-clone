use console::{style, Term};
use crossterm::{
    cursor::{MoveLeft, MoveToColumn},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{io::Write, time::Instant};
use strcursor::StrCursor;

mod language {
    pub const ENGLISH_1K: &str = include_str!("../data/english_1k.list");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut term = Term::stdout();
    enable_raw_mode()?;

    let s = create_sentence();
    write!(term, "{}", style(&s).color256(59))?;
    execute!(term, MoveToColumn(0))?;
    let mut letters = String::with_capacity(s.len());
    let mut chars = StrCursor::new_at_start(&s);
    let mut start_time = None;
    'main: while letters.len() != s.len() {
        loop {
            let c2 = term.read_key()?;
            if start_time.is_none() {
                start_time = Some(Instant::now());
            }
            match c2 {
                console::Key::Shift => {}
                console::Key::Backspace => {
                    let c = chars.prev_cp();
                    if let Some((c, chars_new)) = c {
                        execute!(term, MoveLeft(1), Print(style(c).color256(59)), MoveLeft(1))?;
                        chars = chars_new;
                        letters.pop();
                    }
                }
                console::Key::Char(c2) => {
                    let (c, chars_new) = chars.next_cp().unwrap();
                    chars = chars_new;
                    if c2 == c {
                        write!(term, "{}", style(c).green())?;
                    } else {
                        write!(term, "{}", style(c).red())?;
                    }
                    letters.push(c2);
                    break;
                }
                console::Key::CtrlC => break 'main,
                _ => {
                    // Do nothing for keys we don't recognize yet.
                }
            }
        }
    }
    let end_time = Instant::now();
    let start_time = start_time.unwrap();
    let duration = end_time - start_time;
    let wpm = (letters.len() as f64 / 5.0) / (duration.as_secs_f64() / 60.0);
    disable_raw_mode()?;
    println!();
    println!("WPM: {wpm:.2}");
    Ok(())
}

fn create_sentence() -> String {
    let words = language::ENGLISH_1K.lines().collect::<Vec<_>>();
    let count = words.len();
    let mut sentence = String::new();
    for i in 0..10 {
        if i != 0 {
            sentence.push(' ');
        }
        let word = words[fastrand::usize(..count)];
        sentence.push_str(word);
    }
    sentence
}
