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

#[derive(Copy)]
struct Point {
    col: u16,
    row: u16,
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point{col: self.col, row: self.row}
    }
}

struct Can {
    cols: usize,
    rows: usize,
    matrix: Vec<Vec<bool>>,
}

// impl Can {
//     xxxx
// }

pub trait Figure {
    fn land(&self, rv: &mut [Point; 4]);
    fn down1(&mut self, can: &Can) -> bool;
}

struct BarFigure {
    pos: Point,
    points: [Point; 3],
}

enum GenericFigure {
    None,
    Bar(BarFigure),
}

impl BarFigure {
    fn new(cols: u16, rows: u16) -> BarFigure {
        BarFigure {
            pos: Point{col: cols/2 - 1, row: rows-1},
            points: [
                Point{col: cols/2 - 2, row: rows-1},
                Point{col: cols/2 + 0, row: rows-1},
                Point{col: cols/2 + 1, row: rows-1},
            ]
        }
    }
}

impl Figure for BarFigure {
    fn land(&self, rv: &mut [Point; 4]) {
        rv[0] = self.pos;
        rv[1] = self.points[0];
        rv[2] = self.points[1];
        rv[3] = self.points[2];
    }

    // XXX This is fully generic, where to put this?
    fn down1(&mut self, can: &Can) -> bool {
        let mut points: [Point; 4] = [Point{col:0,row:0}; 4];
        self.land(&mut points);
        for i in 0..4 {
            if points[i].row == 0 {
                ret = false;
                break;
            }
            col1 = points[i].col;
            row1 = points[i].row - 1;
            if can.matrix[row1][col1] {
                ret = false;
                break
            }
        }
    }
}

impl Figure for GenericFigure {
    fn land(&self, rv: &mut [Point; 4]) {
        match self {
            GenericFigure::Bar(f) => f.land(rv),
            _ => panic!("trying to land a None figure"),
        }
    }
}

fn new_figure(fig: &mut GenericFigure, cols: u16, rows: u16) {
    let bar = BarFigure::new(cols, rows);
    *fig = GenericFigure::Bar(bar);
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
    fn update(&mut self, can: &Can, curfig: &dyn Figure);
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
    fn update(&mut self, can: &Can, curfig: &dyn Figure) {
        // XXX Later
        let mut points: [Point; 4] = [Point{col:0,row:0}; 4];
        curfig.land(&mut points);

        let mut field = vec![vec![false; COLS]; ROWS];
        for i in 0..ROWS {
            for j in 0..COLS {
                field[i][j] = can.matrix[i][j];
            }
        }

        for i in 0..4 {
            let p = points[i];
            field[p.row as usize][p.col as usize] = true;
        }

        for i in 0..ROWS {
            let s = format!("{}", termion::cursor::Goto(
                    (TCOFF+1) as u16, (TROFF+i+1) as u16));
            self.dp.write(s.as_bytes()).unwrap();
            // XXX optimize
            for j in 0..COLS {
                if field[ROWS-1-i][j] {
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

    let mut curfig = GenericFigure::None;
    new_figure(&mut curfig, COLS as u16, ROWS as u16);
    dp.update(&can, &curfig);

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
            Key::Down => {
                if curfig.down1(&can) {
                    dp.update(&can, &curfig);
                }
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
