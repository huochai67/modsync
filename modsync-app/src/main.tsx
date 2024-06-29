import ReactDOM from "react-dom/client";
import { useMemo, useState } from "react";
import { NextUIProvider } from "@nextui-org/react";
import clsx from "clsx";

import { Page as RootElement } from "./index";
import { Page as ModConfictElement } from "./ms";
import { Page as DownloadElement } from "./dl";
import TitleBar from "./titlebar";

import { getConfig, setConfig } from "./utils/config";

console.log(window.location.pathname)

function App() {
    const [dark, setdark] = useState(false);
    useMemo(() => {
        setdark(getConfig().darkmode);
    }, []);
    function Body() {
        if (window.location.pathname == "/")
            return (<RootElement />)
        else if (window.location.pathname == "/ms.html")
            return (<ModConfictElement />)
        else if (window.location.pathname == "/dl.html")
            return (<DownloadElement />)
    }
    return (
        <main className={clsx("w-screen h-screen flex flex-col divide-background border-background text-foreground bg-background", { "dark": dark })}>
            <div className="h-[5vh]">
                <TitleBar dark={dark} onDarkChanged={() => { setConfig({ darkmode: !dark }); setdark((d) => !d) }} />
            </div>
            <div className="grow h-max-[95vh]"><Body /></div>
        </main>
    )
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <NextUIProvider>
        <App />
    </NextUIProvider>
);
