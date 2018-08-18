import * as React from "react";
import * as ReactDOM from "react-dom";
import { Observable } from "rxjs";
import { map } from "rxjs/operators";

import { create } from "./util";

import { Board } from "./board/Board";
import * as tile from "./tile/services";

const App = (): Observable<React.JSXElement> {
    const title = <h1>Title of Board</h1>;
    const board = Board({ id: "FIXME" });

    const stream = board.pipe(
        map(board => 
            <React.Fragment>
                {title}
                {board}
            </React.Fragment>
        ),
    );

    return stream;
}

const main = () => {
    const app = App();
    const hook = document.getElementById("main");
    app.subscribe(elem => ReactDOM.render(elem, hook));
}

main();
