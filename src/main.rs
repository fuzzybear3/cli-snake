// use std::vec;
use std::{io::Stdout, thread, time};

use std::io::{stdin, stdout, Write};
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

    write!(
        stdout,
        // "{}{}q to exit. Type stuff, use alt, and so on.{}",
        "{}{}q to exit. Type stuff, use alt, and so on.",
        // "{}q to exit. Type stuff, use alt, and so on.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        // termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    //* migrate to ternmion?
    print!(" \ntest:");
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

    print!(" \ntest:");
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
        let stdin = stdin();
        for k in stdin.keys() {
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(1, 1),
                termion::clear::CurrentLine
            )
            .unwrap();
            match k.as_ref().unwrap() {
                Key::Char('q') => {
                    write!(stdout, "{}", termion::cursor::Show).unwrap();
                    return;
                }
                Key::Char(c) => println!("{}", c),
                Key::Alt(c) => println!("^{}", c),
                Key::Ctrl(c) => println!("*{}", c),
                Key::Esc => println!("ESC"),
                Key::Left => println!("←"),
                Key::Right => println!("→"),
                Key::Up => println!("↑"),
                Key::Down => println!("↓"),
                Key::Backspace => println!("×"),
                _ => {
                    println!("{:?}", k)
                }
            }
            stdout.flush().unwrap();
        }

        return ();
        let mut frame = init_grid(&dimensions);

        frame = draw_border(frame);
        // print_frame(frame, &dimensions);

        write!(stdout, "{}", termion::cursor::Goto(5, 5)).unwrap();
        stdout.flush().unwrap();
        print!("steven");
        print!("steven");
        print!("steven");
        print!("steven");
        print!("steven");
        print!("steven");
        // print_frame_termion(frame, &dimensions, &mut stdout);
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
    println!("stevn");
    println!("stevn");
    println!("stevn");
    println!("stevn");
    println!("stevn");
    write!(stdout, "{}{}", termion::cursor::Goto(1, 3), "steven").unwrap();
    stdout.flush().unwrap();
    // for row in frame.iter() {
    //     for column in row.iter() {
    //         write!(stdout, "{}", BOARDER_CHAR).unwrap();
    //         stdout.flush().unwrap();
    //
    //         print!("{}", column.symble);
    //     }
    //     //* extra newline
    //     println!();
    // }
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
