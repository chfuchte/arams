import type { Component } from "solid-js";
import { ThemeProvider } from "./context/theme";
import { Header } from "./components/layout/header";
import { Footer } from "./components/layout/footer";
import { CodeEditorSection } from "./components/layout/section-code";
import { MachineSection } from "./components/layout/section-machine";

const App: Component = () => {
    return (
        <ThemeProvider>
            <Header />

            <main class="grid grid-cols-1 gap-6 p-6 md:grid-cols-3 xl:grid-cols-5">
                <CodeEditorSection class="col-span-1 md:col-span-2 xl:col-span-4" />
                <MachineSection class="col-span-1" />
            </main>

            <Footer />
        </ThemeProvider>
    );
};

export default App;
