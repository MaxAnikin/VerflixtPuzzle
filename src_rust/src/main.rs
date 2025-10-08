use colored::Colorize;
use std::io;

struct SquarePuzzle {
    orientation: u8
}

trait ConsoleDisplayablePuzzle {
    fn show_in_console(&self);
}

impl ConsoleDisplayablePuzzle for SquarePuzzle
{
    fn show_in_console(&self){
        println!("{} {}", "I am a Square Puzzle. Current orientation: ", self.orientation.to_string());
    }
}

// test the example with `cargo run --example most_simple`
fn main() -> io::Result<()> {
    println!("{}", "Starting a puzzle game... Enjoy!".green());

    println!("{}", "Creating a puzzle...");

    let puzzle: SquarePuzzle = create_puzzle();
    puzzle.show_in_console();
    
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(())
}

fn create_puzzle() -> SquarePuzzle {
    SquarePuzzle { orientation: 0 }
}