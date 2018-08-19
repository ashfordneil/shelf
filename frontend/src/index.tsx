import * as React from "react";
import * as ReactDOM from "react-dom";
import { Observable } from "rxjs";
import { map } from "rxjs/operators";

import { create } from "./util";

import { Board } from "./board/Board";

import * as board from "./board/services";
import * as tile from "./tile/services";

interface State {
    boards: string[];
    activeBoard: number | null;
}

class App extends React.Component<{}, State> {
    constructor(props: {}) {
        super(props);
        this.state = {
            boards: [],
            activeBoard: null,
        };
    }

    async componentDidMount() {
        let rawBoards = localStorage.getItem("boards");
        try {
            JSON.parse(rawBoards);
        } catch (error) {
            localStorage.clear();
            rawBoards = null;
        }
        const boards = rawBoards && JSON.parse(rawBoards);
        if (boards) {
            this.setState({ boards, activeBoard: 0 });
        } else {
            const id = await board.post("My first board");
            this.setState({ boards: [id], activeBoard: 0 });
            localStorage.setItem("boards", JSON.stringify([id]));
        }
    }

    render() {
        console.log(this.state);
        const { boards, activeBoard } = this.state;
        if (boards && activeBoard !== null) {
            return <Board id={boards[activeBoard]} />;
        } else {
            return <h2>Loading...</h2>;
        }
    }
}

ReactDOM.render(<App />, document.getElementById("main"));
