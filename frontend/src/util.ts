import { Observable, Subject } from "rxjs";

export interface Pipe<T> {
    input: Subject<T>;
    output: Observable<T>;
}

export const create = <T>(): Pipe<T> => {
    const pipe = new Subject<T>();
    const input = pipe;
    const output = pipe.asObservable();

    return { input, output }
}
