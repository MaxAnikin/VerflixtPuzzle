use colored::Colorize;
use std::{any::Any, collections::{btree_map::VacantEntry, HashMap}, fmt::Error, io, panic, time::Instant};

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
struct TilePosition {
    id: String,
    direction: u8
}

#[derive(Debug)]
struct PuzzleSolution {
    tile_positions: Vec<TilePosition>
}

#[derive(Debug)]
struct SquarePuzzle {
    tiles: Vec<Tile>
}

impl SquarePuzzle {
    fn print_is_solved(&self, message: String) {
        let is_solved: bool = self.is_solved();

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

    // fn is_solved(&self) -> bool {
    //     if self.tiles.len() != 9 {
    //         panic!("Square puzzle with 9 tiles is supported only!");
    //     }

    //     let center_tile: &Tile = &self.tiles[4];
    //     let left_tile: &Tile = &self.tiles[3];
    //     let up_tile: &Tile = &self.tiles[1];
    //     let right_tile: &Tile = &self.tiles[5];
    //     let down_tile: &Tile = &self.tiles[7];

    //     center_tile.left_side().compare(left_tile.right_side())
    //         && center_tile.up_side().compare(up_tile.down_side())
    //         && center_tile.right_side().compare(right_tile.left_side())
    //         && center_tile.down_side().compare(down_tile.up_side())
    // }

    // fn is_cross_solvable(&self) -> bool {
    //     if self.tiles.len() != 9 {
    //         panic!("Square puzzle with 9 tiles is supported only!");
    //     }

    //     let center_tile: &Tile = &self.tiles[4];
    //     let left_tile: &Tile = &self.tiles[3];
    //     let up_tile: &Tile = &self.tiles[1];
    //     let right_tile: &Tile = &self.tiles[5];
    //     let down_tile: &Tile = &self.tiles[7];

    //     center_tile.left_side().compare(left_tile.right_side())
    //         && center_tile.up_side().compare(up_tile.down_side())
    //         && center_tile.right_side().compare(right_tile.left_side())
    //         && center_tile.down_side().compare(down_tile.up_side())
    // }

    // fn find_and_rotate(&self, reference_side: &TileSide) -> Option<&Tile> {
    //     for tile in &self.tiles {
    //         for side in &tile.sides {
    //             if reference_side.compare(&side) {
    //                 return Some(tile);
    //             }
    //         }
    //     }

    //     None
    // }

    // fn solve_with_selection(&self) -> Result<Vec<u8>, String> {
    //     let result: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

    //     let mut elements: Vec<u8> = (0..8).collect();

    //     for tile in &self.tiles {}

    //     Ok(result)
    // }

    // fn solve_with_permutations(&mut self) -> Result<Vec<Tile>, String> {
    //     let result: Vec<Tile> = vec![];
    //     let mut cross_results: HashMap<String, bool> = HashMap::new();

    //     let _generate_permutations = &self.permutate(cross_results, 0);

    //     Ok(result)
    // }

    // fn permutate(&mut self, mut map: HashMap<String, bool>, start: usize) {
    //     if start == self.tiles.len() {
    //         let cross_key: String = self.get_cross_key();
    //         let val = match map.entry(cross_key) {
    //             Vacant (entry) => 
    //             {
    //                 let is_cross_solvable: bool = self.is_cross_solvable();
    //                 map.insert(cross_key, is_cross_solved)
    //             }
    //             Occupied (entry) => 
    //             {
    //                 entry
    //             }


    //         };
            
    //         return;
    //     }

    //     for i in start..self.tiles.len() {
    //         self.tiles.swap(start, i);
            
    //         let _ = &self.permutate(map, start + 1);

    //         self.tiles.swap(start, i);
    //     }
    // }
    
    fn get_cross_key(&self) -> String {
        if self.tiles.len() != 9 {
            panic!("Cross string key requires 9 tiles.");
        }

        format!("{}{}{}{}{}", self.tiles[1].id, self.tiles[3].id, self.tiles[4].id, self.tiles[5].id, self.tiles[7].id)
    }
}


#[derive(Debug)]
struct SquarePuzzlePermutateByCrossResolver {
    puzzle: SquarePuzzle
}

impl SquarePuzzlePermutateByCrossResolver {
    fn new(puzzle: SquarePuzzle) -> Result<Self, String> {
        if puzzle.tiles.len() != 9 {
            return Err("Only square puzzles with nine tiles are supported.".to_string());
        }

        Ok(SquarePuzzlePermutateByCrossResolver { puzzle: puzzle })
    }

    fn get_solutions(&mut self) -> Result<Vec<PuzzleSolution>, String> {
        let mut solutions: Vec<PuzzleSolution> = vec![];
        let mut cross_map: HashMap<String, bool> = HashMap::new();
        
        match self.permutate(&mut solutions, &mut cross_map, 0) {
            Err(error) => Err(error),
            Ok(()) => Ok(solutions)
        }        
    }

    fn permutate(&mut self, solutions: &mut Vec<PuzzleSolution>, map: &mut HashMap<String, bool>, start: usize) -> Result<(), String> {
        if start == self.puzzle.tiles.len() {
            let cross_key: String = match self.get_cross_key() {
                Err(error) => {
                    return Err(error);
                }
                Ok(key) => key
            };

            let cross_solved:bool = match map.get(&cross_key) {
                Some(entry) => *entry,
                None => {
                    map.insert(cross_key, true);
                    true
                }
            };
        }

        for i in start..self.puzzle.tiles.len() {
            self.puzzle.tiles.swap(start, i);
            
            let _ = self.permutate(solutions, map, start + 1);

            self.puzzle.tiles.swap(start, i);
        }

        Ok(())
    }

    fn get_cross_key(&self) -> Result<String, String> {
        if self.puzzle.tiles.len() != 9 {
            return Err("Cross string key requires 9 tiles.".to_string());
        }

        Ok(format!("{}{}{}{}{}", self.puzzle.tiles[1].id, self.puzzle.tiles[3].id, self.puzzle.tiles[4].id, self.puzzle.tiles[5].id, self.puzzle.tiles[7].id))
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

    let start: Instant = Instant::now();
    let _s1: Result<Vec<Tile>, String> = puzzle.solve_with_permutations();
    let duration: std::time::Duration = start.elapsed();
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
