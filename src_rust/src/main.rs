use colored::Colorize;
use std::{any::Any, io};

struct TileSide {
    equal_part: u8,
    bit_pattern: u8,
    bit_value: u8
}

impl TileSide  {
    fn compare(&self, side: TileSide) -> bool {
        self.equal_part == side.equal_part && self.bit_pattern == side.bit_pattern && (self.bit_value | side.bit_value == self.bit_pattern)
    }
}

struct SquareTile {
    orientation: u8,
    sides: [TileSide; 4]
}

struct SquarePuzzle {
    tiles: Vec<SquareTile>
}

trait ConsoleDisplayablePuzzle {
    fn show_in_console(&self);
}

trait RotatableTile {
    fn rotate(self);
}

impl RotatableTile for SquareTile {
    fn rotate(mut self) {
        self.orientation += 1;
    }
}

impl ConsoleDisplayablePuzzle for SquarePuzzle {
    fn show_in_console(&self){
        println!("I am a Square Puzzle. I have {} tiles.", self.tiles.len());
    }
}

fn main() {
    println!("{}", "Starting a puzzle game... Enjoy!".green());

    println!("{}", "Creating a puzzle...");

    let puzzle: SquarePuzzle = create_puzzle();
    puzzle.show_in_console();
    

    println!("{}", "Game is over. Press ENTER to close.".green());
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Read input error."
    );
}

fn create_puzzle() -> SquarePuzzle {
    const BLUE: u8 = 1;
    const RED: u8 = 2;
    const GREEN: u8 = 3;

    const BODY_PATTERN: u8 = 3;
    const TAIL: u8 = 1;
    const HEAD: u8 = 2;

    let tiles: Vec<SquareTile> = vec![
        // Tile 00
        SquareTile { orientation: 0, sides: [
            TileSide { equal_part: BLUE, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: BLUE, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: BLUE, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: BLUE, bit_pattern: BODY_PATTERN, bit_value: TAIL  } ] }, 
        // Tile 01
        SquareTile { orientation: 0, sides: [0, 1, 2, 3] }, 
        // Tile 02
        SquareTile { orientation: 0, sides: [0, 1, 2, 3] }];
    SquarePuzzle { tiles: tiles }
}