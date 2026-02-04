declare module 'zustand' {
  export type StoreApi<T> = {
    getState: () => T;
    setState: (partial: Partial<T> | ((state: T) => Partial<T>)) => void;
    subscribe: (listener: (state: T) => void) => () => void;
  };

  export type UseStore<T> = {
    (): T;
    <U>(selector: (state: T) => U): U;
  } & StoreApi<T>;

  export function create<T>(
    stateCreator: (
      set: (partial: Partial<T> | ((state: T) => Partial<T>)) => void,
      get: () => T
    ) => T
  ): UseStore<T>;

  export type StateCreator<T> = (
    set: (partial: Partial<T> | ((state: T) => Partial<T>)) => void,
    get: () => T
  ) => T;
}
