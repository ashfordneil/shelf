import * as React from "react";
import { Observable } from "rxjs";
import ContentEditable from "react-contenteditable";

import * as boardServices from "./services";
import * as tileServices from "../tile/services";

import * as boardModels from "./models";

import { create } from "../util";
import { Tile } from "../tile/models";
import { delete_ } from "../tile/services";

import jwtDecode from 'jwt-decode';

interface Props {
    id: string;
    otherBoards: string[];
    newBoard: () => void;
    changeBoard: (id: string) => void;
}

enum Step {
    Loading,
    Done,
    Error,
}

interface JWTThing {
    key: [boolean, string],
    ttl: number
}

const readJWT = (input: string): JWTThing => {
    const decoded = jwtDecode(input);
    decoded.key = JSON.parse(decoded.key);
    return decoded;
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
    interval: number | null;

    boardLocked: string | null;
    boardTitle: null | string;

    // tracking for lock icons on tiles that are being edited
    locks: {[key: string]: number};

    // [[id, name], [id, name], ...]
    otherBoardNames: [string, string][];
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
            interval: null,
            locks: {},
            otherBoardNames: [],
            boardLocked: null,
        }
    }

    async componentDidMount() {
        this.loadBoard();

        var interval = setInterval(
            () => {
                if (this.state.boardLocked == null) {
                    this.loadBoard();
                }
            }
            , 1000);
        this.setState({ interval });

        const boards = await Promise.all(
            this.props.otherBoards.map(board => boardServices.get(board))
        );
        const withNames = this.props.otherBoards.map((id, index) =>
            [id, boards[index].title] as [string, string]
        );
        this.setState({ otherBoardNames: withNames });
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
            this.setState({step: Step.Done, board, boardTitle: board.title});
        }, err => {
            this.setState({step: Step.Error})
        });
    }

    async lockTile(tile: string) {
        const timer = setTimeout(() => {
            this.setState(({ locks }) => {
                delete locks[tile];
                return { locks };
            });
        }, 150);

        this.setState(({ locks }) => {
            if (tile in locks) {
                clearTimeout(locks[tile]);
            }
            locks[tile] = timer;
            return { locks };
        });
    }

    async handleDelete(tileId: string) {
        try {
            await delete_(tileId);
            this.setState(({ board }) => {
                const tiles = board.tiles.filter(x => x.id !== tileId);
                return { board: { ...board, tiles } };
            });
        } catch (error) {
            this.lockTile(tileId);
        }
    }

    newTile() {
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

    stopEditing() {
        const {editingTile} = this.state;
        if (editingTile !== null && editingTile !== 0) {
            let thingo = readJWT((editingTile)) as JWTThing;
            const id = thingo.key[1];
            tileServices.undocheckout(id, editingTile);
        }
        this.setState({editingTile: null});
    }

    async checkout(tile: Tile) {
        if (this.state.editingTile !== null) {
            return;
        }
        try {
            const jwt = await tileServices.checkout(tile.id);
            this.setState({
                editingTile: jwt,
                title: tile.title,
                data: tile.content
            });
        } catch(error) {
            this.lockTile(tile.id);
        }
    }

    submitChanges() {
        const {editingTile} = this.state;
        if (editingTile == 0) {
            this.newTile();
        }
        else {
            let thingo = readJWT((editingTile));
            const id = thingo.key[1];
            tileServices.checkin(id, editingTile, {
                title: this.state.title,
                content: this.state.data
            } as Tile)
            .then(() => {
                this.setState({
                    editingTile: null, 
                    title: 'title here', 
                    data: 'data here'
                });
                this.loadBoard();
            });
        }
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
                            <ContentEditable
                                    html={this.state.boardTitle}
                                    disabled={false}
                                    onChange={async event => {
                                        console.log("aaaa");
                                        if (this.state.boardLocked == null) {
                                            console.log("fff");
                                            const jwt = await boardServices.checkout(this.props.id);
                                            this.setState({boardLocked: jwt});
                                        }
                                        // let boardThing = JSON.parse(JSON.stringify(board));
                                        // boardThing.title = event.target.value;
                                        // this.setState({board: boardThing});
                                        this.setState({boardTitle: event.target.value});
                                    }}
                                    onBlur={async event => {
                                        console.log("bbbb");
                                        let otherBoard = JSON.parse(JSON.stringify(board));
                                        otherBoard.title = this.state.boardTitle;
                                        otherBoard.tiles = otherBoard.tiles.map(o => o.id);
                                        await boardServices.checkin(this.props.id, this.state.boardLocked, otherBoard);
                                        this.setState({boardLocked: null});

                                    }}
                                    tagName="h1"
                            />
                            <div className="dropdown">
                                <button className="dropbtn">
                                    <i className="fas fa-caret-right"></i>
                                    My Boards
                                </button>
                                <div className="dropdown-content">
                                    <a href="#" onClick={() => this.props.newBoard()}>Add Board</a>
                                    {this.state.otherBoardNames.map(([id, name]) => {
                                        return <a
                                            key={id}
                                            href={`/share/${id}`}
                                        >{name}</a>;
                                    })}
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
                                <div className="tileButton" onClick={() => this.stopEditing()}>
                                    <i className="fas fa-times"></i>
                                </div>
                                <div className="tileButton" onClick={() => this.submitChanges()}>
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
                        {board.tiles
                        .filter(t => {
                            if (this.state.editingTile == null || this.state.editingTile == 0) {
                                return true;
                            }
                            const decoded = readJWT((this.state.editingTile));
                            return decoded.key[1] != t.id
                        })
                        .map(tile =>
                            <div
                                key={tile.id}
                                className="tile"
                                onClick={() => {
                                    this.checkout(tile)
                                }}
                            >
                                <h2>
                                    <ContentEditable
                                        html={tile.title}
                                        disabled={true}
                                        tagName="span"
                                    />
                                    <div className="tileButton lock" data-active={tile.id in this.state.locks ? "on" : "off"}>
                                        <i className="fas fa-lock"></i>
                                    </div>
                                    <div className="tileButton" onClick={() => this.handleDelete(tile.id)}>
                                        <i className="fas fa-trash"></i>
                                    </div>
                                </h2>
                                <ContentEditable
                                    html={tile.content}
                                    disabled={true}
                                    tagName="p"
                                />
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


