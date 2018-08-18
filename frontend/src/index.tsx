import * as React from "react";
import * as ReactDOM from "react-dom";
import { Observable } from "rxjs";
import { map } from "rxjs/operators";

import { create } from "./util";

import { Board } from "./board/Board";

const App = () => {
return (
    <React.Fragment>
        <Board id="137fcc0c-6239-4ff9-a95a-da95475f5bc8" />
    </React.Fragment>)
}

ReactDOM.render(<App />, document.getElementById("main"));
