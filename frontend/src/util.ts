import { Observable, BehaviorSubject, Subject } from "rxjs";

export interface Pipe<T> {
    input: Subject<T>;
    output: Observable<T>;
}

export const create = <T>(t: T | null): Pipe<T> => {
    const pipe = t === null
        ? new Subject<T>()
        : new BehaviorSubject<T>(t);
    const input = pipe;
    const output = pipe.asObservable();

    return { input, output }
}
