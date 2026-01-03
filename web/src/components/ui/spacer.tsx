import { cn } from "@/lib/utils";
import { Component, ComponentProps } from "solid-js";

export const Spacer: Component = (props: ComponentProps<"hr">) => {
    return <hr class={cn("invisible w-full flex-1", props.class)} aria-hidden="true" {...props} />;
};
