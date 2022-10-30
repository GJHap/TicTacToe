use super::game_state::GameState;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Game {
    cells: [[char; Game::DIMENSION]; Game::DIMENSION],
    player1_symbol: char,
    player2_symbol: char,
}

#[wasm_bindgen]
impl Game {
    pub fn dimensions(&self) -> usize {
        Game::DIMENSION
    }

    pub fn get_cell_value(&self, row: usize, col: usize) -> char {
        self.cells[row][col]
    }

    pub fn get_game_state(&self) -> GameState {
        let row_state = self.get_row_game_state();
        if row_state.is_win() {
            return row_state;
        }

        let col_state = self.get_col_game_state();
        if col_state.is_win() {
            return col_state;
        }

        let diag_state = self.get_game_diagonal_state();
        if diag_state.is_win() {
            return diag_state;
        }

        if self.is_board_full() {
            return GameState::Draw;
        }

        GameState::InProgress
    }

    pub fn is_valid_move(&self, row: usize, col: usize) -> bool {
        self.cells[row][col] == Game::EMPTY_CELL
    }

    #[wasm_bindgen(constructor)]
    pub fn new(player1_symbol: char, player2_symbol: char) -> Game {
        Game {
            player1_symbol: player1_symbol,
            player2_symbol: player2_symbol,
            cells: [[Game::EMPTY_CELL; Game::DIMENSION]; Game::DIMENSION],
        }
    }

    pub fn set_cell_value(&mut self, row: usize, col: usize, value: char) {
        self.cells[row][col] = value;
    }
}

impl Game {
    const DIMENSION: usize = 3;
    const EMPTY_CELL: char = ' ';

    fn get_col_game_state(&self) -> GameState {
        let wins = (0..self.dimensions())
            .map(|col| {
                let winning_symbol = self.cells[0][col];
                let is_col_win = winning_symbol != Game::EMPTY_CELL
                    && (0..self.dimensions()).all(|row| self.cells[row][col] == winning_symbol);

                return (is_col_win, winning_symbol);
            })
            .filter(|win| win.0)
            .collect::<Vec<(bool, char)>>();

        if wins.is_empty() {
            return GameState::InProgress;
        }
        GameState::from_winner(wins[0].1, self.player1_symbol, self.player2_symbol)
    }

    fn get_game_diagonal_state(&self) -> GameState {
        let winning_symbol = self.cells[0][0];
        let is_diag_win = winning_symbol != Game::EMPTY_CELL
            && (0..self.dimensions()).all(|idx| self.cells[idx][idx] == winning_symbol);

        if is_diag_win {
            return GameState::from_winner(
                winning_symbol,
                self.player1_symbol,
                self.player2_symbol,
            );
        }

        let winning_symbol = self.cells[0][self.dimensions() - 1];
        let is_diag_win = winning_symbol != Game::EMPTY_CELL
            && (0..self.dimensions())
                .all(|idx| self.cells[idx][self.dimensions() - idx - 1] == winning_symbol);

        if is_diag_win {
            return GameState::from_winner(
                winning_symbol,
                self.player1_symbol,
                self.player2_symbol,
            );
        }

        GameState::InProgress
    }

    fn get_row_game_state(&self) -> GameState {
        let wins = (0..self.dimensions())
            .map(|row| {
                let winning_symbol = self.cells[row][0];
                let is_row_win = winning_symbol != Game::EMPTY_CELL
                    && (0..self.dimensions()).all(|col| self.cells[row][col] == winning_symbol);

                return (is_row_win, winning_symbol);
            })
            .filter(|win| win.0)
            .collect::<Vec<(bool, char)>>();

        if wins.is_empty() {
            return GameState::InProgress;
        }
        GameState::from_winner(wins[0].1, self.player1_symbol, self.player2_symbol)
    }

    fn is_board_full(&self) -> bool {
        return (0..self.dimensions())
            .all(|row| (0..self.dimensions()).all(|col| self.cells[row][col] != Game::EMPTY_CELL));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_is_not_valid_when_cell_is_set() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 0, 'X');

        let is_valid = game.is_valid_move(0, 0);

        assert!(!is_valid);
    }

    #[test]
    fn move_is_valid_when_cell_is_empty() {
        let game = Game::new('X', 'O');

        let is_valid = game.is_valid_move(0, 0);

        assert!(is_valid);
    }

    #[test]
    fn state_is_player1_when_row1_is_player1() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 0, 'X');
        game.set_cell_value(0, 1, 'X');
        game.set_cell_value(0, 2, 'X');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player1);
    }

    #[test]
    fn state_is_player1_when_row2_is_player1() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(1, 0, 'X');
        game.set_cell_value(1, 1, 'X');
        game.set_cell_value(1, 2, 'X');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player1);
    }

    #[test]
    fn state_is_player1_when_row3_is_player1() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(2, 0, 'X');
        game.set_cell_value(2, 1, 'X');
        game.set_cell_value(2, 2, 'X');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player1);
    }

    #[test]
    fn state_is_player1_when_col1_is_player1() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 0, 'X');
        game.set_cell_value(1, 0, 'X');
        game.set_cell_value(2, 0, 'X');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player1);
    }

    #[test]
    fn state_is_player1_when_col2_is_player1() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 1, 'X');
        game.set_cell_value(1, 1, 'X');
        game.set_cell_value(2, 1, 'X');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player1);
    }

    #[test]
    fn state_is_player1_when_col3_is_player1() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 2, 'X');
        game.set_cell_value(1, 2, 'X');
        game.set_cell_value(2, 2, 'X');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player1);
    }

    #[test]
    fn state_is_player1_when_diag1_is_player1() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 0, 'X');
        game.set_cell_value(1, 1, 'X');
        game.set_cell_value(2, 2, 'X');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player1);
    }

    #[test]
    fn state_is_player1_when_diag2_is_player1() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 2, 'X');
        game.set_cell_value(1, 1, 'X');
        game.set_cell_value(2, 0, 'X');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player1);
    }

    #[test]
    fn state_is_player1_when_win_on_last_move() {
        let mut game = Game::new('X', 'O');

        game.set_cell_value(0, 0, 'X');
        game.set_cell_value(0, 1, 'O');
        game.set_cell_value(0, 2, 'O');

        game.set_cell_value(1, 0, 'X');
        game.set_cell_value(1, 1, 'X');
        game.set_cell_value(1, 2, 'O');

        game.set_cell_value(2, 0, 'O');
        game.set_cell_value(2, 1, 'O');
        game.set_cell_value(2, 2, 'X');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player1);
    }

    #[test]
    fn state_is_player2_when_row1_is_player2() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 0, 'O');
        game.set_cell_value(0, 1, 'O');
        game.set_cell_value(0, 2, 'O');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player2);
    }

    #[test]
    fn state_is_player2_when_row2_is_player2() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(1, 0, 'O');
        game.set_cell_value(1, 1, 'O');
        game.set_cell_value(1, 2, 'O');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player2);
    }

    #[test]
    fn state_is_player2_when_row3_is_player2() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(2, 0, 'O');
        game.set_cell_value(2, 1, 'O');
        game.set_cell_value(2, 2, 'O');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player2);
    }

    #[test]
    fn state_is_player2_when_col1_is_player2() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 0, 'O');
        game.set_cell_value(1, 0, 'O');
        game.set_cell_value(2, 0, 'O');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player2);
    }

    #[test]
    fn state_is_player2_when_col2_is_player2() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 1, 'O');
        game.set_cell_value(1, 1, 'O');
        game.set_cell_value(2, 1, 'O');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player2);
    }

    #[test]
    fn state_is_player2_when_col3_is_player2() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 2, 'O');
        game.set_cell_value(1, 2, 'O');
        game.set_cell_value(2, 2, 'O');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player2);
    }

    #[test]
    fn state_is_player2_when_diag1_is_player2() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 0, 'O');
        game.set_cell_value(1, 1, 'O');
        game.set_cell_value(2, 2, 'O');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player2);
    }

    #[test]
    fn state_is_player2_when_diag2_is_player2() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 2, 'O');
        game.set_cell_value(1, 1, 'O');
        game.set_cell_value(2, 0, 'O');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player2);
    }

    #[test]
    fn state_is_player2_when_win_on_last_move() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 0, 'O');
        game.set_cell_value(0, 1, 'X');
        game.set_cell_value(0, 2, 'X');

        game.set_cell_value(1, 0, 'O');
        game.set_cell_value(1, 1, 'O');
        game.set_cell_value(1, 2, 'X');

        game.set_cell_value(2, 0, 'X');
        game.set_cell_value(2, 1, 'X');
        game.set_cell_value(2, 2, 'O');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Player2);
    }

    #[test]
    fn state_is_draw_when_board_is_full() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 0, 'X');
        game.set_cell_value(0, 1, 'X');
        game.set_cell_value(0, 2, 'O');

        game.set_cell_value(1, 0, 'O');
        game.set_cell_value(1, 1, 'O');
        game.set_cell_value(1, 2, 'X');

        game.set_cell_value(2, 0, 'X');
        game.set_cell_value(2, 1, 'O');
        game.set_cell_value(2, 2, 'X');

        let state = game.get_game_state();

        assert_eq!(state, GameState::Draw);
    }

    #[test]
    fn state_is_inprogress_when_board_is_empty() {
        let game = Game::new('X', 'O');
        let state = game.get_game_state();

        assert_eq!(state, GameState::InProgress);
    }
}
