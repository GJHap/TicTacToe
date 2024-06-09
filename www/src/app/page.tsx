"use client";

import { useState } from "react";
import Board from "./components/Board";

export default function Home() {
    const [gameOverText, setGameOverText] = useState("");
    function onGameOver(gameOverText: string) {
        setGameOverText(gameOverText);
    }

    return (
        <div className="h-full flex flex-col items-center justify-center gap-[30px]">
            <label className="text-[30px] text-[red]">{gameOverText}</label>
            <Board onGameOver={onGameOver}></Board>
        </div>
    );
}
