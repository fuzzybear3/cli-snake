// use std::vec;
use std::{io::Stdout, thread, time};

use std::io::{stdout, Read, Write};
use termion::async_stdin;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{event::Key, raw::RawTerminal};

struct GridPiece {
    symble: char,
}

impl GridPiece {
    fn set_symble(&mut self, ch: char) {
        self.symble = ch;
    }
}

struct Dimensions {
    width: usize,
    height: usize,
}

struct Location {
    x: u32,
    y: u32,
}

struct World {
    grid: Grid,
    food_loc: Location,
}

type Grid = Vec<Vec<GridPiece>>;
const BOARDER_CHAR: char = '█';

fn main() {
    // terminal setup
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    write!(
        stdout,
        "{}{}q to exit. Type stuff, use alt, and so on.{}",
        // "{}{}q to exit. Type stuff, use alt, and so on.",
        // "{}q to exit. Type stuff, use alt, and so on.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    //* migrate to ternmion?
    if let Some((w, h)) = term_size::dimensions() {
        write!(
            stdout,
            "{}Width: {} -- Height: {}",
            termion::cursor::Goto(1, 2),
            w,
            h
        )
        .unwrap();
        stdout.flush().unwrap();
        // println!();
    } else {
        println!("Unable to get term size :(")
    }

    // let map = Vec::new();
    let size: (usize, usize) = term_size::dimensions().unwrap();
    let dimensions = Dimensions {
        width: size.0,
        //* hack to deal with extra prinln when printing
        height: size.1 - 1,
    };

    // time setup
    let time_step = time::Duration::from_millis(100);
    let now = time::Instant::now();

    let grid = init_grid(&dimensions);
    let world = World {
        food_loc: Location { x: 3, y: 3 },
        grid,
    };

    // Game loop
    loop {
        write!(stdout, "{}", termion::clear::All);
        let b = stdin.next();

        if let Some(Ok(b'q')) = b {
            break;
        }

        if let Some(Ok(b'w')) = b {}
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();

        let mut frame = init_grid(&dimensions);

        frame = draw_border(frame);
        // print_frame(frame, &dimensions);

        // write!(stdout, "{}", termion::cursor::Goto(5, 5)).unwrap();
        // stdout.flush().unwrap();
        print_frame_termion(frame, &dimensions, &mut stdout);
        // _print_frame(frame, &dimensions);
        stdout.flush().unwrap();
        thread::sleep(time_step);
    }
}

fn _print_frame(frame: Grid, dimensions: &Dimensions) {
    for row in frame.iter() {
        for column in row.iter() {
            print!("{}", column.symble);
        }
        //* extra newline
        println!();
    }
}

fn print_frame_termion(frame: Grid, dimensions: &Dimensions, stdout: &mut RawTerminal<Stdout>) {
    write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();
    for row in frame.iter() {
        for column in row.iter() {
            // write!(stdout, "{}", BOARDER_CHAR).unwrap();
            // stdout.flush().unwrap();

            // stdout.write(format!('{}", column.symble)).unwrap();
            //
            print!("{}", column.symble);

            // stdout.write_all(b"#").unwrap();

            // print!("{}", column.symble);
        }
        //* extra newline
        // println!();
        stdout.write_all(b"\n\r").unwrap();
    }
    stdout.flush().unwrap();
}

fn draw_border(mut frame: Grid) -> Grid {
    // top
    for piece in frame.first_mut().unwrap() {
        piece.set_symble(BOARDER_CHAR);
    }
    // sides
    for row in frame.iter_mut() {
        row.first_mut().unwrap().set_symble(BOARDER_CHAR);
        row.last_mut().unwrap().set_symble(BOARDER_CHAR);
    }

    // bottom
    for piece in frame.last_mut().unwrap() {
        piece.set_symble(BOARDER_CHAR);
    }
    frame
}

fn init_grid(dimensions: &Dimensions) -> Grid {
    let mut grid: Vec<Vec<GridPiece>> = Vec::new();

    for _ in 0..dimensions.height {
        let mut row = Vec::new();
        for _ in 0..dimensions.width {
            row.push(GridPiece { symble: ' ' })
        }
        grid.push(row);
    }

    grid
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_make_grid() {
        let grid = init_grid(&Dimensions {
            width: 5,
            height: 14,
        });

        assert_eq!(grid.len(), 14);
        assert_eq!(grid.get(0).unwrap().len(), 5);
    }
}
