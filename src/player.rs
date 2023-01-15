use std::io::BufRead;
use crate::console_player::ConsolePlayer;

pub trait Player {
    fn get_move(&self) -> usize;
    fn replay_game(&self) -> bool;
}
