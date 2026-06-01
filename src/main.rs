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
    fn Flush(&mut self);
    fn Erase(&mut self);
    fn Message(&mut self, msg: &str);
}

impl<'a> Display for DisplayOne<'a> {
    fn Flush(&mut self) {
        self.DP.flush().unwrap();
    }
    fn Erase(&mut self) {
        let s = format!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        self.DP.write(s.as_bytes()).unwrap();
    }
    fn Message(&mut self, msg: &str) {
        let s = format!("{}{}", termion::cursor::Goto(1, 1), msg);
        self.DP.write(s.as_bytes()).unwrap();
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
    //     Matrix: field,
    // };
    let mut dp = DisplayOne {
        Matrix: vec![vec![false; COLS]; ROWS],
        DP: &mut stdout,
    };
    dp.Erase();

    dp.Flush();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => {
                break;
            }
            Key::Char('q') => {
                break;
            }
            Key::Left => {
                dp.Message("<");
            }
            Key::Right => {
                dp.Message(">");
            }
            _ => {}
        }
        dp.Flush();
    }
    dp.Flush();

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
