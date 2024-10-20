import type { Component } from "solid-js";
import { SockerProvider } from "./providers/websocker";
import { toast } from "solid-sonner";
import { Toaster } from "./components/toast";

const App: Component = () => {
    const handleClick = () => {
        toast("Hello Toast");
    };

    return (
        <SockerProvider>
            <Toaster richColors closeButton />
            <button onClick={handleClick}>Show Toast</button>
        </SockerProvider>
    );
};

export default App;
