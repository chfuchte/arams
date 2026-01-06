import { Token as TokenType, TokenKind } from "@/lib/arams";
import { cva } from "class-variance-authority";
import { Component } from "solid-js";

export function tokenKindToVariant(
    kind: Exclude<TokenKind, "newline">,
): "unknown" | "instruction" | "argument" | "comment" | "error" | "label_definition" {
    switch (kind) {
        case "add":
            return "instruction";
        case "comment":
            return "comment";
        case "direct_address_argument":
            return "argument";
        case "div":
            return "instruction";
        case "end":
            return "instruction";
        case "goto":
            return "instruction";
        case "immediate_argument":
            return "argument";
        case "indirect_address_argument":
            return "argument";
        case "jnzero":
            return "instruction";
        case "jump_argument":
            return "argument";
        case "jzero":
            return "instruction";
        case "label_definition":
            return "label_definition";
        case "load":
            return "instruction";
        case "mul":
            return "instruction";
        case "store":
            return "instruction";
        case "sub":
            return "instruction";
        case "unknown":
            return "unknown";
    }
}

type TokenVariant = ReturnType<typeof tokenKindToVariant> | "error";

export const Pre: Component<{
    variant: TokenVariant;
    value: string | undefined;
}> = ({ variant, value }) => {
    const tokenVariants = cva("leading-5 font-medium", {
        variants: {
            variant: {
                unknown: "dark:text-neutral-200 text-neutral-800",
                instruction: "dark:text-neutral-200 text-neutral-800",
                argument: "text-green-500",
                comment: "text-muted-foreground",
                label_definition: "text-blue-500",
                error: "text-destructive",
            },
        },
        defaultVariants: {
            variant: "unknown",
        },
    });

    return <pre class={tokenVariants({ variant })}>{value}</pre>;
};

export const Line: Component<{ tokens: TokenType[]; errors: string[] }> = ({ tokens, errors }) => {
    return (
        <div class="flex space-x-2">
            {tokens.map((token) => {
                if (token.kind === "newline") return null;
                return <Pre variant={tokenKindToVariant(token.kind)} value={token.lexeme} />;
            })}
            {errors.length > 0 && <Pre value={`\t ${errors.join(", ")}`} variant="error" />}
        </div>
    );
};

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
