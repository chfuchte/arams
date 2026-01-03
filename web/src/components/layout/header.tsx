import { Component } from "solid-js";
import { ThemeToggle } from "@/components/theme-toggle";
import { Spacer } from "@/components/ui/spacer";
import { Seperator } from "@/components/ui/seperator";

export const Header: Component = () => {
    return (
        <header class="bg-darker text-foreground flex flex-row items-center gap-4 px-4">
            <div class="inline-flex cursor-default flex-row items-center gap-2 select-none">
                <h1 class="text-sm font-bold">ARAMS</h1>
                <h2 class="text-muted-foreground text-sm">v0.1.0-alpha</h2>
            </div>
            <Seperator orientation="vertical" />
            <Spacer />
            <ThemeToggle variant="icon" />
        </header>
    );
};
