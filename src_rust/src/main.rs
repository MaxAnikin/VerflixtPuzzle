use colored::Colorize;
use std::{any::Any, collections::HashMap, fmt::Error, io, time::Instant};

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
    bit_value: u8,
}

impl TileSide {
    fn compare(&self, side: &TileSide) -> bool {
        self.equal_part == side.equal_part
            && self.bit_pattern == side.bit_pattern
            && (self.bit_value | side.bit_value == self.bit_pattern)
    }
}

#[derive(Debug)]
struct Tile {
    id: String,
    direction: u8,
    sides: Vec<TileSide>,
}

impl Tile {
    fn rotate(mut self) {
        self.direction += 1;
        if self.direction > 3 {
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
    tiles: Vec<Tile>,
}

impl SquarePuzzle {
    fn print_is_solved(&self, message: String) {
        let is_solved_result = self.is_solved();

        let is_solved = match is_solved_result {
            Ok(is_solved) => is_solved,
            Err(error) => panic!("Problem solving a puzzle: {error:?}"),
        };

        println!(
            "{}:{}",
            message,
            if is_solved {
                "True".green()
            } else {
                "False".red()
            }
        );
    }

    fn is_solved(&self) -> Result<bool, String> {
        if self.tiles.len() != 9 {
            return Err("Square puzzle with 9 tiles is supported only!".to_string());
        }

        let center_tile: &Tile = &self.tiles[4];
        let left_tile: &Tile = &self.tiles[3];
        let up_tile: &Tile = &self.tiles[1];
        let right_tile: &Tile = &self.tiles[5];
        let down_tile: &Tile = &self.tiles[7];

        Ok(center_tile.left_side().compare(left_tile.right_side())
            && center_tile.up_side().compare(up_tile.down_side())
            && center_tile.right_side().compare(right_tile.left_side())
            && center_tile.down_side().compare(down_tile.up_side()))
    }

    fn find_and_rotate(&self, reference_side: &TileSide) -> Option<&Tile> {
        for tile in &self.tiles {
            for side in &tile.sides {
                if reference_side.compare(&side) {
                    return Some(tile);
                }
            }
        }

        None
    }

    fn solve_with_selection(&self) -> Result<Vec<u8>, String> {
        let result: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

        let mut elements: Vec<u8> = (0..8).collect();

        for tile in &self.tiles {}

        Ok(result)
    }

    fn solve_with_permutations(&mut self) -> Result<Vec<Tile>, String> {
        let result: Vec<Tile> = vec![];
        let mut cross_results: HashMap<String, bool> = HashMap::new();

        &self.generate_permutations(0);

        Ok(result)
    }

    fn generate_permutations(&mut self, start: usize) {
        if start == self.tiles.len() {
            return;
        }

        for i in start..self.tiles.len() {
            self.tiles.swap(start, i);

            let _ = &self.generate_permutations(start + 1);

            self.tiles.swap(start, i);
        }
    }
}

trait ConsoleDisplayablePuzzle {
    fn show_in_console(&self);
}

impl ConsoleDisplayablePuzzle for SquarePuzzle {
    fn show_in_console(&self) {
        println!("I am a Square Puzzle. I have {} tiles.", self.tiles.len());
    }
}

fn main() {
    println!("{}", "Starting a puzzle game... Enjoy!".blue());

    println!("{}", "Creating a puzzle...");

    let mut puzzle: SquarePuzzle = create_puzzle();
    puzzle.show_in_console();
    puzzle.print_is_solved("Base puzzle solve result".to_string());

    let all_red_puzzle: SquarePuzzle = create_puzzle_all_red();
    all_red_puzzle.print_is_solved("All red puzzle solve result".to_string());

    let start = Instant::now();
    let _s1 = puzzle.solve_with_permutations();
    let duration = start.elapsed();
    println!("Permutations generated in: {:?}", duration);

    println!("{}", "Game is over. Press ENTER to close.".blue());
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Read input error.");
}

fn create_puzzle_all_red() -> SquarePuzzle {
    let tiles: Vec<Tile> = vec![
        // Tile 00
        Tile {
            id: "00".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
            ],
        },
        // Tile 01
        Tile {
            id: "01".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
            ],
        },
        // Tile 02
        Tile {
            id: "02".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
            ],
        },
        // Tile 10
        Tile {
            id: "10".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
            ],
        },
        // Tile 11
        Tile {
            id: "11".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
            ],
        },
        // Tile 12
        Tile {
            id: "12".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
            ],
        },
        // Tile 20
        Tile {
            id: "20".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
            ],
        },
        // Tile 21
        Tile {
            id: "21".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
            ],
        },
        // Tile 22
        Tile {
            id: "22".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
            ],
        },
    ];

    SquarePuzzle { tiles: tiles }
}

fn create_puzzle() -> SquarePuzzle {
    let tiles: Vec<Tile> = vec![
        // Tile 00
        Tile {
            id: "00".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: BLUE,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: BLUE,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: GREEN,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
            ],
        },
        // Tile 01
        Tile {
            id: "00".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: GREEN,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: BLUE,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: BLUE,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
            ],
        },
        // Tile 02
        Tile {
            id: "00".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: YELLOW,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: GREEN,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: BLUE,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
            ],
        },
        // Tile 10
        Tile {
            id: "00".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: YELLOW,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: GREEN,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: BLUE,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
            ],
        },
        // Tile 11
        Tile {
            id: "00".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: GREEN,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: YELLOW,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
            ],
        },
        // Tile 12
        Tile {
            id: "00".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: GREEN,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: BLUE,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: YELLOW,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: BLUE,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
            ],
        },
        // Tile 20
        Tile {
            id: "00".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: YELLOW,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: BLUE,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: GREEN,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
            ],
        },
        // Tile 21
        Tile {
            id: "00".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: GREEN,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: BLUE,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: YELLOW,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
            ],
        },
        // Tile 22
        Tile {
            id: "00".to_string(),
            direction: 0,
            sides: vec![
                TileSide {
                    equal_part: GREEN,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
                TileSide {
                    equal_part: RED,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: BLUE,
                    bit_pattern: BODY_PATTERN,
                    bit_value: HEAD,
                },
                TileSide {
                    equal_part: YELLOW,
                    bit_pattern: BODY_PATTERN,
                    bit_value: TAIL,
                },
            ],
        },
    ];

    SquarePuzzle { tiles: tiles }
}
