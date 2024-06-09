import { useEffect, useRef, useState } from "react";
import BoardRow from "./BoardRow";
import init, {
    Game,
    is_gave_over,
    get_best_move,
    game_over_message,
} from "../../../public/wasm/tic_tac_toe";

export default function ({
    onGameOver,
}: {
    onGameOver: (message: string) => void;
}) {
    const player1Symbol = "X";
    const player2Symbol = "O";
    const gameRef = useRef<Game>();
    const [cells, setCells] = useState<string[][]>([]);

    useEffect(() => {
        init("wasm/tic_tac_toe_bg.wasm").then(() => {
            gameRef.current = new Game(player1Symbol, player2Symbol);
            setCells(getCells());
        });
    }, []);

    const onCellClicked = (row: number, col: number) => {
        if (!gameRef.current) return;

        const game = gameRef.current;

        let game_state = game.get_game_state();
        if (!is_gave_over(game_state) && game.is_valid_move(row, col)) {
            game.set_cell_value(row, col, player1Symbol);
            let game_state = game.get_game_state();

            if (!is_gave_over(game_state)) {
                let ai_move = get_best_move(game, player2Symbol, player1Symbol);
                if (game.is_valid_move(ai_move.row, ai_move.col)) {
                    game.set_cell_value(
                        ai_move.row,
                        ai_move.col,
                        player2Symbol,
                    );
                    game_state = game.get_game_state();
                }
            }

            setCells(getCells());
            if (is_gave_over(game_state)) {
                onGameOver(game_over_message(game_state));
            }
        }
    };

    const getCells = () => {
        if (!gameRef.current) return [];

        const game = gameRef.current;

        return [...Array(game.dimensions()).keys()].map((row) =>
            [...Array(game.dimensions()).keys()].map((col) =>
                game.get_cell_value(row, col),
            ),
        );
    };

    return (
        <div className="flex flex-col w-[40vmin] h-[40vmin]">
            {cells.flatMap((row, rowIdx) => (
                <BoardRow
                    key={rowIdx}
                    rowIdx={rowIdx}
                    row={row}
                    onCellClicked={onCellClicked}
                />
            ))}
        </div>
    );
}
