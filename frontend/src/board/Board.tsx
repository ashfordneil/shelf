import * as React from "react";
import { Observable } from "rxjs";
import ContentEditable from "react-contenteditable";

import * as boardServices from "./services";
import * as tileServices from "../tile/services";

import * as boardModels from "./models";

import { create } from "../util";
import { Tile } from "../tile/models";
import { delete_ } from "../tile/services";

interface Props {
    id: string;
}


enum Step {
    Loading,
    Done,
    Error,
}

interface State {
    step: Step;
    board: boardModels.Board | null;
    // null for not editing
    // string for editing (string = jwt)
    // 0 for new tile
    editingTile: null | string | 0;
    title: string;
    data: string;
    interval: NodeJS.Timer | null;
}

export class Board extends React.Component<Props, State> {
    constructor(props: Props) {
        super(props);
        this.state = {
            step: Step.Loading,
            board: null,
            editingTile: null,
            title: "title here...",
            data: "data here...",
        }
    }

    componentDidMount() {
        this.loadBoard();

        var interval = setInterval(
            () => {
                this.loadBoard();
            }
            , 1000);
        this.setState({ interval });
    }

    componentWillUnmount() {
        const { interval } = this.state;
        if (interval) {
            clearInterval(interval);
        }
        this.setState({ interval: null })
    }

    loadBoard() {
        // this.setState({step: Step.Loading});
        boardServices.get(this.props.id).then(board => {
            this.setState({step: Step.Done, board});
        }, err => {
            this.setState({step: Step.Error})
        });
    }

    handleDelete(tileId: string) {
        delete_(tileId).then(() => this.loadBoard());
    }

    newTile() {
        console.log("CREATING TILE");
        console.log(`TITLE: ${this.state.title}`);
        console.log(`DATA: ${this.state.data}`);
        tileServices.postForBoard({
            title: this.state.title,
            content: this.state.data,
        }, this.props.id)
        .then(() => this.loadBoard())
        .then(() => {
            this.setState({
                editingTile: null, 
                title: 'title here', 
                data: 'data here'
            })
        });
    }

    render() {
        // const {} = this.props;
        const {step, board} = this.state;

        switch (step) {
            case Step.Loading: {
                return <h2>Loading</h2>
            }
            case Step.Done: {
                const header = 
                    <div className="topColour">
                        <div className="header">
                            <h1>{board.title}</h1>
                            <div className="dropdown">
                                <button className="dropbtn">
                                    <i className="fas fa-caret-right"></i>
                                    Options
                                </button>
                                <div className="dropdown-content">
                                    <a href="#">Add Board</a>
                                    <a href="#">Delete Board</a>
                                    <a href="#">Other boards</a>
                                </div>
                            </div>
                        </div>
                    </div>;

                const footer =
                    <div className="footer">
                        <h2>SHELF</h2>
                        <div className="addButton" onClick={() => {
                            if (this.state.editingTile == null) {
                                this.setState({editingTile: 0})
                            }
                        }}>
                            <h2><i className="fas fa-plus"></i></h2>
                        </div>
                    </div>;

                const editing = this.state.editingTile === null
                    ? null
                    : <div className="tile new">
                            <h2>
                                <ContentEditable
                                    html={this.state.title}
                                    disabled={false}
                                    onChange={event => this.setState({ title: event.target.value })}
                                    tagName="span"
                                />
                                <div className="tileButton" onClick={() => this.setState({ editingTile: null })}>
                                    <i className="fas fa-times"></i>
                                </div>
                                <div className="tileButton" onClick={() => this.newTile()}>
                                    <i className="fas fa-save"></i>
                                </div>
                            </h2>
                            <ContentEditable
                                html={this.state.data}
                                disabled={false}
                                onChange={event => this.setState({ data: event.target.value })}
                                tagName="p"
                            />
                        </div>;
                
                const boardR = 
                    <div className="board">
                        {board.tiles.map(tile =>
                            <div key={tile.id} className="tile">
                                <h2>
                                    <span>
                                        {tile.title}
                                    </span>
                                    <div className="tileButton" onClick={() => this.handleDelete(tile.id)}>
                                        <i className="fas fa-trash"></i>
                                    </div>
                                </h2>
                                <p>{tile.content}</p>
                            </div>)
                        }
                        {editing}
                    </div>
                return <React.Fragment>
                    {header}
                    {boardR}
                    {footer}
                </React.Fragment>
            }
            case Step.Error: {
                return <div className="board">
                    <div className="tile">
                        <h2>Error</h2>
                        <p>sad dab</p>
                    </div>
                </div>
            }
            default: {
                return <div className="board">
                    <div className="tile">
                        <h2>what</h2>
                        <p>This shouldn't happen.</p>
                    </div>
                </div>
            }
        }

    }
}


