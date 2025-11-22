# Copilot Instructions for VerflixtPuzzle

## Project Overview
- This is a Rust implementation of a tile-based puzzle game, focused on solving a 3x3 square puzzle using permutations and color/bit patterns.
- Main logic is in `src/main.rs` and `src/puzzle.rs`. The puzzle is modeled with `Tile`, `TileSide`, and `SquarePuzzle` structs.
- The solution approach involves generating permutations and checking cross-tile constraints using custom logic.

## Architecture & Data Flow
- Tiles are represented by the `Tile` struct, with four sides (`TileSide`). Each side has color and bit pattern attributes.
- The puzzle state is managed by `SquarePuzzle`, which holds a vector of tiles and tracks whether the puzzle is solved.
- Permutation-based solving is implemented in `SquarePuzzlePermutateByCrossResolver`, which recursively generates tile arrangements and checks for valid solutions.
- The main entry point is the `main()` function in `src/main.rs`, which sets up puzzles, runs solvers, and prints results.

## Developer Workflows
- **Build:** Use the VS Code task `cargo Build` or run `cargo build --bin=verflist_puzzle` in the terminal.
- **Run:** Use the VS Code task `cargo run` or run `cargo run --bin=verflist_puzzle`.
- **Debug:** The main logic is in `main.rs`; start debugging from `main()`.
- **Dependencies:** Uses `colored` for console output. All dependencies are managed via `Cargo.toml`.

## Project-Specific Conventions
- Only 3x3 (9-tile) square puzzles are supported; code will panic for other sizes.
- Color constants (`BLUE`, `RED`, `GREEN`, `YELLOW`) and bit patterns are defined at the top of `main.rs`.
- Puzzle creation functions (`create_puzzle`, `create_puzzle_all_red`) are in `main.rs` and provide example setups.
- Console output uses colored text for status messages (e.g., solved/unsolved).
- Trait `ConsoleDisplayablePuzzle` is used for console rendering of puzzle state.

## Integration Points
- No external service integration; all logic is local and self-contained.
- Extend puzzle logic by adding new structs or methods in `src/puzzle.rs` and updating `main.rs`.

## Key Files
- `src/main.rs`: Main entry, puzzle setup, and execution.
- `src/puzzle.rs`: Puzzle struct definitions and trait implementations.
- `Cargo.toml`: Dependency and build configuration.

## Example Patterns
- To add a new puzzle type, define a struct and implement `ConsoleDisplayablePuzzle`.
- To change tile logic, update `Tile` and `TileSide` methods in `main.rs`.

---
For questions or unclear sections, review the code in `src/main.rs` and `src/puzzle.rs` for concrete examples.
