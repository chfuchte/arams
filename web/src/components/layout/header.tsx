import { Component } from "solid-js";
import { ThemeToggle } from "@/components/theme-toggle";
import { Spacer } from "@/components/ui/spacer";
import { Seperator } from "@/components/ui/seperator";
import { Button } from "../ui/button";
import { Reset, Run } from "../icons";
import { run } from "@chfuchte/arams";
import { aramsStore, setAramsStore } from "@/stores/arams";

export const Header: Component = () => {
    const runCode = () => {
        try {
            const result = run(aramsStore.sourcecode, aramsStore.preseedRegisters || new Map());
            if (Array.isArray(result)) {
                const errorMessages = result.map((err) => `Line ${err.line + 1}: ${err.message}`).join("\n");
                alert(`Errors compiling the code:\n${errorMessages}`);
            } else {
                setAramsStore("machine", {
                    accumulator: result.accumulator,
                    registers: result.registers,
                });
            }
        } catch (e) {
            alert(`Error running the code: ${(e as Error).message}`);
        }
    };

    const resetRegisters = () => {
        setAramsStore("machine", undefined);
        setAramsStore("preseedRegisters", new Map());
    };

    return (
        <header class="bg-darker text-foreground flex flex-row items-center gap-4 px-4">
            <div class="inline-flex cursor-default flex-row items-center gap-2 select-none">
                <h1 class="text-sm font-bold">ARAMS</h1>
                <h2 class="text-muted-foreground text-sm">v0.1.0</h2>
            </div>
            <Seperator orientation="vertical" />
            <Button onClick={runCode} size="sm" variant="success">
                <Run class="size-5 fill-green-600" /> Run
            </Button>
            <Button onClick={resetRegisters} size="sm" variant="ghost">
                <Reset class="size-5 fill-white" /> Reset
            </Button>
            <Spacer />
            <ThemeToggle variant="icon" />
        </header>
    );
};
