import * as React from "react";
import * as ReactDOM from "react-dom";
import { Observable } from "rxjs";
import { map } from "rxjs/operators";

import { create } from "./util";

import { Board } from "./board/Board";

const App = () => {
return (
    <React.Fragment>
        <Board id="0cf72c16-aa39-4216-ac86-2c1e2a412851" />
        <div className="spacey" />
        <div className="footer">
            <h3>Footer</h3>
        </div>
    </React.Fragment>)
}

ReactDOM.render(<App />, document.getElementById("main"));