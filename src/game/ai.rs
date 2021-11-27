use super::{game::Game, game_state::GameState};
use core::cmp::{max, min};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CellLocation {
    pub row: usize,
    pub col: usize,
}

#[wasm_bindgen]
pub fn get_best_move(
    game: &Game,
    current_player_symbol: char,
    other_player_symbol: char,
) -> CellLocation {
    let (_, cell_location) = alpha_beta(
        game,
        current_player_symbol,
        std::i8::MIN,
        other_player_symbol,
        std::i8::MAX,
        true,
    );

    cell_location
}

fn alpha_beta(
    game: &Game,
    alpha_player: char,
    alpha: i8,
    beta_player: char,
    beta: i8,
    maximizing_player: bool,
) -> (i8, CellLocation) {
    let game_state = game.get_game_state();
    if game_state.is_win() {
        if maximizing_player {
            return (-1, CellLocation { row: 0, col: 0 });
        }
        return (1, CellLocation { row: 0, col: 0 });
    }

    if game_state == GameState::Draw {
        return (0, CellLocation { row: 0, col: 0 });
    }

    let mut alpha = alpha;
    let mut beta = beta;
    if maximizing_player {
        let mut maximized_move = (std::i8::MIN, CellLocation { row: 0, col: 0 });

        for cell_location in get_possible_moves(game) {
            let mut game_copy = *game;
            game_copy.set_cell_value(cell_location.row, cell_location.col, alpha_player);
            let result = alpha_beta(&game_copy, alpha_player, alpha, beta_player, beta, false);

            if result.0 > maximized_move.0 {
                maximized_move = (result.0, cell_location);
            }

            if result.0 >= beta {
                break;
            }

            alpha = max(alpha, result.0);
        }

        maximized_move
    } else {
        let mut minimized_move = (std::i8::MAX, CellLocation { row: 0, col: 0 });

        for cell_location in get_possible_moves(game) {
            let mut game_copy = *game;
            game_copy.set_cell_value(cell_location.row, cell_location.col, beta_player);
            let result = alpha_beta(&game_copy, alpha_player, alpha, beta_player, beta, true);

            if result.0 < minimized_move.0 {
                minimized_move = (result.0, cell_location);
            }

            if result.0 <= alpha {
                break;
            }

            beta = min(beta, result.0);
        }

        minimized_move
    }
}

fn get_possible_moves(game: &Game) -> Vec<CellLocation> {
    (0..game.dimensions())
        .flat_map(|row| (0..game.dimensions()).map(move |col| CellLocation { row, col }))
        .filter(|cell_location| game.is_valid_move(cell_location.row, cell_location.col))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_move_is_winning_move_on_row() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(1, 0, 'X');
        game.set_cell_value(0, 0, 'O');
        game.set_cell_value(2, 0, 'X');
        game.set_cell_value(0, 1, 'O');
        game.set_cell_value(1, 1, 'X');

        let best_move = get_best_move(&game, 'O', 'X');
        assert_eq!(best_move.row, 0);
        assert_eq!(best_move.col, 2);
    }

    #[test]
    fn best_move_is_winning_move_on_col() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(1, 1, 'X');
        game.set_cell_value(0, 0, 'O');
        game.set_cell_value(1, 2, 'X');
        game.set_cell_value(1, 0, 'O');
        game.set_cell_value(0, 1, 'X');

        let best_move = get_best_move(&game, 'O', 'X');
        assert_eq!(best_move.row, 2);
        assert_eq!(best_move.col, 0);
    }

    #[test]
    fn best_move_is_winning_mov_on_diag() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 1, 'X');
        game.set_cell_value(0, 0, 'O');
        game.set_cell_value(0, 2, 'X');
        game.set_cell_value(1, 0, 'O');
        game.set_cell_value(2, 0, 'X');
        game.set_cell_value(1, 1, 'O');
        game.set_cell_value(1, 2, 'X');

        let best_move = get_best_move(&game, 'O', 'X');

        assert_eq!(best_move.row, 2);
        assert_eq!(best_move.col, 2);
    }

    #[test]
    fn best_move_prevents_other_player_win() {
        let mut game = Game::new('X', 'O');
        game.set_cell_value(0, 0, 'X');
        game.set_cell_value(0, 1, 'X');

        let best_move = get_best_move(&game, 'O', 'X');

        assert_eq!(best_move.row, 0);
        assert_eq!(best_move.col, 2);
    }
}
