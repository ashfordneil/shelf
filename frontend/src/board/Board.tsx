import * as React from "react";
import { interval } from "rxjs";

import { get } from "./services";

import { create } from "../util";

interface Props {
    id: string;
}

export const Board = (props: Props): Observable<React.JSXElement> {
    const { input, output } = create<React.JSXElement>(<h2>Loading</h2>);

    (async () => {
        try {
            const board = await get(props.id);
            const tiles = board.tiles.map(tile =>
                <div id={tile.id} className="tile">
                    <h2>Title</h2>
                    <p>{tile.content}</p>
                </div>
            );
            input.next(
                <div className="board">
                    {tiles}
                </div>
            );
        } catch (error) {
            input.next(
                <div className="board">
                    <div className="tile">
                        <h2>Error</h2>
                        <p>{error.message}</p>
                    </div>
                </div>
            );
        }
    })();

    return output;
}
