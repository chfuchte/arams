import type { Component } from "solid-js";
import { ThemeProvider } from "./context/theme";
import { Header } from "./components/layout/header";
import { Footer } from "./components/layout/footer";
import { WASMTest } from "./components/wasm-test";

const App: Component = () => {
    return (
        <ThemeProvider>
            <Header />

            <main class="p-6">
                <WASMTest />
            </main>

            <Footer />
        </ThemeProvider>
    );
};

export default App;
