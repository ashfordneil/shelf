import * as React from "react";
import { Observable } from "rxjs";

import * as boardServices from "./services";
import * as tileServices from "../tile/services";

import * as boardModels from "./models";

import { create } from "../util";
import {Formik, Form, Field, FormikErrors} from 'formik';
import { Tile } from "../tile/models";
import { delete_ } from "../tile/services";

interface Props {
    id: string;
}

const handleDelete = (tile: Tile) => (): Promise<void> => {
    return delete_(tile.id)
}

enum Step {
    Loading,
    Done,
    Error,
}

interface State {
    step: Step;
    board: boardModels.Board | null;
}

export class Board extends React.Component<Props, State> {
    constructor(props: Props) {
        super(props);
        this.state = {
            step: Step.Loading,
            board: null,
        }
    }

    componentDidMount() {
        this.loadBoard();
    }

    loadBoard() {
        this.setState({step: Step.Loading});
        boardServices.get(this.props.id).then(board => {
            this.setState({step: Step.Done, board});
        }, err => {
            this.setState({step: Step.Error})
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
                return <React.Fragment>
                    <div className="header">
                        <h1>{board.title}</h1>
                    </div>
                    <div className="board">
                        {board.tiles.map(tile =>
                        <div key={tile.id} className="tile">
                            <h2>{tile.title}</h2>
                            <p>{tile.content}</p>
                            <p onClick={handleDelete(tile)}>X</p>
                        </div>)}
                        <Formik
                            initialValues={{
                                title: '',
                                content: '',
                            }}
                            validate={(values) => {
                                const errors: FormikErrors<typeof values> = {};
                                if (!values.title) {
                                    errors.title = "Title required";
                                }

                                if (!values.content) {
                                    errors.content = "content required";
                                }

                                return errors;
                            }}
                            onSubmit={(values, {setSubmitting, resetForm}) => {
                                tileServices.post(values).then(tileId => {
                                    boardServices.checkout(this.props.id).then(jwt => {
                                        let newArr = board.tiles.map(o => o.id);
                                        newArr.push(tileId);
                                        boardServices.checkin(this.props.id, jwt, {
                                            title: board.title,
                                            tiles: newArr as Tile[],
                                        })
                                        .then(() => this.loadBoard())
                                        .then(() => resetForm())
                                    })
                                });
                            }}
                            render={({values}) => 
                                <Form>
                                    <div id='newthing' className="tile">
                                        <h2>
                                            <Field 
                                                id="title"
                                                name="title"
                                            />
                                        </h2>
                                        <p>
                                            <Field 
                                                id="content"
                                                name="content"
                                            />
                                        </p>
                                        <button type="submit">Submit</button>
                                    </div>
                                </Form>
                        }
                        />
                    </div>
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


