// use std::vec;
use std::{
    io::{Bytes, Stdout},
    thread, time,
};

use std::io::{stdout, Read, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{async_stdin, AsyncReader};
use termion::{event::Key, raw::RawTerminal};

use std::collections::LinkedList;

enum Move {
    Left,
    Right,
    Up,
    Down,
    Exit,
}

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
    x: usize,
    y: usize,
}

struct Snake {
    head_location: Location,
    direction: Move,
    body: LinkedList<Location>,
}

struct World {
    grid: Grid,
    food_loc: Location,
    snake: Snake,
}

// row then column
type Grid = Vec<Vec<GridPiece>>;
const BOARDER_CHAR: char = '█';

fn main() {
    // terminal setup
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    write!(
        stdout,
        // "{}{}q to exit. Type stuff, use alt, and so on.",
        "{}q to exit. Type stuff, use alt, and so on.{}",
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
    let snake_head = Snake {
        head_location: Location { x: 7, y: 7 },
        direction: Move::Up,
        body: LinkedList::new(),
    };
    let mut world = World {
        food_loc: Location { x: 3, y: 3 },
        grid,
        snake: snake_head,
    };

    // Game loop
    loop {
        let action = read_move(&mut stdin);

        if let Some(Move::Exit) = action {
            write!(stdout, "{}{}", termion::cursor::Show, termion::clear::All).unwrap();
            return;
        }

        if let Some(action) = action {
            world = control_snake(action, world);
        }

        let b = stdin.next();

        if let Some(Ok(b'w')) = b {}
        write!(stdout, "{}", termion::cursor::Goto(1, 1),).unwrap();

        let mut frame = init_grid(&dimensions);

        frame = draw_border(frame);
        frame = draw_snake(frame, &world);
        // print_frame(frame, &dimensions);

        // write!(stdout, "{}", termion::cursor::Goto(5, 5)).unwrap();
        // stdout.flush().unwrap();
        print_frame_termion(frame, &dimensions, &mut stdout);
        // _print_frame(frame, &dimensions);
        stdout.flush().unwrap();
        thread::sleep(time_step);
    }
}

fn draw_snake(mut frame: Grid, world: &World) -> Grid {
    let head_loc = &world.snake.head_location;

    frame[head_loc.y][head_loc.x].set_symble('X');

    frame
}

fn control_snake(action: Move, mut world: World) -> World {
    let move_amount = 1;
    match action {
        Move::Left => {
            world.snake.head_location.x -= move_amount;
        }
        Move::Right => {
            world.snake.head_location.x += move_amount;
        }
        Move::Up => {
            world.snake.head_location.y -= move_amount;
        }
        Move::Down => {
            world.snake.head_location.y += move_amount;
        }
        _ => (),
    }

    world
}

fn read_move(stdin: &mut Bytes<AsyncReader>) -> Option<Move> {
    let byte = stdin.next();

    if let Some(Ok(_)) = byte {
        // print!("{:?}", byte);

        match byte.unwrap().unwrap() {
            b'a' => Some(Move::Left),
            b's' => Some(Move::Right),
            b'w' => Some(Move::Up),
            b'r' => Some(Move::Down),
            b'q' => Some(Move::Exit),
            _ => None,
        }
    } else {
        None
    }
}

fn _print_frame(frame: Grid, _dimensions: &Dimensions) {
    for row in frame.iter() {
        for column in row.iter() {
            print!("{}", column.symble);
        }
        //* extra newline
        println!();
    }
}

fn print_frame_termion(frame: Grid, _dimensions: &Dimensions, stdout: &mut RawTerminal<Stdout>) {
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
