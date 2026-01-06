import { Component, ComponentProps, createEffect, createSignal } from "solid-js";
import { CodeEditor } from "../editor";
import { aramsStore, setAramsStore } from "@/stores/arams";
import { cn } from "@/lib/utils";
import { intoTokens } from "@/lib/arams";

export const CodeEditorSection: Component<ComponentProps<"section">> = ({ class: className, ...props }) => {
    const [errors, setErrors] = createSignal<Record<number, string[]> | undefined>(undefined);

    createEffect(() => {
        const lines = intoTokens(aramsStore.sourcecode);

        const errorsByLine: Record<number, string[]> = lines.reduce(
            (acc, line, lineNumber) => {
                if (line.errors.length > 0) {
                    acc[lineNumber + 1] = line.errors;
                }
                return acc;
            },
            {} as Record<number, string[]>,
        );

        setErrors(errorsByLine);
    }, [aramsStore.sourcecode]);

    return (
        <section class={cn("border-ring ring-ring/50 h-full rounded-md p-2 ring-2", className)} {...props}>
            <CodeEditor
                code={() => aramsStore.sourcecode}
                setCode={(c) => setAramsStore("sourcecode", c)}
                errors={errors}
            />
        </section>
    );
};
