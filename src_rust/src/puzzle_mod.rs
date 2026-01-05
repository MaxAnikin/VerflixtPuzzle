// puzzle_mod.rs
// Contains all puzzle logic except main, create_puzzle, and create_puzzle_all_red

use colored::Colorize;
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use std::collections::HashMap;

pub const BLUE: u8 = 1;
pub const RED: u8 = 2;
pub const GREEN: u8 = 3;
pub const YELLOW: u8 = 4;
pub const BODY_PATTERN: u8 = 3;
pub const TAIL: u8 = 1;
pub const HEAD: u8 = 2;

fn color_to_str(value: u8) -> &'static str {
    match value {
        BLUE => "BLUE",
        RED => "RED",
        GREEN => "GREEN",
        YELLOW => "YELLOW",
        _ => "UNKNOWN",
    }
}

fn str_to_color(s: &str) -> u8 {
    match s {
        "BLUE" => BLUE,
        "RED" => RED,
        "GREEN" => GREEN,
        "YELLOW" => YELLOW,
        _ => 0,
    }
}

fn body_to_str(value: u8) -> &'static str {
    match value {
        TAIL => "TAIL",
        HEAD => "HEAD",
        _ => "UNKNOWN",
    }
}

fn str_to_body(s: &str) -> u8 {
    match s {
        "TAIL" => TAIL,
        "HEAD" => HEAD,
        _ => 0,
    }
}

fn serialize_color<S>(value: &u8, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(color_to_str(*value))
}

fn deserialize_color<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(str_to_color(&s))
}

fn serialize_body<S>(value: &u8, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(body_to_str(*value))
}

fn deserialize_body<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    Ok(str_to_body(&s))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileSide {
    #[serde(serialize_with = "serialize_color", deserialize_with = "deserialize_color")]
    pub equal_part: u8,
    pub bit_pattern: u8,
    #[serde(serialize_with = "serialize_body", deserialize_with = "deserialize_body")]
    pub bit_value: u8,
}

impl TileSide {
    pub fn compare(&self, side: &TileSide) -> bool {
        self.equal_part == side.equal_part
            && self.bit_pattern == side.bit_pattern
            && (self.bit_value | side.bit_value == self.bit_pattern)
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Clone)]
pub struct TilePosition {
    pub id: String,
    pub direction: u8,
}

#[derive(Debug, Clone)]
pub struct PuzzleSolution {
    pub tile_positions: Vec<TilePosition>,
}

impl PuzzleSolution {
    pub fn print(&self) {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut output = String::from("solution: ");

        for (i, tile_position) in self.tile_positions.iter().enumerate() {
            output.push_str(&tile_position.id);
            if i != self.tile_positions.len() - 1 {
                output.push(' ');
            }
        }
        output.push('\n');

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("permurtate.txt")
            .expect("Unable to open permurtate.txt");
        
        file.write_all(output.as_bytes())
            .expect("Unable to write to permurtate.txt");
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SquarePuzzle {
    pub tiles: Vec<Tile>,
    pub is_solved: bool,
}

impl SquarePuzzle {
    pub fn print_is_solved(&self, message: String) {
        let is_solved: bool = self.is_solved;
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

    pub fn get_cross_key(&self) -> String {
        if self.tiles.len() != 9 {
            panic!("Cross string key requires 9 tiles.");
        }
        format!(
            "{}{}{}{}{}",
            self.tiles[1].id,
            self.tiles[3].id,
            self.tiles[4].id,
            self.tiles[5].id,
            self.tiles[7].id
        )
    }

    pub fn solve_with_permutations(&self) -> Result<PuzzleSolution, String> {
        // Implementation placeholder
        Err("Not implemented".to_string())
    }
}

#[derive(Debug)]
pub struct SquarePuzzlePermutateByCrossResolver {
    pub puzzle: SquarePuzzle,
    solutions: Vec<PuzzleSolution>,
    cross_map: HashMap<String, bool>,
    counter: u32,
}

impl SquarePuzzlePermutateByCrossResolver {
    pub fn new(puzzle: SquarePuzzle) -> Result<Self, String> {
        if puzzle.tiles.len() != 9 {
            return Err("Only square puzzles with nine tiles are supported.".to_string());
        }
        
        // Очистить файл при создании резолвера
        use std::fs::File;
        File::create("permurtate.txt").expect("Unable to create/clear permurtate.txt");
        
        Ok(SquarePuzzlePermutateByCrossResolver { puzzle: puzzle, solutions: vec![], cross_map: HashMap::new(), counter: 0 })
    }

    pub fn print_count(&self) {
        use std::fs::OpenOptions;
        use std::io::Write;

        let output = format!("solution: Total permutations checked: {}\n", self.counter);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("permurtate.txt")
            .expect("Unable to open permurtate.txt");
        
        file.write_all(output.as_bytes())
            .expect("Unable to write to permurtate.txt");
    }

    pub fn get_solutions(&mut self) -> Result<Vec<PuzzleSolution>, String> {
        match self.permutate_tiles(0) {
            Err(error) => Err(error),
            Ok(()) => Ok(self.solutions.clone()),
        }
    }

    fn permutate_tiles(&mut self, cur: usize) -> Result<(), String> {
        if cur == self.puzzle.tiles.len() {
            self.counter += 1;
            return Ok(());
        }

        for i in cur..self.puzzle.tiles.len() {
            self.puzzle.tiles.swap(cur, i);
            
            let _ = self.permutate_tiles(cur + 1);

            self.puzzle.tiles.swap(cur, i);
        }

        // if start == self.puzzle.tiles.len() {
        //     let cross_key: String = match self.get_cross_key() {
        //         Err(error) => {
        //             return Err(error);
        //         }
        //         Ok(key) => key,
        //     };

        //     let cross_solved: bool = match self.cross_map.get(&cross_key) {
        //         Some(entry) => *entry,
        //         None => {
        //             let cross_solutions: Vec<String> = vec![];
        //             self.cross_map.insert(cross_key, true);
        //             true
        //         }
        //     };
        // }

        // for i in start..self.puzzle.tiles.len() {
        //     self.puzzle.tiles.swap(start, i);
        //     let _ = self.permutate(start + 1);
        //     self.puzzle.tiles.swap(start, i);
        // }

        Ok(())
    }

    fn get_cross_key(&self) -> Result<String, String> {
        if self.puzzle.tiles.len() != 9 {
            return Err("Cross string key requires 9 tiles.".to_string());
        }
        Ok(format!(
            "{}{}{}{}{}",
            self.puzzle.tiles[1].id,
            self.puzzle.tiles[3].id,
            self.puzzle.tiles[4].id,
            self.puzzle.tiles[5].id,
            self.puzzle.tiles[7].id
        ))
    }
    
    fn get_current_solution(&self) -> PuzzleSolution {
        let mut tile_positions: Vec<TilePosition> = vec![];
        for tile in &self.puzzle.tiles {
            tile_positions.push(TilePosition {
                id: tile.id.clone(),
                direction: tile.direction,
            });
        }
        PuzzleSolution { tile_positions }
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
