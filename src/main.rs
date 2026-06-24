// use std::io;
// use std::io::Read;  // the trait Read provides read_exact
use std::io::{stdin, stdout, Write};
use std::process::ExitCode;
use termion::event::Key;
use termion::input::TermRead; // provides keys()
use termion::raw::IntoRawMode;

const ROWS: usize = 20;
const COLS: usize = 10;

struct Can {
    cols: i32,
    rows: i32,
    matrix: Vec<Vec<bool>>,
}

struct DisplayOne<'a> {
    // This looks exactly like a Can but represents what is displayed
    // at present.
    matrix: Vec<Vec<bool>>,
    dp: &'a mut dyn Write,
}

pub trait Display {
    fn flush(&mut self);
    fn erase(&mut self);
    fn message(&mut self, msg: &str);
}

impl<'a> Display for DisplayOne<'a> {
    fn flush(&mut self) {
        self.dp.flush().unwrap();
    }
    fn erase(&mut self) {
        let s = format!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        self.dp.write(s.as_bytes()).unwrap();


    }
    fn message(&mut self, msg: &str) {
        let s = format!("{}{}", termion::cursor::Goto(1, 1), msg);
        self.dp.write(s.as_bytes()).unwrap();
    }
}

// This basically exists in order to drop termion's RawTerminal restorer.
fn game() -> ExitCode {

    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    // let mut field: Vec<Vec<bool>> = Vec::new();
    // let mut field = vec![vec![false; COLS]; ROWS];
    // let mut can = Can {
    //     cols: COLS,
    //     rows: ROWS,
    //     matrix: field,
    // };
    let mut dp = DisplayOne {
        matrix: vec![vec![false; COLS]; ROWS],
        dp: &mut stdout,
    };
    dp.erase();

    dp.flush();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => {
                break;
            }
            Key::Char('q') => {
                break;
            }
            Key::Left => {
                dp.message("<");
            }
            Key::Right => {
                dp.message(">");
            }
            _ => {}
        }
        dp.flush();
    }
    dp.flush();

    ExitCode::from(0)
}

fn main() -> ExitCode {

    let ec = game();

    println!("Goodbye");
    ec
}

//
//    let size = termion::terminal_size().unwrap();
//    Self {
//        terminal_size: Coordinates {
//            x: size.0 as usize,
//            y: size.1 as usize,
//        },
//    }
