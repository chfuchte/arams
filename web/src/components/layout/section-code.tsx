import { Component, ComponentProps, createEffect, createSignal } from "solid-js";
import { CodeEditor } from "../editor";
import { aramsStore, setAramsStore } from "@/stores/arams";
import { check } from "@chfuchte/arams";
import { cn } from "@/lib/utils";

export const CodeEditorSection: Component<ComponentProps<"section">> = ({ class: className, ...props }) => {
    const [errors, setErrors] = createSignal<Record<number, string[]> | undefined>(undefined);

    createEffect(() => {
        const checkCodeResult = check(aramsStore.sourcecode);

        if (checkCodeResult === true) {
            setErrors(undefined);
            return;
        }

        const errorsByLine: Record<number, string[]> = checkCodeResult.reduce(
            (acc, curr) => {
                if (!acc[curr.line]) {
                    acc[curr.line] = [];
                }
                acc[curr.line].push(curr.message);
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
