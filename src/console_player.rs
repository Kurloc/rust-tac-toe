use std::io::{BufRead, stdin, Stdin};
use crate::player::Player;

pub struct ConsolePlayer {
    std_in: Stdin,
}

impl ConsolePlayer {
    pub fn new() -> ConsolePlayer {
        return ConsolePlayer { std_in: stdin() };
    }
}

impl Player for ConsolePlayer {
    fn get_move(&self) -> usize {
        let mut keep_getting_input = false;
        let mut input_string = String::new();
        println!("Please make a move:");
        loop {
            input_string.clear();
            self.std_in
                .lock()
                .read_line(&mut input_string)
                .unwrap();

            input_string = input_string.trim().parse().unwrap();
            for c in input_string.chars() {
                if !c.is_numeric() {
                    keep_getting_input = true;
                    break;
                }
                keep_getting_input = false;
            }

            if !keep_getting_input && input_string.len() == 1 {
                let value = input_string.parse::<usize>().unwrap();
                if value >= 0 && value < 9 {
                    return value;
                }
            }

            println!("\n\nThat was an invalid move, please enter a numeric value (0-8):");
        }
    }

    fn replay_game(&self) -> bool {
        let mut input_string = String::new();
        println!("Do you want to play another game? (y or n):");
        input_string.clear();
        self.std_in
            .lock()
            .read_line(&mut input_string)
            .unwrap();

        input_string = input_string.trim().parse().unwrap();
        return match input_string.to_lowercase().chars().next().unwrap() {
            'y' => true,
            'n' => false,
            _ => false
        };
    }
}