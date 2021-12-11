import Cell from '../Cell/Cell';
import './BoardRow.css';

export default function BoardRow(props) {
    return (
        <div className="board-row">
            {
                props.row.map((cell, colIdx) =>
                    <Cell
                        key={`${props.rowIdx},${colIdx}`}
                        value={cell}
                        onClick={() => props.onCellClicked(props.rowIdx, colIdx)} />
                )
            }   
        </div>
    );
}