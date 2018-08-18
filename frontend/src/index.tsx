import * as React from "react";
import * as ReactDOM from "react-dom";
import { Observable } from "rxjs";
import { from } from "rxjs";

import { create } from "./util";

import * as tile from "./tile/services";

const App = (): Observable<React.JSXElement> {
    const title = <h1>Title of Board</h1>;
    const board = <div className="board">
        <div className="tile">
            <h2>Title</h2>
            <p>Hello</p>
        </div>
        <div className="tile">
            <h2>Title</h2>
            <p>World</p>
        </div>
        <div className="tile">
            <h2>Title</h2>
            <p>World</p>
        </div>
        <div className="tile">
            <h2>Title</h2>
            <p>World</p>
        </div>
        <div className="tile">
            <h2>Title</h2>
            <p>World</p>
        </div>
        <div className="tile">
            <h2>Title</h2>
            <p>World</p>
        </div>
    </div>;

    const first = <React.Fragment>
        {title}
        {board}
    </React.Fragment>;

    return from([first]);
}

const main = () => {
    const app = App();
    const hook = document.getElementById("main");
    app.subscribe(elem => ReactDOM.render(elem, hook));
}

main();
