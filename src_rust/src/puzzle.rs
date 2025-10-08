 use colored::Colorize;

 mod puzzle_game {
    #[derive(Debug)]
    pub struct SquarePuzzle {
        orientation: u8
    }

    pub trait consoleDisplayablePuzzle{
        fn showInConsole();
    }

    impl consoleDisplayablePuzzle for SquarePuzzle
    {
        fn showInConsole(){
            println!("{}", "SquarePuzzle".blue());
        }
    }
}