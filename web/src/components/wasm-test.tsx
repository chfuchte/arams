import { hello_from_wasm } from "@chfuchte/arams";
import { Component } from "solid-js";

export const WASMTest: Component = () => {
    return <h1>{hello_from_wasm()}</h1>;
};
