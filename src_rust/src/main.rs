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
        println!("{} {}", "I am a Square Puzzle. Current orientation: ", self.orientation);
    }
}

fn main() {
    println!("{}", "Starting a puzzle game... Enjoy!".green());

    println!("{}", "Creating a puzzle...");

    let puzzle: SquarePuzzle = create_puzzle();
    puzzle.show_in_console();

    println!("{}", "Game is over. Press ENTER to close.".green());
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Read input error."
    );
}

fn create_puzzle() -> SquarePuzzle {
    SquarePuzzle { orientation: 0 }
}