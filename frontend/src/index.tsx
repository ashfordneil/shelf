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

    const stream = board.pipe(
        map(board =>
            <React.Fragment>
                {board}
                <div className="spacey" />
                <div className="footer">
                    <h3>Footer</h3>
                </div>
            </React.Fragment>
        )
    );

    return stream;
}

const main = () => {
    const app = App();
    const hook = document.getElementById("main");
    app.subscribe(elem => ReactDOM.render(elem, hook));
}

main();
