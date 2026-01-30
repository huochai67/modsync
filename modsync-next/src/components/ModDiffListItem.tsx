
import React, { ReactElement } from 'react';

import { Avatar, Chip, Description, Label, ListBox, ListBoxProps, Surface } from "@heroui/react";

import { MODDiff, MSMOD } from '@/types';
import { Binary, File, FileCode, FileMinus, FilePlus, Package, ShieldCheck } from 'lucide-react';
import { formatBytes } from '@/utils';

interface ModDiffListItemProps {
    moddiff: MODDiff;
}

const ModDiffListItem: React.FC<ModDiffListItemProps> = ({ moddiff }) => {
    if (moddiff.local === undefined && moddiff.remote === undefined) {
        throw Error("Both local and remote mod data are undefined");
    }

    let badge_type: ReactElement | null = null;
    const localsize = moddiff.local ? moddiff.local.size : 0;
    const remotesize = moddiff.remote ? moddiff.remote.size : 0;
    const size = remotesize - localsize;
    if (moddiff.difftype === 'MODIFIED') {
        badge_type = <Chip size='sm' color='accent'><FileCode /><p className='text-center font-bold'>MODIFIED</p></Chip>
    }
    if (moddiff.difftype === 'NEWED') {
        badge_type = <Chip size='sm' color='success'><FilePlus /><p className='text-center font-bold'>NEWED</p></Chip>
    }
    if (moddiff.difftype === 'DELETED') {
        badge_type = <Chip size='sm' color='danger'><FileMinus /><p className='text-center font-bold'>DELETED</p></Chip>
    }
    let chip_version: ReactElement | null = null;
    if (moddiff.difftype === 'MODIFIED' && moddiff.local && moddiff.remote) {
        chip_version = <Chip size='sm' color='accent'>{moddiff.local.version} → {moddiff.remote.version}</Chip>
    }

    const info: MSMOD = moddiff.local ? moddiff.local : moddiff.remote!;

    return (
        <ListBox.Item id={moddiff.name} key={moddiff.name} textValue={moddiff.name} aria-label=''>
            <div className='w-full h-auto flex flex-row items-center space-x-4'>
                <div className='flex flex-row items-center space-x-2 min-w-1/4'>
                    <div className='text-accent'>{moddiff.kind === 'MOD' ? <File /> : <Package />}</div>
                    <div className="flex flex-col">
                        <Label>{moddiff.name}</Label>
                        <Description>{moddiff.kind === 'MOD' ? info.modid : "PLAINFILE"}</Description>
                    </div>
                </div>
                <Chip className='w-28'>{badge_type}</Chip>
                <div className='flex flex-col w-full text-xs items-center space-y-1'>
                    <div>{`${formatBytes(localsize)} -> ${formatBytes(remotesize)}`}</div>
                    <Chip color={size > 0 ? 'success' : 'danger'}>{formatBytes(size, true)}</Chip>
                    {chip_version}
                </div>
                <div className='flex flex-col min-w-1/3'>
                    <div className="space-y-2 text-xs truncate font-mono">
                        <div className="flex items-center gap-2 text-muted bg-background-secondary p-1.5 rounded">
                            <Binary size={14} />
                            <span>
                                {moddiff.local ? moddiff.local.md5 : 'N/A'}
                            </span>
                        </div>
                        <div className="flex items-center gap-2 text-foreground bg-background p-1.5 rounded">
                            <ShieldCheck size={14} />
                            <span>
                                {moddiff.remote ? moddiff.remote.md5 : 'N/A'}
                            </span>
                        </div>
                    </div>
                </div>
                <ListBox.ItemIndicator />
            </div>
        </ListBox.Item>
    )
}

// interface ModDiffListProps {
//     moddiffs: MODDiff[];
//     className?: string;
//     onClicked: () => Promise<void>;
// }
// const ModDiffList: React.FC<ModDiffListProps> = ({ onClicked, moddiffs, className }) => {
//     return (
//         <ListBox selectionMode="multiple" className={className} defaultSelectedKeys={"all"}>
//             {moddiffs.map((diff) => <ModDiffListItem moddiff={diff} />)}
//         </ListBox>
//     );

// };

export default ModDiffListItem;