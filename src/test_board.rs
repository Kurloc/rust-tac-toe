#[cfg(test)]
mod test_board {
    use test_case::test_case;
    use crate::board::TicTacToeBoard;

    #[test_case(0, 1, 2, true)]
    #[test_case(3, 4, 5, true)]
    #[test_case(6, 7, 8, true)]
    #[test_case(2, 4, 6, true)]
    #[test_case(0, 4, 8, true)]
    #[test_case(1, 4, 8, false)]
    #[test_case(2, 4, 8, false)]
    #[test_case(1, 2, 4, false)]
    fn test_row_wins(m1: usize, m2: usize, m3: usize, expected: bool) {
        let mut board = TicTacToeBoard::new();
        let player_char = 'X';
        board.make_move(m1, player_char);
        board.make_move(m2, player_char);
        board.make_move(m3, player_char);
        board.print_board();
        let won_game = board.check_for_win_or_draw();
        assert_eq!(won_game, expected);
    }

    #[test]
    fn test_game_draw() {
        let mut board = TicTacToeBoard::new();
        let player_chars = ['X', 'O'];
        for i in 0..8 {
            let player_character = if i % 2 == 0 { player_chars[0] } else { player_chars[1] };
            board.make_move(i, player_character);
        }
        board.print_board();
        let won_game = board.check_for_win_or_draw();
        assert_eq!(won_game, true);
    }
}