import { useEffect, useRef, useState } from 'react';
import BoardRow from '../BoardRow/BoardRow';
import './Board.css'

export default function Board(props) {
    const player1Symbol = 'X';
    const player2Symbol = 'O';
    const wasm = useRef(null);
    const game = useRef(null);
    const [cells, setCells] = useState([]);

    useEffect(() => {
        const loadWASM = async () => {
            wasm.current = await import('tic_tac_toe');
            game.current = wasm.current.Game.new(player1Symbol, player2Symbol);
            setCells(getCells());
        };
        loadWASM();
    }, [])

    const onCellClicked = (row, col) => {
        let game_state = game.current.get_game_state();
        if (!wasm.current.is_gave_over(game_state) && game.current.is_valid_move(row, col)) {
            game.current.set_cell_value(row, col, player1Symbol);
            let game_state = game.current.get_game_state();

            if (!wasm.current.is_gave_over(game_state))
            {
                let ai_move = wasm.current.get_best_move(game.current, player2Symbol, player1Symbol);
                if (game.current.is_valid_move(ai_move.row, ai_move.col)) {
                    game.current.set_cell_value(ai_move.row, ai_move.col, player2Symbol);
                    game_state = game.current.get_game_state();
                }
            }

            setCells(getCells());
            if (wasm.current.is_gave_over(game_state)) {
                props.onGameOver(wasm.current.game_over_message(game_state));
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
        <div id="board">
            {
                cells.flatMap((row, rowIdx) =>
                    <BoardRow key={rowIdx} rowIdx={rowIdx} row={row} onCellClicked={onCellClicked} /> 
                )
            }
        </div>
    );
}