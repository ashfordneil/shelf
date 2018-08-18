import * as React from "react";
import { Observable, from } from "rxjs";
import { switchMap } from "rxjs/operators";

import { Board } from "./Board";
import { checkout, checkin, post } from "./services";
import * as tile from "../tile/services";

import { create } from "../util";

export const NewBoard = (): Observable<React.JSXElement> {
    const { input, output } = create<React.JSXElement>(<h2>Loading</h2>);

    (async () => {
        const id = await post("My First Board");
        const auth = await checkout(id);
        const tileId = await tile.post({
            title: "Tile #1",
            content: "Put your text here"
        });
        await checkin(id, auth, { title: "My First Board", tiles: [tileId] });

        const board = new Board({ id });
        board.subscribe(x => input.next(x));
    })();

    return output;
}
