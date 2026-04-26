// use std::io;
// use std::io::Read;  // the trait Read provides read_exact
use std::io::{stdin, stdout, Write};
use std::process::ExitCode;
use termion::event::Key;
use termion::input::TermRead; // provides keys()
use termion::raw::IntoRawMode;

// struct
// ....
//    array Matrix


fn main() -> ExitCode {

    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => {
                break;
            }
            Key::Char('q') => {
                break;
            }
            Key::Left => {
                stdout.write(b"<").unwrap();
            }
            Key::Right => {
                stdout.write(b">").unwrap();
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }

    // XXX Restore terminal to cooked before printing this.
    println!("Goodbye");
    ExitCode::from(0)
}

//
//    let size = termion::terminal_size().unwrap();
//    Self {
//        terminal_size: Coordinates {
//            x: size.0 as usize,
//            y: size.1 as usize,
//        },
//    }

//    fn set_pos(&mut self, x: usize, y: usize) {
//        self.cur_pos.x = x;
//        self.cur_pos.y = y;
//        println!(
//            "{}",
//            termion::cursor::Goto(
//                self.cur_pos.x as u16, (self.cur_pos.y) as u16)
//        );
//    }
