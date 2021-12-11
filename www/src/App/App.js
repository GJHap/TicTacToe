import { useState } from 'react';
import Board from '../Board/Board';
import './App.css'

export default function App() {
  const [gameOverText, setGameOverText] = useState('');
  function onGameOver(gameOverText) {
    setGameOverText(gameOverText);
  }

  return (
    <div id="App">
      <label className="gameOver">{gameOverText}</label>
      <Board onGameOver={onGameOver}></Board>
    </div>
  );
}