// use std::io;
// use std::io::Read;  // the trait Read provides read_exact
use std::io::{stdin, stdout, Write};
use std::process::ExitCode;
use termion::event::Key;
use termion::input::TermRead; // provides keys()
use termion::raw::IntoRawMode;

const ROWS: usize = 20;
const COLS: usize = 10;
const TROFF: usize = 1;
const TCOFF: usize = 5;

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
        let s = format!("{}", termion::clear::All);
        self.dp.write(s.as_bytes()).unwrap();

        for i in 0..ROWS {
            let s = format!("{}", termion::cursor::Goto(
                    (TCOFF+1) as u16, (TROFF+i+1) as u16));
            self.dp.write(s.as_bytes()).unwrap();
            for _ in 0..COLS {
                let s = " . ";
                self.dp.write(s.as_bytes()).unwrap();
            }
        }
        let s = format!("{}", termion::cursor::Goto(1, 1));
        self.dp.write(s.as_bytes()).unwrap();
    }
    fn message(&mut self, msg: &str) {
        let s = format!("{}{}", termion::cursor::Goto(1, 1), msg);
        self.dp.write(s.as_bytes()).unwrap();
    }
    fn update(&mut self, &can: Can, &curfig: Figure) {
        // XXX Later
        let points = [Point(0,0), 4];
        // XXX if OK(curfig)
        curfig.land(points);

        let field = vec![vec![false; COLS]; ROWS];
        // XXX smash with can

        for i in 0..4 {
            let p = points[i];
            field[p.row][p.col] = true;
        }

        for i in 0..ROWS {
            let s = format!("{}", termion::cursor::Goto(
                    (TCOFF+1) as u16, (TROFF+i+1) as u16));
            self.dp.write(s.as_bytes()).unwrap();
            // XXX optimize
            for j in 0..COLS {
                if field[i][j] {
                    let s = "[_]";
                    self.dp.write(s.as_bytes()).unwrap();
                } else {
                    let s = " . ";
                    self.dp.write(s.as_bytes()).unwrap();
                }
            }
        }
    }
}

struct Point {
    col: u16,
    row: u16,
}

pub trait Figure {
    fn init(&self, cols: u16, rows: u16);
    fn land(&self, rv: &mut Point[4]);
}

struct barFigure {
    pos: Point,
    points: [Point, 3],
}

enum GenericFigure {
   bar(BarFigure),
}

impl Figure for GenericFigure {
    fn init(&self, cols: u16, rows: u16) {
        self.pos = Point(cols, rows);
        // XXX
        self.points[0] = Point(1, 1);
        self.points[1] = Point(1, 1);
        self.points[2] = Point(1, 1);
    }
    fn land(&self, &mut rv Point[4]) {
        // XXX
        rv[0] = Point(2, 1);
        rv[1] = Point(3, 1);
        rv[2] = Point(4, 1);
        rv[3] = Point(5, 1);
    }
}

fn new_figure(&mut fig: GenericFigure) {
    let bar = GenericFigure::bar(Point(1,1), [Point(1,1), Point(1,1), Point(1,1)]);
    fig = bar;
}

// This basically exists in order to drop termion's RawTerminal restorer.
fn game() -> ExitCode {

    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    // let mut field: Vec<Vec<bool>> = Vec::new();
    let mut field = vec![vec![false; COLS]; ROWS];
    let mut can = Can {
        cols: COLS,
        rows: ROWS,
        matrix: field,
    };
    let mut dp = DisplayOne {
        matrix: vec![vec![false; COLS]; ROWS],
        dp: &mut stdout,
    };
    dp.erase();

    let curfig = GenericFigure();
    new_figure(curfig, COLS, ROWS);
    dp.update(can, curfig);

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
    let s = format!("{}", termion::cursor::Goto(1, (TROFF+ROWS+1) as u16));
    dp.dp.write(s.as_bytes()).unwrap();
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
