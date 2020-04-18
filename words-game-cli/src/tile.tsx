import React, {SFC} from "react";
import { stylesheet } from "./style";

const WIDTH = 4;
const HEIGHT = 3;

interface Props {
    tile: string | number;
    row: number;
    column: number;
}

// function tileToContent(tile: string | number) {
//   if (typeof tile == 'number') {
//     return String(tile);
//   }

//   switch(tile) {
//     case
//   }
// }

export const Tile: SFC<Props> =({ tile, row, column }) => {
  return <blessed-box
            class={[stylesheet.bordered]}
            width={WIDTH}
            height={HEIGHT}
            top={HEIGHT * row}
            left={WIDTH*column}
            content={tile}/>
}
