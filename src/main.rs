// use std::io;
// use std::io::Read;  // the trait Read provides read_exact
use std::io::{stdin, stdout, Write};
use std::process::ExitCode;
use termion::event::Key;
use termion::input::TermRead; // provides keys()
use termion::raw::IntoRawMode;

const ROWS: u16 = 20;
const COLS: u16 = 10;
const TROFF: u16 = 1;
const TCOFF: u16 = 5;

#[derive(Copy)]
pub struct Point {
    col: u16,
    row: u16,
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point{col: self.col, row: self.row}
    }
}

pub struct Can {
    cols: u16,
    rows: u16,
    matrix: Vec<Vec<bool>>,
}

pub trait Figure {
    fn snap(&self, rv: &mut [Point; 4]);
    fn down1(&mut self, can: &Can) -> bool;
    fn left1(&mut self, can: &Can) -> bool;
    fn right1(&mut self, can: &Can) -> bool;
}

impl Can {
    // Imprint the fig into the can
    // This is usually done before disposing of the figure.
    fn imprint(&mut self, fig: &dyn Figure) {
        let mut points: [Point; 4] = [Point{col:0,row:0}; 4];
        fig.snap(&mut points);
        for i in 0..points.len() {
            let col = usize::from(points[i].col);
            let row = usize::from(points[i].row);
            self.matrix[row][col] = true;
        }
    }
}

struct BarFigure {
    pos: Point,
    points: [Point; 3],
}

struct TeeFigure {
    pos: Point,
    points: [Point; 3],
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

impl TeeFigure {
    fn new(cols: u16, rows: u16) -> TeeFigure {
        TeeFigure {
            pos: Point{col: cols/2, row: rows-1},
            points: [
                Point{col: cols/2 - 1, row: rows-1},
                Point{col: cols/2 + 1, row: rows-1},
                Point{col: cols/2, row: rows-2},
            ]
        }
    }
}

impl Figure for BarFigure {
    fn snap(&self, rv: &mut [Point; 4]) {
        rv[0] = self.pos;
        rv[1] = self.points[0];
        rv[2] = self.points[1];
        rv[3] = self.points[2];
    }

    fn down1(&mut self, can: &Can) -> bool {

        // Step 1: check
        // XXX This is generic, good for facoring

        let mut points: [Point; 4] = [Point{col:0,row:0}; 4];

        // Less Rust-like, but let's try it anyway
        let mut ret: bool = true;
        // More Rust-like
        // let mut ret = true;

        self.snap(&mut points);
        for i in 0..points.len() {
            if points[i].row == 0 {
                ret = false;
                break;
            }
            // i as usize
            // usize::from(i)
            // trait TryFrom
            // (i).into()
            let col1 = usize::from(points[i].col);
            let row1 = usize::from(points[i].row - 1);
            if can.matrix[row1][col1] {
                ret = false;
                break
            }
        }

        // Step 2: move

        if ret {
            self.pos.row -= 1;
            for i in 0..self.points.len() {
                self.points[i].row -= 1;
            }
        }

        ret
    }

    fn left1(&mut self, can: &Can) -> bool {

        let mut points: [Point; 4] = [Point{col:0,row:0}; 4];
        self.snap(&mut points);
        for i in 0..points.len() {
            if points[i].col == 0 {
                return false;
            }
            let col1 = usize::from(points[i].col - 1);
            let row1 = usize::from(points[i].row);
            if can.matrix[row1][col1] {
                return false;
            }
        }

        self.pos.col -= 1;
        for i in 0..self.points.len() {
            self.points[i].col -= 1;
        }

        true
    }

    fn right1(&mut self, can: &Can) -> bool {

        let mut points: [Point; 4] = [Point{col:0,row:0}; 4];
        self.snap(&mut points);
        for i in 0..points.len() {
            if points[i].col + 1 >= can.cols {
                return false;
            }
            let col1 = usize::from(points[i].col + 1);
            let row1 = usize::from(points[i].row);
            if can.matrix[row1][col1] {
                return false;
            }
        }

        self.pos.col += 1;
        for i in 0..self.points.len() {
            self.points[i].col += 1;
        }

        true
    }
}

impl Figure for TeeFigure {
    fn snap(&self, rv: &mut [Point; 4]) {
        rv[0] = self.pos;
        rv[1] = self.points[0];
        rv[2] = self.points[1];
        rv[3] = self.points[2];
    }

    fn down1(&mut self, can: &Can) -> bool {

        // Step 1: check
        // XXX This is generic, good for facoring

        let mut points: [Point; 4] = [Point{col:0,row:0}; 4];

        // Less Rust-like, but let's try it anyway
        let mut ret: bool = true;
        // More Rust-like
        // let mut ret = true;

        self.snap(&mut points);
        for i in 0..points.len() {
            if points[i].row == 0 {
                ret = false;
                break;
            }
            // i as usize
            // usize::from(i)
            // trait TryFrom
            // (i).into()
            let col1 = usize::from(points[i].col);
            let row1 = usize::from(points[i].row - 1);
            if can.matrix[row1][col1] {
                ret = false;
                break
            }
        }

        // Step 2: move

        if ret {
            self.pos.row -= 1;
            for i in 0..self.points.len() {
                self.points[i].row -= 1;
            }
        }

        ret
    }

    fn left1(&mut self, can: &Can) -> bool {

        let mut points: [Point; 4] = [Point{col:0,row:0}; 4];
        self.snap(&mut points);
        for i in 0..points.len() {
            if points[i].col == 0 {
                return false;
            }
            let col1 = usize::from(points[i].col - 1);
            let row1 = usize::from(points[i].row);
            if can.matrix[row1][col1] {
                return false;
            }
        }

        self.pos.col -= 1;
        for i in 0..self.points.len() {
            self.points[i].col -= 1;
        }

        true
    }

    fn right1(&mut self, can: &Can) -> bool {

        let mut points: [Point; 4] = [Point{col:0,row:0}; 4];
        self.snap(&mut points);
        for i in 0..points.len() {
            if points[i].col + 1 >= can.cols {
                return false;
            }
            let col1 = usize::from(points[i].col + 1);
            let row1 = usize::from(points[i].row);
            if can.matrix[row1][col1] {
                return false;
            }
        }

        self.pos.col += 1;
        for i in 0..self.points.len() {
            self.points[i].col += 1;
        }

        true
    }
}

fn new_figure(cols: u16, rows: u16) -> Box<dyn Figure> {
    let bar = BarFigure::new(cols, rows);
    let fig = Box::new(bar);
    fig
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
            let s = format!("{}", termion::cursor::Goto(TCOFF+1, TROFF+i+1));
            self.dp.write(s.as_bytes()).unwrap();
            for j in 0..COLS {
                let s = " . ";
                self.dp.write(s.as_bytes()).unwrap();
                self.matrix[(ROWS-1-i) as usize][j as usize] = false;
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
        // Seems weirdly inefficient to create a whole new field,
        // just to merge the (potentially conflicting) figure on
        // top of the can, and then update the display with the
        // merged field. But what else is out there?

        let mut points: [Point; 4] = [Point{col:0,row:0}; 4];
        curfig.snap(&mut points);

        let mut field = vec![vec![false; can.cols as usize]; can.rows as usize];
        for i in 0..(can.rows as usize) {
            for j in 0..(can.cols as usize) {
                field[i][j] = can.matrix[i][j];
            }
        }

        for i in 0..4 {
            let p = points[i];
            field[p.row as usize][p.col as usize] = true;
        }

        for i in 0..can.rows {
            let row: usize = (can.rows-1-i) as usize;

            let mut changed = false;
            for j in 0..can.cols {
                if field[row][j as usize] != self.matrix[row][j as usize] {
                    changed = true;
                    break;
                }
            }
            if changed {

                let s = format!("{}", termion::cursor::Goto(TCOFF+1, TROFF+i+1));
                self.dp.write(s.as_bytes()).unwrap();
                for j in 0..can.cols {
                    if field[row][j as usize] {
                        let s = "[_]";
                        self.dp.write(s.as_bytes()).unwrap();
                    } else {
                        let s = " . ";
                        self.dp.write(s.as_bytes()).unwrap();
                    }
                    self.matrix[row][j as usize] = field[row][j as usize]
                }
            }
        }

        // It's not beautiful if we leave cursor linger along
        // the right side of the can.
        let s = format!("{}", termion::cursor::Goto(1, 1));
        self.dp.write(s.as_bytes()).unwrap();
    }
}

fn is_conflict(can: &Can, curfig: &dyn Figure) -> bool {

    let mut points: [Point; 4] = [Point{col:0,row:0}; 4];
    curfig.snap(&mut points);

    let mut conflict = false;
    for i in 0..4 {
        let p = points[i];
        if can.matrix[p.row as usize][p.col as usize] {
            conflict = true;
        }
    }

    conflict
}

// This basically exists in order to drop termion's RawTerminal restorer.
fn game() -> ExitCode {

    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    // let mut field: Vec<Vec<bool>> = Vec::new();
    let field = vec![vec![false; COLS as usize]; ROWS as usize];
    let mut can = Can {
        cols: COLS,
        rows: ROWS,
        matrix: field,
    };
    let mut dp = DisplayOne {
        matrix: vec![vec![false; COLS as usize]; ROWS as usize],
        dp: &mut stdout,
    };
    dp.erase();

    let mut curfig = new_figure(can.cols, can.rows);
    dp.update(&can, &*curfig);

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
                if curfig.left1(&can) {
                    dp.update(&can, &*curfig);
                }
            }
            Key::Right => {
                dp.message(">");
                if curfig.right1(&can) {
                    dp.update(&can, &*curfig);
                }
            }
            Key::Down => {
                if curfig.down1(&can) {
                    dp.update(&can, &*curfig);
                } else {
                    can.imprint(&*curfig);
                    curfig = new_figure(can.cols, can.rows);
                    dp.update(&can, &*curfig);
                    if is_conflict(&can, &*curfig) {
                        dp.message("Game Over");
                        break;
                    }
                }
            }
            _ => {}
        }
        dp.flush();
    }
    let s = format!("{}", termion::cursor::Goto(1, TROFF+ROWS+1));
    dp.dp.write(s.as_bytes()).unwrap();
    dp.flush();

    ExitCode::from(0)
}

fn main() -> ExitCode {

    let ec = game();

    println!("Goodbye");
    ec
}
