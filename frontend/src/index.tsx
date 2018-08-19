import * as React from "react";
import * as ReactDOM from "react-dom";
import { Observable } from "rxjs";
import { map } from "rxjs/operators";

import { create } from "./util";

import { Board } from "./board/Board";

import * as board from "./board/services";
import * as tileServices from "./tile/services";

interface Props {
    url: string;
}

interface State {
    boards: string[];
    activeBoard: number | null;
}

class App extends React.Component<Props, State> {
    constructor(props: Props) {
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
            if (this.props.url) {
                const index = boards.indexOf(this.props.url);
                if (index === -1) {
                    boards.push(this.props.url);
                    this.setState({ boards, activeBoard: boards.length - 1 });
                    localStorage.setItem("boards", JSON.stringify(boards));
                } else {
                    this.setState({ boards, activeBoard: index });
                }
            } else {
                this.setState({ boards, activeBoard: 0 });
            }
        } else {
            if (this.props.url) {
                const boards = [this.props.url];
                this.setState({ boards, activeBoard: 0 });
                localStorage.setItem("boards", JSON.stringify(boards));
            } else {
                await this.newBoard();
            }
        }
    }

    async newBoard() {
        const id = await board.post("My first board");
        const tile = await tileServices.post({
            title: "My first tile",
            content: "This is an example tile. Click the + button to add a new tile."
        });

        await board.cheekyupdate(id, { title: "My first board", tiles: [tile] });

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

const parseUrl = (url: string) => {
    const test = /.*\/share\/(.*)/.exec(url);
    if (test) {
        return test[1];
    } else {
        return null;
    }
}

ReactDOM.render(<App url={parseUrl(window.location)}/>, document.getElementById("main"));
