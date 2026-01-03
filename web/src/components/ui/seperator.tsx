import { cn } from "@/lib/utils";
import { Component, ComponentProps } from "solid-js";

export const Seperator: Component<ComponentProps<"hr"> & { orientation: "horizontal" | "vertical" }> = ({
    orientation,
    ...props
}) => {
    return (
        <hr
            data-orientation={orientation}
            class={cn(
                "bg-border shrink-0 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px",
                props.class,
            )}
            aria-hidden="true"
            {...props}
        />
    );
};
