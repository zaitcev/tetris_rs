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
    Matrix: Vec<Vec<bool>>,
}

struct DisplayOne<'a> {
    // This looks exactly like a Can but represents what is displayed
    // at present.
    Matrix: Vec<Vec<bool>>,
    // DP: RawTerminal, // error
    // DP: &Write, // error: expected a type, found a trait
    //             // you can add the `dyn` keyword if you want a trait object
    // DP: &dyn Write,  // error: expected named lifetime parameter
    // DP: &mut Write,  // error: expected named lifetime parameter
    // DP: &mut dyn Write,  // error: expected named lifetime parameter
    // DP: &'a Write,   // error: expected a type, found a trait
    // DP: &'a dyn Write, // error later: self.DP ^ cannot borrow as mutable
    // DP: &'a mut dyn Write, // error later: DP: &stdout types differ in mutability
    DP: &'a mut dyn Write,
}

pub trait Display {
    fn Erase(&mut self);
}

impl<'a> Display for DisplayOne<'a> {
    fn Erase(&mut self) {
        let s = format!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        self.DP.write(s.as_bytes()).unwrap();
    }
}

fn main() -> ExitCode {

    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    // let mut field: Vec<Vec<bool>> = Vec::new();
    // let mut field = vec![vec![false; COLS]; ROWS];
    // let mut can = Can {
    //     cols: COLS,
    //     rows: ROWS,
    //     Matrix: field,
    // };
    let mut dp = DisplayOne {
        Matrix: vec![vec![false; COLS]; ROWS],
        DP: &mut stdout,
    };
    dp.Erase();

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
    println!("Goodbye\r");
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
