/* tslint:disable */
/* eslint-disable */

import type * as wasm from "./dist/arams_wasm.d.ts";

export type SourceCode = string | string[];

export type Machine = {
    accumulator: number;
    registers: Map<number, number>;
};

export type CompilationError = {
    line: number;
    message: string;
};

export type ExecutionError = {
    message: string;
};

export type AnalyzeToken = {
    kind: string;
    lexeme: string;
    errors: string[];
    about: string;
};

export declare function run(
    input: SourceCode,
    registers: Map<number, number>,
):
    | { status: "ok"; machine: Machine }
    | { status: "compilationerror"; compilation_error: CompilationError[] }
    | { status: "executionerror"; execution_error: ExecutionError };

export declare function analyze(input: SourceCode): AnalyzeToken[][];

export { wasm };
