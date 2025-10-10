use colored::Colorize;
use std::{any::Any, fmt::Error, io};

const BLUE: u8 = 1;
const RED: u8 = 2;
const GREEN: u8 = 3;
const YELLOW: u8 = 4;

const BODY_PATTERN: u8 = 3;
const TAIL: u8 = 1;
const HEAD: u8 = 2;

#[derive(Debug)]
struct TileSide {
    equal_part: u8,
    bit_pattern: u8,
    bit_value: u8
}

impl TileSide  {
    fn compare(&self, side: &TileSide) -> bool {
        self.equal_part == side.equal_part && self.bit_pattern == side.bit_pattern && (self.bit_value | side.bit_value == self.bit_pattern)
    }
}

#[derive(Debug)]
struct SquareTile {
    direction: u8,
    sides: [TileSide; 4]
}

impl SquareTile {
    fn rotate(mut self) {
        self.direction += 1;
        if self.direction > 3 
        {
            self.direction = 0;
        }
    }

    fn left_side(&self) -> &TileSide {
        &self.sides[self.direction as usize]
    }

    fn up_side(&self) -> &TileSide {
        &self.sides[(self.direction + 1) as usize]
    }

    fn right_side(&self) -> &TileSide {
        &self.sides[(self.direction + 2) as usize]
    }

    fn down_side(&self) -> &TileSide {
        &self.sides[(self.direction + 3) as usize]
    }
}

#[derive(Debug)]
struct SquarePuzzle {
    tiles: Vec<SquareTile>
}

impl SquarePuzzle {
    fn print_is_solved(&self, message: String){
        let is_solved_result = self.is_solved();

        let is_solved = match is_solved_result {
            Ok(is_solved) => is_solved,
            Err(error) => panic!("Problem solving a puzzle: {error:?}"),
        };
        
        println!("{}:{}", message, if is_solved { "True".green() } else { "False".red() });
    }

    fn is_solved(&self) -> Result<bool, String> {
        if self.tiles.len() != 9 {
            return Err("Square puzzle with 9 tiles is supported only!".to_string());
        }

        let center_tile:&SquareTile = &self.tiles[4];
        let left_tile:&SquareTile = &self.tiles[3];
        let up_tile:&SquareTile = &self.tiles[1];
        let right_tile:&SquareTile = &self.tiles[5];
        let down_tile:&SquareTile = &self.tiles[7];

        Ok(center_tile.left_side().compare(left_tile.right_side()) 
            && center_tile.up_side().compare(up_tile.down_side()) 
            && center_tile.right_side().compare(right_tile.left_side())
            && center_tile.down_side().compare(down_tile.up_side()))
    }
}

trait ConsoleDisplayablePuzzle {
    fn show_in_console(&self);
}


impl ConsoleDisplayablePuzzle for SquarePuzzle {
    fn show_in_console(&self){
        println!("I am a Square Puzzle. I have {} tiles.", self.tiles.len());
    }
}

fn main() {
    println!("{}", "Starting a puzzle game... Enjoy!".blue());

    println!("{}", "Creating a puzzle...");

    let puzzle: SquarePuzzle = create_puzzle();
    puzzle.show_in_console();
    puzzle.print_is_solved("Base puzzle solve result".to_string());

    let all_red_puzzle: SquarePuzzle = create_puzzle_all_red();
    all_red_puzzle.print_is_solved("All red puzzle solve result".to_string());

    println!("{}", "Game is over. Press ENTER to close.".blue());
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Read input error."
    );
}

fn create_puzzle_all_red() -> SquarePuzzle {
    let tiles: Vec<SquareTile> = vec![
        // Tile 00
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  } ] }, 
        // Tile 01
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  } ] }, 
        // Tile 02
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  } ] },
            
        // Tile 10
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  } ] },
        // Tile 11
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  } ] },
        // Tile 12
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  } ] },

        // Tile 20
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  } ] },
        // Tile 21
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  } ] },
        // Tile 22
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  } ] }
        ];

    SquarePuzzle { tiles: tiles }
}

fn create_puzzle() -> SquarePuzzle {
    let tiles: Vec<SquareTile> = vec![
        // Tile 00
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: BLUE, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: BLUE, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: GREEN, bit_pattern: BODY_PATTERN, bit_value: TAIL  } ] }, 
        // Tile 01
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: GREEN, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: BLUE, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: BLUE, bit_pattern: BODY_PATTERN, bit_value: HEAD  } ] }, 
        // Tile 02
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: YELLOW, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: GREEN, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: BLUE, bit_pattern: BODY_PATTERN, bit_value: TAIL  } ] },
            
        // Tile 10
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: YELLOW, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: GREEN, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: BLUE, bit_pattern: BODY_PATTERN, bit_value: TAIL  } ] },
        // Tile 11
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: GREEN, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: YELLOW, bit_pattern: BODY_PATTERN, bit_value: TAIL   }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  } ] },
        // Tile 12
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: GREEN, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: BLUE, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: YELLOW, bit_pattern: BODY_PATTERN, bit_value: HEAD   }, 
            TileSide { equal_part: BLUE, bit_pattern: BODY_PATTERN, bit_value: TAIL  } ] },

        // Tile 20
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: YELLOW, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: BLUE, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: GREEN, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  } ] },
        // Tile 21
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: GREEN, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: BLUE, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: YELLOW, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: TAIL  } ] },
        // Tile 22
        SquareTile { direction: 0, sides: [
            TileSide { equal_part: GREEN, bit_pattern: BODY_PATTERN, bit_value: TAIL  }, 
            TileSide { equal_part: RED, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: BLUE, bit_pattern: BODY_PATTERN, bit_value: HEAD  }, 
            TileSide { equal_part: YELLOW, bit_pattern: BODY_PATTERN, bit_value: TAIL  } ] }
        ];

    SquarePuzzle { tiles: tiles }
}
