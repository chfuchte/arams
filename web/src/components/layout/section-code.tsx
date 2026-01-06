import { Component, ComponentProps } from "solid-js";
import { CodeEditor } from "../editor";
import { aramsStore, setAramsStore } from "@/stores/arams";
import { cn } from "@/lib/utils";

export const CodeEditorSection: Component<ComponentProps<"section">> = ({ class: className, ...props }) => {
    return (
        <section class={cn("border-ring ring-ring/50 h-full rounded-md p-2 ring-2", className)} {...props}>
            <CodeEditor code={() => aramsStore.sourcecode} setCode={(c) => setAramsStore("sourcecode", c)} />
        </section>
    );
};
