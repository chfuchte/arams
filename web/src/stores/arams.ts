import { createStore } from "solid-js/store";

const [aramsStore, setAramsStore] = createStore<{
    machine:
        | {
              accumulator: number;
              registers: Map<number, number>;
          }
        | undefined;
    sourcecode: string;
    preseedRegisters?: Map<number, number>;
}>({
    machine: undefined,
    preseedRegisters: new Map([
        [1, 0],
        [2, 0],
        [3, 0],
        [4, 0],
        [5, 0],
        [6, 0],
        [7, 0],
        [8, 0],
        [9, 0],
        [10, 0],
    ]),
    sourcecode:
        "\t\t\tload 1\
\n\t\t\tjzero return_one\n\
\t\t\tsub #1\n\
\t\t\tjzero return_one\n\
\t\t\tload 1\n\
\t\t\tstore 2\n\
loop:\t\tload 1\n\
\t\t\tsub #1\n\
\t\t\tjzero break\n\
\t\t\tstore 1\n\
\t\t\tmul 2\n\
\t\t\tstore 2\n\
\t\t\tgoto loop\n\
return_one:\tload #1\n\
\t\t\tend\n\
break:\t\tload 2\n\
\t\t\tend",
});

export { aramsStore, setAramsStore };
