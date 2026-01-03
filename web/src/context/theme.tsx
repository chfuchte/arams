import { createEffect, createContext, createSignal, JSX, useContext } from "solid-js";

type Theme = "light" | "dark";

type ThemeContextType = {
    setTheme: (theme: Theme) => void;
    toggleTheme: () => void;
};

const THEME_KEY = "theme";

const getInitialTheme = (): Theme => {
    const stored = typeof localStorage !== "undefined" ? localStorage.getItem(THEME_KEY) : null;
    if (stored === "light" || stored === "dark") return stored;
    if (
        typeof window !== "undefined" &&
        window.matchMedia &&
        window.matchMedia("(prefers-color-scheme: dark)").matches
    ) {
        return "dark";
    }
    return "light";
};

const ThemeContext = createContext<ThemeContextType>();

export const ThemeProvider = (props: { children: JSX.Element }) => {
    const [theme, setTheme] = createSignal<Theme>(getInitialTheme());
    const htmlRef = typeof document !== "undefined" ? document.documentElement : null;

    const toggleTheme = () => {
        setTheme((t) => (t === "light" ? "dark" : "light"));
    };

    createEffect(() => {
        const t = theme();
        if (htmlRef) {
            if (t === "dark") htmlRef.classList.add("dark");
            else htmlRef.classList.remove("dark");
        }
        if (typeof localStorage !== "undefined") localStorage.setItem(THEME_KEY, t);
    });

    const context: ThemeContextType = {
        setTheme,
        toggleTheme,
    };

    return <ThemeContext.Provider value={context}>{props.children}</ThemeContext.Provider>;
};

export const useTheme = () => {
    const ctx = useContext(ThemeContext);
    if (!ctx) {
        throw new Error("useTheme must be used within a ThemeProvider");
    }
    return ctx;
};
