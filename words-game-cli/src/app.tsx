import React, { SFC } from "react";
import blessed from "blessed";
import { render } from "react-blessed";
import { Board } from "./board";
import * as WordsGame from "words-game-wasm";

const App: SFC = () => {
    const game = new WordsGame.Game(1);

    return (
        <blessed-box
            width="2"
            height="1"
            border={{type: 'line'}}
            style={{border: {fg: 'blue'}}}>
                <Board board={game.board}/>
        </blessed-box>
    );
}

const screen = blessed.screen({
    autoPadding: true,
    smartCSR: true,
    title: 'Words Game'
});

screen.key('C-c', () => process.exit(0));

render(<App/>, screen);
