// use std::vec;
use std::{thread, time};
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
    let time_step = time::Duration::from_millis(100);
    let now = time::Instant::now();
    if let Some((w, h)) = term_size::dimensions() {
        println!("Width: {}\nHeight: {}", w, h);
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

    let grid = init_grid(&dimensions);
    let world = World {
        food_loc: Location { x: 3, y: 3 },
        grid,
    };

    // Game loop
    loop {
        thread::sleep(time_step);

        let mut frame = init_grid(&dimensions);

        frame = draw_border(frame);
        print_frame(frame, &dimensions);
    }
}

fn print_frame(frame: Grid, dimensions: &Dimensions) {
    for row in frame.iter() {
        for column in row.iter() {
            print!("{}", column.symble);
        }
        //* extra newline
        println!();
    }
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
