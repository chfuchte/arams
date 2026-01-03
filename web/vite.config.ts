import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import devtools from "solid-devtools/vite";
import path from "node:path";

export default defineConfig({
    plugins: [devtools(), solidPlugin(), tailwindcss()],
    base: process.env.VITE_BASE_PATH || "/",
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./src"),
        },
    },
    server: {
        port: 3000,
    },
    build: {
        target: "esnext",
    },
});
