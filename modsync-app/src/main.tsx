import ReactDOM from "react-dom/client";
import { NextUIProvider } from "@nextui-org/react";
import App from "./App";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <NextUIProvider>
        <main className="w-screen h-screen">
            <App />
        </main>
    </NextUIProvider>
);
