import ReactDOM from "react-dom/client";
import { NextUIProvider } from "@nextui-org/react";

import { Page as RootElement } from "./index";
import { Page as ModConfictElement } from "./ms";
import { Page as DownloadElement } from "./dl";
import { getConfig } from "./utils/config";
import clsx from "clsx";

console.log(window.location.pathname)

function App() {
    if (window.location.pathname == "/")
        return (<RootElement />)
    else if (window.location.pathname == "/ms.html")
        return (<ModConfictElement />)
    else if (window.location.pathname == "/dl.html")
        return (<DownloadElement />)
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <NextUIProvider>
        <main className={clsx("w-screen h-screen", { "dark": getConfig().darkmode })}>
            <App />
        </main>
    </NextUIProvider>
);
