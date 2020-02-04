use colored::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let terminal_size = termion::terminal_size().unwrap();

    let width = terminal_size.0;
    let heigth = terminal_size.1;

    write!(stdout, "{}{}{}{} - {}",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1), 
           // Hide the cursor.
           termion::cursor::Hide,
           terminal_size.0,
           terminal_size.1,
        ).unwrap();
    // Flush stdout (i.e. make the output appear).
    stdout.flush().unwrap();



    let mut down = 1;
    let mut right = 1;

    

    for c in stdin.keys() {
        let movement: (i16, i16) = match c.unwrap() {
            Key::Char('q') => break,
            Key::Up => (-1, 0),
            Key::Right => (0, 1),
            Key::Down => (1, 0),
            Key::Left => (0, -1),
            Key => (0, 0),
        };
        
        if  down + movement.0 > 0i16 && down + movement.0 <= terminal_size.1 as i16 {
            down += movement.0;
        }
        if right + movement.1 * 2 > 0i16 && right + movement.1 * 2 <= terminal_size.0 as i16 - 1 {
            right += movement.1 * 2;
        }

        write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(right as u16, down as u16), "  ".on_blue()).unwrap();
        stdout.flush().unwrap();
    }

    // Show the cursor again before we exit.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn startup() {

}