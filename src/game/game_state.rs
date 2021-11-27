use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Draw,
    Player1,
    Player2,
}

#[wasm_bindgen]
pub fn game_over_message(game_state: GameState) -> String {
    match game_state {
        GameState::InProgress => String::from("InProgress"),
        GameState::Draw => String::from("Draw"),
        GameState::Player1 => String::from("Player 1 wins!"),
        GameState::Player2 => String::from("Player 2 wins!"),
    }
}

#[wasm_bindgen]
pub fn is_gave_over(game_state: GameState) -> bool {
    match game_state {
        GameState::InProgress => false,
        GameState::Draw => true,
        GameState::Player1 => true,
        GameState::Player2 => true,
    }
}

impl GameState {
    pub fn from_winner(
        winning_symbol: char,
        player1_symbol: char,
        player2_symbol: char,
    ) -> GameState {
        if winning_symbol == player1_symbol {
            return GameState::Player1;
        } else if winning_symbol == player2_symbol {
            return GameState::Player2;
        }

        GameState::InProgress
    }

    pub fn is_win(self) -> bool {
        match self {
            GameState::Player1 => true,
            GameState::Player2 => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_over_when_isdraw() {
        let game_state = GameState::Draw;
        let is_over = is_gave_over(game_state);

        assert!(is_over);
    }

    #[test]
    fn game_over_when_is_player1() {
        let game_state = GameState::Player1;
        let is_over = is_gave_over(game_state);

        assert!(is_over);
    }

    #[test]
    fn game_over_when_is_player2() {
        let game_state = GameState::Player2;
        let is_over = is_gave_over(game_state);

        assert!(is_over);
    }

    #[test]
    fn game_not_over_when_inprogress() {
        let game_state = GameState::InProgress;
        let is_over = is_gave_over(game_state);

        assert!(!is_over);
    }

    #[test]
    fn win_when_is_player1() {
        let game_state = GameState::Player1;
        let is_win = game_state.is_win();

        assert!(is_win);
    }

    #[test]
    fn win_when_is_player2() {
        let game_state = GameState::Player2;
        let is_win = game_state.is_win();

        assert!(is_win);
    }

    #[test]
    fn not_win_when_inprogress() {
        let game_state = GameState::InProgress;
        let is_win = game_state.is_win();

        assert!(!is_win);
    }

    #[test]
    fn not_win_when_draw() {
        let game_state = GameState::Draw;
        let is_win = game_state.is_win();

        assert!(!is_win);
    }
}
