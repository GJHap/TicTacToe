import React from 'react';
import Cell from '../Cell/Cell'
import './Board.css'

export default class Board extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            cells: [
                [' ', ' ', ' '],
                [' ', ' ', ' '],
                [' ', ' ', ' ']
            ]
        };

        this.onCellClicked = this.onCellClicked.bind(this);
    }

    onCellClicked(row, col) {
        this.setState(
            {
                cells: this.state.cells.map((cellRow, rowIdx) => {
                    return cellRow.map((cell, colIdx) => {
                        if (row === rowIdx && col === colIdx) {
                            return 'X';
                        }
                        return cell;
                    });
                })
            }
        )
    }

    render() {
        return <div id="board">
            {
                this.state.cells.flatMap((row, rowIdx) =>
                    <div key={rowIdx.toString()} className="cell-row">
                        {
                            row.map((cell, colIdx) => 
                                <Cell key={rowIdx.toString() + "," + colIdx.toString()} value={cell} onClick={() => this.onCellClicked(rowIdx, colIdx)} />
                        )}
                    </div>
                
            )}
        </div>
    }
}