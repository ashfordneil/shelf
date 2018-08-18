import * as React from "react";
import { Observable } from "rxjs";

import { get } from "./services";

import { create } from "../util";
import { Tile } from "../tile/models";
import { delete_ } from "../tile/services";

interface Props {
    id: string;
}

const handleDelete = (tile: Tile) => (): Promise<void> => {
    return delete_(tile.id)
}

export const Board = (props: Props): Observable<React.JSXElement> {
    const { input, output } = create<React.JSXElement>(<h2>Loading</h2>);

    (async () => {
        try {
            const board = await get(props.id);
            const tiles = board.tiles.map(tile =>
                <div key={tile.id} className="tile">
                    <h2>{tile.title}</h2>
                    <p>{tile.content}</p>
                    <p onClick={handleDelete(tile)}>X</p>
                </div>
            );
            input.next(
                <React.Fragment>
                    <div className="header">
                        <h1>{board.title}</h1>
                    </div>
                    <div className="board">
                        {tiles}
                    </div>
                </React.Fragment>
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
