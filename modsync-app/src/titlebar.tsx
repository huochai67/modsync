import { MinusIcon, MoonIcon, SunIcon, XMarkIcon } from "@heroicons/react/24/solid";
import { Button } from "@nextui-org/react";
import { getCurrent } from "@tauri-apps/api/window";

export function TitleBar({ dark, onDarkChanged }: { dark: boolean, onDarkChanged: (() => void) }) {
    return (
        <div data-tauri-drag-region className="flex flex-row-reverse items-center h-full">
            <Button aria-label="close" isIconOnly size="sm" variant="light" onClick={() => { getCurrent().close() }} endContent={<XMarkIcon />} />
            <Button aria-label="close" isIconOnly size="sm" variant="light" onClick={() => { getCurrent().minimize() }} endContent={<MinusIcon />} />
            <Button size='sm' variant='light' isIconOnly aria-label="Dark" onClick={onDarkChanged}>{dark ? <SunIcon /> : <MoonIcon />}</Button>
        </div>
    )
}

export default TitleBar;