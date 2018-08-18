import { Observable, BehaviorSubject, Subject } from "rxjs";

export interface Pipe<T> {
    input: Subject<T>;
    output: Observable<T>;
}

export const create = <T>(t: T): Pipe<T> => {
    const pipe = new BehaviorSubject<T>(t);
    const input = pipe;
    const output = pipe.asObservable();

    return { input, output }
}
