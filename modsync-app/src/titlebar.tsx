import { MinusIcon, MoonIcon, SunIcon, XMarkIcon } from "@heroicons/react/24/solid";
import { Button, ButtonGroup } from "@nextui-org/react";
import { getCurrent } from "@tauri-apps/api/window";
import React from "react";

export function TitleBar({ dark, onDarkChanged }: { dark: boolean, onDarkChanged: (() => void) }) {
    const [title, settitle] = React.useState("Title")
    getCurrent().title().then(settitle)
    return (
        <div data-tauri-drag-region className="flex items-center h-full border-x-4 border-background">
            <p>{title}</p>
            <div className="grow"/>
            <ButtonGroup variant="light" size="sm">
                <Button isIconOnly aria-label="tb-darkmode" onClick={onDarkChanged}>{dark ? <SunIcon /> : <MoonIcon />}</Button>
                <Button isIconOnly aria-label="tb-minus" onClick={() => { getCurrent().minimize() }}><MinusIcon /></Button>
                <Button isIconOnly aria-label="tb-close" onClick={() => { getCurrent().close() }}><XMarkIcon /></Button>
            </ButtonGroup>
        </div>
    )
}

export default TitleBar;