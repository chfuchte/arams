import { hello_from_wasm } from "arams-wasm";
import { Component } from "solid-js";

export const WASMTest: Component = () => {
    return <h1>{hello_from_wasm()}</h1>;
};
