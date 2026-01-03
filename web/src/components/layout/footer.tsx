import { Component } from "solid-js";
import { GitHub } from "@/components/icons";
import { ThemeToggle } from "@/components/theme-toggle";

export const Footer: Component = () => {
    return (
        <footer class="bg-darker text-foreground flex flex-row items-center justify-between px-6 py-2 text-sm max-sm:flex-col-reverse">
            <span>&copy; {new Date().getFullYear()} Christian Fuchte</span>
            <ThemeToggle variant="text" />
            <a
                class="flex flex-row items-center gap-1 hover:underline"
                href="https://github.com/chfuchte/arams"
                target="_blank"
                rel="noopener noreferrer">
                <GitHub class="size-5" />
                View on Github
            </a>
        </footer>
    );
};
