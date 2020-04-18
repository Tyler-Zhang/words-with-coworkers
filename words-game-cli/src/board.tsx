import React, { SFC } from "react";
import * as WordsGame from "words-game-wasm";
import { Tile } from "./tile";
import { stylesheet } from "./style";

interface Props {
  board: WordsGame.Board;
}

export const Board: SFC<Props> = ({ board }) => {
  const tiles = Array.from(board.cells).map((c, idx) => {
    const row = Math.ceil(idx / board.board_dimension);
    const col = idx % board.board_dimension;

    return <Tile column={col} row={row} tile={c}/>
  });

  return (
    <blessed-box
      label="Board"
      class={stylesheet.bordered}
      content={tiles}/>);
};
