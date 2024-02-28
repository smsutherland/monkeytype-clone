use console::{style, Term};
use crossterm::{
    cursor::{MoveLeft, MoveToColumn},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::Write;
use strcursor::StrCursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut term = Term::stdout();
    enable_raw_mode()?;

    let s = "Word1 Word2";
    write!(term, "{}", style(s).color256(59))?;
    execute!(term, MoveToColumn(0))?;
    let mut letters = String::with_capacity(s.len());
    let mut chars = StrCursor::new_at_start(s);
    'main: while letters.len() != s.len() {
        loop {
            let c2 = term.read_key()?;
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
                        write!(term, "{}", style(c2).green())?;
                    } else {
                        write!(term, "{}", style(c2).red())?;
                    }
                    letters.push(c2);
                    break;
                }
                console::Key::CtrlC => break 'main,
                other => todo!("{:?}", other),
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}
