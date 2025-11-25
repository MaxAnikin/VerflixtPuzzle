use colored::Colorize;
mod puzzle_mod;
use puzzle_mod::*;
use puzzle_mod::{BLUE, RED, GREEN, YELLOW, BODY_PATTERN, TAIL, HEAD};

use std::io;
use std::time::Instant;

fn main() {
    println!("{}", "Starting a puzzle game... Enjoy!".blue());
    println!("{}", "Creating a puzzle...");

    let puzzle: SquarePuzzle = create_puzzle();
    puzzle.show_in_console();
    puzzle.print_is_solved("Base puzzle solve result".to_string());

    let all_red_puzzle: SquarePuzzle = create_puzzle_all_red();
    let mut resolver: SquarePuzzlePermutateByCrossResolver = SquarePuzzlePermutateByCrossResolver::new(all_red_puzzle).unwrap_or_else(|err| panic!("Error creating resolver: {}", err));
    let solutions: Vec<PuzzleSolution> = resolver.get_solutions().unwrap_or_else(|err| panic!("Solver error: {}", err));
    println!("Found {} solutions for all-red puzzle.", solutions.len());

    let start: Instant = Instant::now();
    let _s1: Result<PuzzleSolution, String> = puzzle.solve_with_permutations();
    let duration: std::time::Duration = start.elapsed();
    println!("Permutations generated in: {:?}", duration);

    println!("{}", "Game is over. Press ENTER to close.".blue());
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Read input error.");
}

// Make puzzle creation functions public for use in main
pub fn create_puzzle_all_red() -> SquarePuzzle {
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
        // ...remaining tiles...
    ];
    SquarePuzzle { tiles: tiles, is_solved: false }
}

pub fn create_puzzle() -> SquarePuzzle {
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
        // ...remaining tiles...
    ];
    SquarePuzzle { tiles: tiles, is_solved: false }
}

// ...existing code...
