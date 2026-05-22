use colored::Colorize;
mod puzzle_mod;
use puzzle_mod::*;
use puzzle_mod::{BLUE, RED, GREEN, YELLOW, BODY_PATTERN, TAIL, HEAD};
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("{}", "Starting a puzzle game... Enjoy!".blue());
    println!("{}", "Reading the all_red_puzzle...");
    // initialize solver with all-red puzzle from JSON
    // let all_red_puzzle: SquarePuzzle = read_json("all_red_puzzle.json").unwrap_or_else(|err| panic!("Error reading all_red_puzzle JSON: {}", err));
    // all_red_puzzle.show_in_console();

    println!("{}", "Reading the one_cross_solution_puzzle...");
    // initialize solver with all-red puzzle from JSON
    let mut one_cross_solution_puzzle: SquarePuzzle = read_json("one_cross_solution_puzzle.json").unwrap_or_else(|err| panic!("Error reading one_cross_solution_puzzle JSON: {}", err));

    // Create resolver and find solutions for the all-red puzzle
    println!("{}", "Creating \"Permutation by cross\" resolver...");
    let mut resolver: SquarePuzzlePermutateByCrossResolver = SquarePuzzlePermutateByCrossResolver::new(one_cross_solution_puzzle).unwrap_or_else(|err| panic!("Error creating resolver: {}", err));

    println!("{}", "Working on solutions...");
    let start: Instant = Instant::now();
    let solutions: Vec<PuzzleSolution> = resolver.get_solutions().unwrap_or_else(|err| panic!("Solver error: {}", err));
    
    resolver.print_count();
    let duration: std::time::Duration = start.elapsed();
    //println!("Found {} solutions for all-red puzzle in {:?}.", solutions.len(), duration);

    let start: Instant = Instant::now();
    let _s1: Result<PuzzleSolution, String> = resolver.puzzle.solve_with_permutations();
    let duration: std::time::Duration = start.elapsed();
    println!("Permutations generated in: {:?}", duration);

    println!("{}", "Game is over. Press ENTER to close.".blue());
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Read input error.");
}


fn read_json<T>(path: impl AsRef<Path>) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data = serde_json::from_str(&contents)?;
    Ok(data)
}

fn save_json_pretty<T>(data: &T, path: impl AsRef<Path>) -> std::io::Result<()>
where
    T: Serialize,
{
    let mut file: File = File::create(path)?;
    let json: String = serde_json::to_string_pretty(data)?;
    file.write_all(json.as_bytes())
}

// Make puzzle creation functions public for use in main
// pub fn create_puzzle_all_red() -> SquarePuzzle {
//     let tiles: Vec<Tile> = vec![
//         // Tile 00
//         Tile {
//             id: "00".to_string(),
//             cur_rotation: 0,
//             sides: vec![
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//             ],
//         },
//                 // Tile 01
//         Tile {
//             id: "01".to_string(),
//             cur_rotation: 0,
//             sides: vec![
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//             ],
//         },
//                 // Tile 02
//         Tile {
//             id: "02".to_string(),
//             cur_rotation: 0,
//             sides: vec![
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//             ],
//         },
//                 // Tile 10
//         Tile {
//             id: "10".to_string(),
//             cur_rotation: 0,
//             sides: vec![
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//             ],
//         },
//                 // Tile 11
//         Tile {
//             id: "11".to_string(),
//             cur_rotation: 0,
//             sides: vec![
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//             ],
//         },
//                 // Tile 12
//         Tile {
//             id: "12".to_string(),
//             cur_rotation: 0,
//             sides: vec![
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//             ],
//         },
//                 // Tile 20
//         Tile {
//             id: "20".to_string(),
//             cur_rotation: 0,
//             sides: vec![
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//             ],
//         },
//                 // Tile 21
//         Tile {
//             id: "21".to_string(),
//             cur_rotation: 0,
//             sides: vec![
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//             ],
//         },
//                 // Tile 22
//         Tile {
//             id: "22".to_string(),
//             cur_rotation: 0,
//             sides: vec![
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: HEAD,
//                 },
//                 TileSide {
//                     equal_part: RED,
//                     bit_pattern: BODY_PATTERN,
//                     bit_value: TAIL,
//                 },
//             ],
//         }
//     ];
//     SquarePuzzle { tiles: tiles, is_solved: false }
// }

 pub fn create_puzzle() -> SquarePuzzle {
    let tiles: Vec<Tile> = vec![
        // Tile 00
        Tile {
            id: "00".to_string(),
            cur_rotation: 0,
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
        // ...remaining tiles...
    ];
    SquarePuzzle { tiles: tiles, is_solved: false }
}

// ...existing code...
