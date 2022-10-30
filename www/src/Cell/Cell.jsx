import styles from './Cell.module.css'

export default function ({ onClick, value }) {
    return (
        <button className={styles['board-cell']} onClick={onClick}>
            {value}
        </button>
    );
}