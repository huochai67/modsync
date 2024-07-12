import { MinusIcon, MoonIcon, SunIcon, XMarkIcon } from "@heroicons/react/24/solid";
import { Button, ButtonGroup, Chip, Tooltip } from "@nextui-org/react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrent } from "@tauri-apps/api/window";
import clsx from "clsx";
import React from "react";

export function TitleBar({ dark, onDarkChanged }: { dark: boolean, onDarkChanged: (() => void) }) {
    const [title, settitle] = React.useState("Title");
    const [version, setversion] = React.useState("");
    const [buildinfo, setbuildinfo] = React.useState("");
    getCurrent().title().then(settitle);
    invoke<string>('get_version').then(setversion)
    invoke<string>('get_buildinfo').then(setbuildinfo)
    return (
        <div data-tauri-drag-region className="flex items-center h-full border-x-4 border-background text-foreground bg-background">
            <p>{title}</p>
            <Tooltip content={buildinfo} closeDelay={100} placement="right-end" className={clsx("text-foreground bg-background", {"dark" : dark})}>
                <Chip size="sm" variant="bordered">{version}</Chip>
            </Tooltip>
            <div className="grow" />
            <ButtonGroup variant="light" size="sm">
                <Button isIconOnly aria-label="tb-darkmode" onClick={onDarkChanged}>{dark ? <SunIcon /> : <MoonIcon />}</Button>
                <Button isIconOnly aria-label="tb-minus" onClick={() => { getCurrent().minimize() }}><MinusIcon /></Button>
                <Button isIconOnly aria-label="tb-close" onClick={() => { getCurrent().close() }}><XMarkIcon /></Button>
            </ButtonGroup>
        </div>
    )
}

export default TitleBar;