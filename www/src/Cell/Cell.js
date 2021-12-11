import './Cell.css'

export default function Cell(props) {
    return (
        <button className="board-cell" onClick={props.onClick}>
            {props.value}
        </button>
    );
}
