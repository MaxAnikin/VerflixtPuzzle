// puzzle_mod.rs
// Contains all puzzle logic except main, create_puzzle, and create_puzzle_all_red

use colored::Colorize;
use std::collections::HashMap;

pub const BLUE: u8 = 1;
pub const RED: u8 = 2;
pub const GREEN: u8 = 3;
pub const YELLOW: u8 = 4;
pub const BODY_PATTERN: u8 = 3;
pub const TAIL: u8 = 1;
pub const HEAD: u8 = 2;

#[derive(Debug)]
pub struct TileSide {
    pub equal_part: u8,
    pub bit_pattern: u8,
    pub bit_value: u8,
}

impl TileSide {
    pub fn compare(&self, side: &TileSide) -> bool {
        self.equal_part == side.equal_part
            && self.bit_pattern == side.bit_pattern
            && (self.bit_value | side.bit_value == self.bit_pattern)
    }
}

#[derive(Debug)]
pub struct Tile {
    pub id: String,
    pub direction: u8,
    pub sides: Vec<TileSide>,
}

impl Tile {
    pub fn rotate(&mut self) {
        self.direction += 1;
        if self.direction > 3 {
            self.direction = 0;
        }
    }
    pub fn left_side(&self) -> &TileSide {
        &self.sides[self.direction as usize]
    }
    pub fn up_side(&self) -> &TileSide {
        &self.sides[(self.direction + 1) as usize]
    }
    pub fn right_side(&self) -> &TileSide {
        &self.sides[(self.direction + 2) as usize]
    }
    pub fn down_side(&self) -> &TileSide {
        &self.sides[(self.direction + 3) as usize]
    }
}

#[derive(Debug)]
pub struct TilePosition {
    pub id: String,
    pub direction: u8
}

#[derive(Debug)]
pub struct PuzzleSolution {
    pub tile_positions: Vec<TilePosition>
}

#[derive(Debug)]
pub struct SquarePuzzle {
    pub tiles: Vec<Tile>,
    pub is_solved: bool
}

impl SquarePuzzle {
    pub fn print_is_solved(&self, message: String) {
        let is_solved: bool = self.is_solved;
        println!("{}:{}", message, if is_solved { "True".green() } else { "False".red() });
    }
    pub fn get_cross_key(&self) -> String {
        if self.tiles.len() != 9 {
            panic!("Cross string key requires 9 tiles.");
        }
        format!("{}{}{}{}{}", self.tiles[1].id, self.tiles[3].id, self.tiles[4].id, self.tiles[5].id, self.tiles[7].id)
    }
    pub fn solve_with_permutations(&self) -> Result<PuzzleSolution, String> {
        // Implementation placeholder
        Err("Not implemented".to_string())
    }
}

#[derive(Debug)]
pub struct SquarePuzzlePermutateByCrossResolver {
    pub puzzle: SquarePuzzle
}

impl SquarePuzzlePermutateByCrossResolver {
    pub fn new(puzzle: SquarePuzzle) -> Result<Self, String> {
        if puzzle.tiles.len() != 9 {
            return Err("Only square puzzles with nine tiles are supported.".to_string());
        }
        Ok(SquarePuzzlePermutateByCrossResolver { puzzle: puzzle })
    }

    pub fn get_solutions(&mut self) -> Result<Vec<PuzzleSolution>, String> {
        let mut solutions: Vec<PuzzleSolution> = vec![];
        let mut cross_map: HashMap<String, bool> = HashMap::new();

        match self.permutate_and_check_is_solved(&mut solutions, &mut cross_map, 0) {
            Err(error) => Err(error),
            Ok(()) => Ok(solutions)
        }
    }

    fn permutate_and_check_is_solved(&mut self, solutions: &mut Vec<PuzzleSolution>, map: &mut HashMap<String, bool>, start: usize) -> Result<(), String> {
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
            let _ = self.permutate_and_check_is_solved(solutions, map, start + 1);
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

pub trait ConsoleDisplayablePuzzle {
    fn show_in_console(&self);
}

impl ConsoleDisplayablePuzzle for SquarePuzzle {
    fn show_in_console(&self) {
        println!("I am a Square Puzzle. I have {} tiles.", self.tiles.len());
    }
}
