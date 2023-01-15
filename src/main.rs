use std::io::{self, BufRead, stdin, Stdin};
use board::TicTacToeBoard;
use console_player::ConsolePlayer;
use player::Player;

mod board;
mod test_board;
mod player;
mod console_player;
mod board_tile;


fn main() {
    let mut board = TicTacToeBoard::new();
    let mut current_player: usize = 1;
    let player_characters = ['X', 'O'];
    let player_one: &dyn Player = &ConsolePlayer::new() as &dyn Player;
    let player_two: &dyn Player = &ConsolePlayer::new() as &dyn Player;
    let players = [player_one, player_two];
    loop {
        let player = players[current_player - 1];
        let player_character = player_characters[current_player - 1];
        println!("Player {}'s turn has started.", current_player);
        board.print_board();

        let move_position = player.get_move();
        let is_success = board.make_move(move_position, player_character);
        if is_success {
            if board.check_for_win_or_draw() {
                println!("Player {} has won!", current_player);
                board.print_board();

                let mut player_another = false;
                player_another = player.replay_game();
                if player_another {
                    board.reset_board();
                    current_player = 1;
                    continue;
                }
                break;
            }
        }

        if current_player == 2 {
            current_player = 1;
            continue;
        }
        current_player += 1;
    }
    println!("Thank you for playing!");
}


