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
            await newBoard();
        }
    }

    async newBoard() {
        const id = await board.post("My first board");
        const { boards } = this.state;
        this.setState({ boards: [...boards, id], activeBoard: boards.length });
        localStorage.setItem("boards", JSON.stringify([...boards, id]));
    }

    render() {
        const { boards, activeBoard } = this.state;
        if (boards && activeBoard !== null) {
            return <Board
                        id={boards[activeBoard]}
                        otherBoards={boards}
                        newBoard={() => this.newBoard()}
                        changeBoard={(id: string) => {
                            const index = this.state.boards.indexOf(id);
                            if (index >= 0) {
                                this.setState({ activeBoard: index });
                            }
                        }}
                />;
        } else {
            return <h2>Loading...</h2>;
        }
    }
}

ReactDOM.render(<App />, document.getElementById("main"));
