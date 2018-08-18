import * as React from "react";
import * as ReactDOM from "react-dom";
import { Observable } from "rxjs";
import { map } from "rxjs/operators";

import { create } from "./util";

import { NewBoard } from "./board/NewBoard";
import { Board } from "./board/Board";
import * as tile from "./tile/services";

const App = (): Observable<React.JSXElement> {
    const board = NewBoard();

    const stream = board;

    return stream;
}

const main = () => {
    const app = App();
    const hook = document.getElementById("main");
    app.subscribe(elem => ReactDOM.render(elem, hook));
}

main();
