import * as React from "react";
import { Observable } from "rxjs";

import * as boardServices from "./services";
import * as tileServices from "../tile/services";

import { create } from "../util";
import {Formik, Form, Field, FormikErrors} from 'formik';

interface Props {
    id: string;
}

export const Board = (props: Props): Observable<React.JSXElement> {
    const { input, output } = create<React.JSXElement>(<h2>Loading</h2>);

    (async () => {
        try {
            const board = await boardServices.get(props.id);
            const tiles = board.tiles.map(tile =>
                <div id={tile.id} className="tile">
                    <h2>{tile.title}</h2>
                    <p>{tile.content}</p>
                </div>
            );
            input.next(
                <React.Fragment>
                    <h1>{board.title}</h1>
                    <div className="board">
                        {tiles}
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
                                    boardServices.checkout(props.id).then(jwt => {
                                        let newArr = board.tiles.map(o => o.id);
                                        newArr.push(tileId);
                                        boardServices.checkin(props.id, jwt, {
                                            title: board.title,
                                            tiles: newArr as Tile[],
                                        }).then(() => resetForm())
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
            );
        } catch (error) {
            input.next(
                <div className="board">
                    <div className="tile">
                        <h2>Error</h2>
                        <p>{error.message}</p>
                    </div>
                </div>
            );
        }
    })();

    return output;
}
