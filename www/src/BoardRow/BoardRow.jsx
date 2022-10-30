import Cell from '../Cell/Cell';
import styles from './BoardRow.module.css'

export default function ({ row, rowIdx, onCellClicked }) {
    return (
        <div className={styles['board-row']}>
            {
                row.map((cell, colIdx) =>
                    <Cell
                        key={`${rowIdx},${colIdx}`}
                        value={cell}
                        onClick={() => onCellClicked(rowIdx, colIdx)} />
                )
            }
        </div>
    );
}