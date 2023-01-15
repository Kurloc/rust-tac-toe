use std::collections::HashMap;
use std::fmt;
use crate::board_tile::BoardTile;

pub struct TicTacToeBoard {
    board: HashMap<usize, BoardTile>,
}

impl TicTacToeBoard {
    pub fn new() -> TicTacToeBoard {
        TicTacToeBoard {
            board: Self::create_board()
        }
    }

    pub fn create_board() -> HashMap<usize, BoardTile> {
        let mut return_board = HashMap::new();

        for i in 0..9 {
            return_board.insert(
                i,
                BoardTile {
                    value: i.to_string().chars().next().unwrap(),
                    initialized: false,
                },
            );
        }

        return return_board;
    }

    pub fn print_board(&self) {
        let b = &self.board;
        println!("{}|{}|{}", b.get(&0).unwrap(), b.get(&1).unwrap(), b.get(&2).unwrap());
        println!("{}|{}|{}", b.get(&3).unwrap(), b.get(&4).unwrap(), b.get(&5).unwrap());
        println!("{}|{}|{}", b.get(&6).unwrap(), b.get(&7).unwrap(), b.get(&8).unwrap());
        println!();
    }

    pub fn make_move(&mut self, position: usize, value: char) -> bool {
        let original_value = self.board.get(&position).unwrap();
        if original_value.initialized {
            return false;
        }
        self.board.insert(position, BoardTile { value, initialized: true });

        return true;
    }

    pub fn check_for_win_or_draw(&self) -> bool {
        let mut index = 0;
        let mut all_spaces_filled = true;
        for i in 0..9 {
            let val = self.board.get(&i).unwrap().value;
            if val.is_numeric() {
                all_spaces_filled = false;
            }

            let temp_i = i + 1;
            if temp_i % 3 == 0 && i != 0 {
                let t1 = &(i - 1);
                let t2 = &(i - 2);
                let i_value = self.board[&i].to_string().chars().skip(1).next().unwrap();
                let t1_value = self.board[t1].to_string().chars().skip(1).next().unwrap();
                let t2_value = self.board[t2].to_string().chars().skip(1).next().unwrap();
                // diagonal wins
                if temp_i == 3 {
                    let t3 = &(index + 2);
                    let t4 = &(index + 4);
                    let t5 = &(index + 6);
                    let t3_value = self.board[t3].to_string().chars().skip(1).next().unwrap();
                    let t4_value = self.board[t4].to_string().chars().skip(1).next().unwrap();
                    let t5_value = self.board[t5].to_string().chars().skip(1).next().unwrap();
                    if i_value == t3_value && t3_value == t4_value {
                        return true;
                    }
                    if t2_value == t3_value && t3_value == t5_value {
                        return true;
                    }
                }
                // row win
                if t2_value == t1_value && t1_value == i_value {
                    return true;
                }
                index = 0;
            } else {
                index += 1;
            }
        }

        if all_spaces_filled {

        }
        return false;
    }

    pub fn reset_board(&mut self) {
        for i in 0..9 {
            self.board.insert(
                i,
                BoardTile {
                    value: i.to_string().chars().next().unwrap(),
                    initialized: false,
                },
            );
        }
    }
}
