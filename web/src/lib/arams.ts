import { analyze } from "@chfuchte/arams";

export type Token = {
    kind: TokenKind;
    lexeme: string;
    about: string;
};

export type TokenKind =
    | "load"
    | "store"
    | "add"
    | "sub"
    | "mul"
    | "div"
    | "goto"
    | "jzero"
    | "jnzero"
    | "end"
    | "label_definition"
    | "jump_argument"
    | "immediate_argument"
    | "indirect_address_argument"
    | "direct_address_argument"
    | "comment"
    | "newline"
    | "unknown";

export function intoTokens(input: string): Array<{ tokens: Token[]; errors: string[] }> {
    const lines = analyze(input);

    console.log(lines);

    return lines.map((line) => {
        const lineErrors: string[] = [];
        const tokens = line.map((t) => {
            lineErrors.push(...t.errors);
            return {
                kind: t.kind as TokenKind,
                lexeme: t.lexeme,
                about: t.about,
            };
        });
        return { tokens, errors: lineErrors };
    });
}
