// puzzle_mod.rs
// Contains all puzzle logic except main, create_puzzle, and create_puzzle_all_red

use colored::Colorize;
use crossterm::{cursor, execute, terminal, style::{Color as TermColor, SetForegroundColor, Print, ResetColor}};
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use std::{clone, collections::HashMap, io, thread, time::Duration};
use std::io::stdout;

use crate::create_puzzle;

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
    pub sides: Vec<TileSide>,
    pub cur_rotation: u8
}

impl Tile {
    pub fn new(id: String, sides: Vec<TileSide>) -> Self {
        Tile {
            id,
            sides,
            cur_rotation: 0,
        }
    }

    pub fn rotate(&mut self) {
        self.cur_rotation += 1;
        if self.cur_rotation > 3 {
            self.cur_rotation = 0;
        }
    }
    pub fn left_side(&self) -> &TileSide {
        let mut tile_num: i8 = 0 - self.cur_rotation as i8;

        if tile_num < 0 {
            tile_num += 4;
        }

        &self.sides[tile_num as usize]
    }
    pub fn up_side(&self) -> &TileSide {
        let mut tile_num: i8 = 1 - self.cur_rotation as i8;

        if tile_num < 0 {
            tile_num += 4;
        }

        &self.sides[tile_num as usize]
    }
    pub fn right_side(&self) -> &TileSide {
        let mut tile_num: i8 = 2 - self.cur_rotation as i8;

        if tile_num < 0 {
            tile_num += 4;
        }

        &self.sides[tile_num as usize]
    }
    pub fn down_side(&self) -> &TileSide {
        let mut tile_num: i8 = 3 - self.cur_rotation as i8;

        if tile_num < 0 {
            tile_num += 4;
        }

        &self.sides[tile_num as usize]
    }
}

#[derive(Debug, Clone)]
pub struct TilePosition {
    pub id: String,
    pub cur_rotation: u8,
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
    pub fn get_tile(&self, index: usize) -> &Tile {
        &self.tiles[index]
    }

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
        match self.permutate_and_validate(0) {
            Err(error) => Err(error),
            Ok(solutions) => Ok(solutions),
        }
    }

    // go recursively through all permutations of tiles, and validate each one
    fn permutate_and_validate(&mut self, cur: usize) -> Result<Vec<PuzzleSolution>, String> {
        let mut valid_solutions: Vec<PuzzleSolution> = vec![];

        // end of recursion, validate current permutation
        if cur == self.puzzle.tiles.len() {
            let rotation_result: Result<Vec<PuzzleSolution>, String> = self.rotate_and_validate();

            match rotation_result {
                Err(error) => return Err(error),
                Ok(solutions) => valid_solutions.extend(solutions),
            }
        }

        for i in cur..self.puzzle.tiles.len() {
            self.puzzle.tiles.swap(cur, i);
            
            let _ = self.permutate_and_validate(cur + 1);

            self.puzzle.tiles.swap(cur, i);
        }

        Ok(valid_solutions)
    }

    fn rotate_and_validate(&mut self) -> Result<Vec<PuzzleSolution>, String> {
        let mut valid_solutions: Vec<PuzzleSolution> = vec![];

        let cross_solutions_result: Result<Vec<PuzzleSolution>, String> = self.get_cross_solutions();
        match cross_solutions_result {
            Err(error) => return Err(error),
            // return if empty cross solutions, no need to rotate corners
            Ok(ref v) if v.is_empty() => return Ok(vec![]),
            // continue if there are cross solutions
            Ok(_) => {}
        }

        let cross_solutions = cross_solutions_result.unwrap();
        for i in 0..cross_solutions.len() {
            // apply cross solution to the puzzle 
            let apply_result: Result<(), String> = self.apply_cross_solution(cross_solutions[i].clone());
            match apply_result {
                Err(error) => return Err(error),
                // continue if applied successfully
                Ok(_) => {}
            }

            let full_solutions: Result<Vec<PuzzleSolution>, String> = self.rotate_and_validate_corners();
            match full_solutions {
                Err(error) => return Err(error),
                // return if empty full solutions
                Ok(ref v) if v.is_empty() => return Ok(vec![]),
                // continue if there are full solutions
                Ok(_) => {}
            }

            valid_solutions.extend(full_solutions.unwrap());
        }

        Ok(valid_solutions)
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
                cur_rotation: tile.cur_rotation,
            });
        }
        PuzzleSolution { tile_positions }
    }
    
    fn is_solved(&self) -> Result<bool, String> {
        let t = &self.puzzle.tiles;

        // Horizontal pairs: right side of left tile must match left side of right tile
        let h_pairs = [(0,1),(1,2),(3,4),(4,5),(6,7),(7,8)];
        for (a, b) in h_pairs.iter() {
            if !t[*a].right_side().compare(t[*b].left_side()) {
                return Ok(false);
            }
        }

        // Vertical pairs: down side of top tile must match up side of bottom tile
        let v_pairs = [(0,3),(1,4),(2,5),(3,6),(4,7),(5,8)];
        for (a, b) in v_pairs.iter() {
            if !t[*a].down_side().compare(t[*b].up_side()) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn is_cross_solved(&self) -> Result<bool, String> {
        let t = &self.puzzle.tiles;

        // Horizontal pairs: right side of left tile must match left side of right tile
        let h_pairs = [(3,4),(4,5)];
        for (a, b) in h_pairs.iter() {
            if !t[*a].right_side().compare(t[*b].left_side()) {
                return Ok(false);
            }
        }

        // Vertical pairs: down side of top tile must match up side of bottom tile
        let v_pairs = [(1,4),(4,7)];
        for (a, b) in v_pairs.iter() {
            if !t[*a].down_side().compare(t[*b].up_side()) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn get_cross_solutions(&mut self) -> Result<Vec<PuzzleSolution>, String> {
        // todo!(try reading from cache);

        let mut cross_tiles: Vec<usize> = vec![1, 3, 4, 5, 7];
        let mut cross_solutions: Vec<PuzzleSolution> = vec![];
        self.get_cross_solutions_recursive(&mut cross_solutions, &mut cross_tiles)?;
        Ok(cross_solutions)
    }

    fn get_cross_solutions_recursive(&mut self, cross_solutions: &mut Vec<PuzzleSolution>, cross_tiles: &mut Vec<usize>) -> Result<(),String> {
        if cross_tiles.is_empty() {
            self.puzzle.show_in_console();
            println!("{}", "Press ENTER to continue.".green());

            let mut input = String::new();
                io::stdin()
                .read_line(&mut input)
                .expect("Read input error.");

            let is_solved = self.is_cross_solved();
                match is_solved {
                    Err(error) => return Err(error),    
                    Ok(true) => {
                        self.puzzle.show_in_console();
                        println!("{}", "Press ENTER to continue.".green());
                        let mut input = String::new();
                            io::stdin()
                            .read_line(&mut input)
                            .expect("Read input error.");

                        cross_solutions.push(self.get_current_solution());
                        return Ok(())
                    },
                    Ok(false) => return Ok(()),
                }
        }

        let tile_to_rotate = cross_tiles.pop().unwrap();
        for _i in 0..3 
        {
            self.get_cross_solutions_recursive(cross_solutions, cross_tiles)?;
            self.puzzle.tiles[tile_to_rotate].rotate();
        }

        return Ok(());
    }
    
    fn apply_cross_solution(&self, solution: PuzzleSolution) -> Result<(), String> {
        todo!()
    }
    
    fn rotate_and_validate_corners(&self) -> Result<Vec<PuzzleSolution>, String> {
        todo!()
    }
}

pub trait ConsoleDisplayablePuzzle {
    fn show_in_console(&self);
}

fn map_term_color(equal_part: u8) -> TermColor {
    match equal_part {
        BLUE   => TermColor::Blue,
        RED    => TermColor::Red,
        GREEN  => TermColor::Green,
        YELLOW => TermColor::Yellow,
        _      => TermColor::White,
    }
}

fn side_block(side: &TileSide) -> &'static str {
    if side.bit_value == TAIL { "TAIL" } else { "HEAD" }
}

impl ConsoleDisplayablePuzzle for SquarePuzzle {
    fn show_in_console(&self) {
        let mut out = stdout();
        execute!(out,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        ).unwrap();

        // Each tile cell: 13 cols wide, 4 rows tall (3 content + 1 gap row)
        let tile_w: u16 = 13;
        let tile_h: u16 = 4;

        for (idx, tile) in self.tiles.iter().enumerate() {
            let gc = (idx % 3) as u16;
            let gr = (idx / 3) as u16;
            let bx = gc * tile_w;
            let by = gr * tile_h;

            //  row by+0:   "  UP   "   — UP side centred
            let up = tile.up_side();
            execute!(out,
                cursor::MoveTo(bx + 4, by),
                SetForegroundColor(map_term_color(up.equal_part)),
                Print(side_block(up)),
                ResetColor
            ).unwrap();

            //  row by+1:  "LEFT  ID  RIGHT" — LEFT, tile-id, RIGHT
            let lf = tile.left_side();
            execute!(out,
                cursor::MoveTo(bx, by + 1),
                SetForegroundColor(map_term_color(lf.equal_part)),
                Print(side_block(lf)),
                ResetColor,
                cursor::MoveTo(bx + 5, by + 1),
                Print(format!("{:>2}", &tile.id)),
                cursor::MoveTo(bx + 8, by + 1)
            ).unwrap();
            let rt = tile.right_side();
            execute!(out,
                SetForegroundColor(map_term_color(rt.equal_part)),
                Print(side_block(rt)),
                ResetColor
            ).unwrap();

            //  row by+2:   "  DOWN   "   — DOWN side centred
            let dn = tile.down_side();
            execute!(out,
                cursor::MoveTo(bx + 4, by + 2),
                SetForegroundColor(map_term_color(dn.equal_part)),
                Print(side_block(dn)),
                ResetColor
            ).unwrap();
        }

        // Move cursor below the entire grid
        execute!(out, cursor::MoveTo(0, 3 * tile_h + 1)).unwrap();
    }
}

