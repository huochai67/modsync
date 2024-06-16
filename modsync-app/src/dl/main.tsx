import ReactDOM from "react-dom/client";
import { NextUIProvider } from "@nextui-org/react";
import clsx from "clsx";

import { getConfig } from "../config";
import App from "./App";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <NextUIProvider>
        <main className={clsx("w-screen h-screen", { "dark": getConfig().darkmode })}>
            <App />
        </main>
    </NextUIProvider>
);
