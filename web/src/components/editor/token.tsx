import { cva } from "class-variance-authority";
import { Component } from "solid-js";

export const Token: Component<{
    variant: "plain" | "comment" | "error";
    value: string | undefined;
}> = ({ variant, value }) => {
    const tokenVariants = cva("leading-5 font-medium", {
        variants: {
            variant: {
                plain: "dark:text-neutral-200 text-neutral-800  ",
                comment: "text-muted-foreground",
                error: "text-destructive",
            },
        },
        defaultVariants: {
            variant: "plain",
        },
    });

    return <pre class={tokenVariants({ variant })}>{value}</pre>;
};
