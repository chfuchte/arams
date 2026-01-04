import { check, run } from "@chfuchte/arams";
import { Component } from "solid-js";

export const WASMTest: Component = () => {
    const code = `load 1
            jzero return_one
            sub #1
            jzero return_one
            load 1
            store 2
loop:       load 1
            sub #1
            jzero break
            store 1
            mul 2
            store 2
            goto loop
return_one: load #1
            end
break:      load 2
            end`;

    const regs = new Map<Number, Number>();
    regs.set(1, 5);

    console.log(run(code, regs));

    return <h1>{check("load 1") ? "Success" : "Failure"}</h1>;
};
