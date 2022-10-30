import { useState } from 'react';
import Board from '../Board/Board';
import styles from './App.module.css'

export default function () {
    const [gameOverText, setGameOverText] = useState('');
    function onGameOver(gameOverText) {
        setGameOverText(gameOverText);
    }

    return (
        <div id={styles.App}>
            <label className={styles.gameOver}>{gameOverText}</label>
            <Board onGameOver={onGameOver}></Board>
        </div>
    );
}