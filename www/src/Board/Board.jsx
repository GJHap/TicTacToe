import { useEffect, useRef, useState } from 'react';
import BoardRow from '../BoardRow/BoardRow';
import styles from './Board.module.css'
import init, { Game, is_gave_over, get_best_move, game_over_message } from '../wasm/tic_tac_toe'

export default function ({ onGameOver }) {
    const player1Symbol = 'X';
    const player2Symbol = 'O';
    const game = useRef(null);
    const [cells, setCells] = useState([]);

    useEffect(async () => {
        await init('../wasm/tic_tac_toe_bg.wasm')
        game.current = new Game(player1Symbol, player2Symbol);
        setCells(getCells());
    }, [])

    const onCellClicked = (row, col) => {
        let game_state = game.current.get_game_state();
        if (!is_gave_over(game_state) && game.current.is_valid_move(row, col)) {
            game.current.set_cell_value(row, col, player1Symbol);
            let game_state = game.current.get_game_state();

            if (!is_gave_over(game_state)) {
                let ai_move = get_best_move(game.current, player2Symbol, player1Symbol);
                if (game.current.is_valid_move(ai_move.row, ai_move.col)) {
                    game.current.set_cell_value(ai_move.row, ai_move.col, player2Symbol);
                    game_state = game.current.get_game_state();
                }
            }

            setCells(getCells());
            if (is_gave_over(game_state)) {
                onGameOver(game_over_message(game_state));
            }
        }
    };

    const getCells = () => {
        return [...Array(game.current.dimensions()).keys()].map(row =>
            [...Array(game.current.dimensions()).keys()].map(col =>
                game.current.get_cell_value(row, col)
            )
        )
    };

    return (
        <div id={styles.board}>
            {
                cells.flatMap((row, rowIdx) =>
                    <BoardRow key={rowIdx} rowIdx={rowIdx} row={row} onCellClicked={onCellClicked} />
                )
            }
        </div>
    );
}
