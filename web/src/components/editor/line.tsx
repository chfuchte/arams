import { Component } from "solid-js";
import { Token } from "./token";

export const Line: Component<{ code: string; errors: string[] }> = ({ code, errors }) => {
    return (
        <div class="flex space-x-2">
            <Token value={code} variant="plain" />
            {errors.length > 0 && <Token value={`\t ${errors.join(", ")}`} variant="error" />}
        </div>
    );
};
