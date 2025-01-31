// use std::vec;
use std::{
    io::{Bytes, Stdout},
    process::exit,
    thread, time,
};

use rand::Rng;
use std::io::{stdout, Read, Write};
// use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{async_stdin, AsyncReader};
// use termion::event::Key;
use termion::raw::RawTerminal;

use std::collections::LinkedList;

#[derive(PartialEq)]
enum Move {
    Left,
    Right,
    Up,
    Down,
    Exit,
}

struct GridPiece {
    symbol: char,
}

impl GridPiece {
    fn set_symble(&mut self, ch: char) {
        self.symbol = ch;
    }
}

struct Dimensions {
    width: usize,
    height: usize,
}

#[derive(Clone, PartialEq, Debug)]
struct Location {
    x: usize,
    y: usize,
}

struct Snake {
    head_location: Location,
    direction: Move,
    body: LinkedList<Location>,
    food_in_belly: usize,
}

struct World {
    _grid: Grid,
    food_location: Location,
    snake: Snake,
}

// row then column
type Grid = Vec<Vec<GridPiece>>;
const BOARDER_CHAR: char = '█';
const FOOD_CHAR: char = '*';
const SNAKE_BODY_CHAR: char = 'X';
const GROW_AMOUNT: usize = 50;
const BOARDER_WIDTH: usize = 1;

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
    let _now = time::Instant::now();

    let grid = init_grid(&dimensions);
    let mut body = LinkedList::new();
    for i in 1..6 {
        body.push_back(Location { x: 6, y: 7 + i });
    }
    let snake_head = Snake {
        head_location: Location { x: 7, y: 7 },
        direction: Move::Down,
        body,
        food_in_belly: 0,
    };
    let mut world = World {
        food_location: Location { x: 3, y: 3 },
        _grid: grid,
        snake: snake_head,
    };

    world.food_location = gen_random_location(&dimensions);

    // Game loop
    loop {
        let action = read_move(&mut stdin);

        if let Some(Move::Exit) = action {
            write!(stdout, "{}{}", termion::clear::All, termion::cursor::Show).unwrap();
            return;
        }

        if let Some(action) = action {
            world = control_snake(action, world);
        }

        world = advance_snake(world, &dimensions);
        check_collision(&world, &dimensions);

        write!(stdout, "{}", termion::cursor::Goto(1, 1),).unwrap();

        let mut frame = init_grid(&dimensions);

        frame = draw_border(frame);
        frame = draw_snake(frame, &world);
        frame = draw_food(frame, &world);

        print_frame_termion(frame, &dimensions, &mut stdout);
        // println!("{:?}", world.snake.head_location);
        stdout.flush().unwrap();

        thread::sleep(time_step);
    }
}

// fn feed_snake(mut world: World) -> world {}
fn check_collision(world: &World, dimensions: &Dimensions) {
    let head = &world.snake.head_location;

    if head.x < 1 || head.x > dimensions.width - 2 * BOARDER_WIDTH {
        exit(0);
    }

    if head.y < 1 || head.y > dimensions.height - 2 * BOARDER_WIDTH {
        exit(0);
    }

    for node in world.snake.body.iter() {
        if head == node {
            exit(0);
        }
    }
}

fn draw_food(mut frame: Grid, world: &World) -> Grid {
    frame[world.food_location.y][world.food_location.x].set_symble(FOOD_CHAR);
    frame
}

fn gen_random_location(dimensions: &Dimensions) -> Location {
    //* don't spawn on snake
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(1..dimensions.width);
    let y = rng.gen_range(1..dimensions.height - BOARDER_WIDTH);
    Location { x, y }
}

fn draw_snake(mut frame: Grid, world: &World) -> Grid {
    let head_loc = &world.snake.head_location;

    // draw head
    frame[head_loc.y][head_loc.x].set_symble(SNAKE_BODY_CHAR);

    // draw body
    for node in world.snake.body.iter() {
        frame[node.y][node.x].set_symble(SNAKE_BODY_CHAR);
    }

    frame
}

fn advance_snake(mut world: World, dimensions: &Dimensions) -> World {
    let move_amount = 1;

    // Check for food
    if world.food_location == world.snake.head_location {
        world.food_location = gen_random_location(dimensions);
        world.snake.food_in_belly += GROW_AMOUNT;
    }

    if world.snake.food_in_belly == 0 {
        // move snake tail
        world.snake.body.pop_back().unwrap();
    } else {
        // grow tail
        world.snake.food_in_belly -= 1;
    }

    world
        .snake
        .body
        .push_front(world.snake.head_location.clone());

    // move head
    match world.snake.direction {
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

fn control_snake(action: Move, mut world: World) -> World {
    match action {
        Move::Left => {
            if world.snake.direction != Move::Right {
                world.snake.direction = Move::Left;
            }
        }
        Move::Right => {
            if world.snake.direction != Move::Left {
                world.snake.direction = Move::Right;
            }
        }
        Move::Up => {
            if world.snake.direction != Move::Down {
                world.snake.direction = Move::Up;
            }
        }
        Move::Down => {
            if world.snake.direction != Move::Up {
                world.snake.direction = Move::Down;
            }
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
            print!("{}", column.symbol);
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
            print!("{}", column.symbol);
        }
        //* extra newline
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
            row.push(GridPiece { symbol: ' ' })
        }
        grid.push(row);
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_grid() {
        let grid = init_grid(&Dimensions {
            width: 5,
            height: 14,
        });

        assert_eq!(grid.len(), 14);
        assert_eq!(grid.first().unwrap().len(), 5);
    }
}
