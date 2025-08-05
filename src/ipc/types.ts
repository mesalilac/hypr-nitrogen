export * from './bindings';

export type Mode = 'default' | 'contain' | 'tile';

export type Error = string;

export type Response<T> = {
    data: T;
};
