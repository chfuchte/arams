import { Component } from "solid-js";
import { useTheme } from "@/context/theme";
import { Button } from "@/components/ui/button";
import { DayNight } from "./icons";

export const ThemeToggle: Component<{ variant: "icon" | "text" }> = ({ variant }) => {
    const { toggleTheme } = useTheme();

    switch (variant) {
        case "text":
            return (
                <Button variant="ghost" on:click={toggleTheme} title="Switch theme">
                    Switch Theme
                </Button>
            );
        case "icon":
            return (
                <Button variant="ghost" size="icon" on:click={toggleTheme} title="Switch theme">
                    <DayNight class="fill-secondary-foreground hover:fill-accent-foreground size-5" />
                </Button>
            );
    }
};
