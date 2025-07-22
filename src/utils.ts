import { createSignal, Accessor, Setter } from 'solid-js';

export type SignalObject<T> = {
    get: Accessor<T>;
    set: Setter<T>;
};

export function createSignalObject<T>(v: T) {
    const [get, set] = createSignal(v);

    return { get, set };
}
