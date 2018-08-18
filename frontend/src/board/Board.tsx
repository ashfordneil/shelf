import * as React from "react";
import { interval } from "rxjs";

import { create } from "../util";

interface Props {
    id: string;
}

export const Board = (props: Props): Observable<React.JSXElement> {
    const { input, output } = create<React.JSXElement>(<h2>Loading</h2>);

    interval(1000).subscribe(
        () => input.next(<div className="board">
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
            </div>
        )
    );

    return output;
}
