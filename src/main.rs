use colored::*;
use std::io::{stdin, stdout, Error};
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;


fn main() -> Result<(), Error> {
    let stdin = stdin();
    let stdout = stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let terminal_size = termion::terminal_size().unwrap();
    let width = terminal_size.0;
    let heigth = terminal_size.1;

    terminal.hide_cursor()?;
    terminal.clear()?;

    terminal.draw(|mut f| {
        let size = f.size();
        Block::default()
            .borders(Borders::ALL)
            .render(&mut f, size);
    })?;

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            _ => continue,
        }
    }

    terminal.show_cursor()?;
    terminal.clear()?;

    Ok(())
}