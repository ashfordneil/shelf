import * as React from "react";
import { Observable, from } from "rxjs";
import { switchMap } from "rxjs/operators";

import { Board } from "./Board";
import { post } from "./services";

import { create } from "../util";

export const NewBoard = (): Observable<React.JSXElement> {
    const { input, output } = create<React.JSXElement>(<h2>Loading</h2>);

    (async () => {
        const id = await post("My First Board");
        const board = Board({ id });
        board.subscribe(x => input.next(x));
    })();

    return output;
}
