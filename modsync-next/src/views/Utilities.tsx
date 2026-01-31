
import React from 'react';

import { RuntimeContext } from '../runtimecontext';

import { Button, Card, Separator } from '@heroui/react';
import { invoke } from '@tauri-apps/api/core';
import SyncButton from '@/components/SyncButton';
import { openUrl } from '@tauri-apps/plugin-opener';

const Utilities: React.FC = () => {
    const runtime = React.useContext(RuntimeContext);

    const commoncallback = async (func: string) => {
        await invoke<void>(func);
        alert("下载完成！");
    }

    return (
        <div className='w-full min-h-full flex flex-col gap-6 overflow-y-auto'>
            <Card className='w-full'>
                <Card.Header>
                    <Card.Title>杂项下载</Card.Title>
                    <Card.Description>点一下玩一年</Card.Description>
                </Card.Header>
                <Separator />
                <Card.Content className='flex flex-row flex-wrap gap-3'>
                    <SyncButton isDisabled={!runtime.has_options} onClicked={() => commoncallback("download_options")} >按键设置</SyncButton>
                    <SyncButton isDisabled={!runtime.has_serverdat} onClicked={() => commoncallback("download_serverlist")} >服务器列表</SyncButton>
                    <SyncButton isDisabled={!runtime.has_hcml} onClicked={() => commoncallback("download_hmcl")} >HMCL</SyncButton>
                    <SyncButton isDisabled={!runtime.has_pclce} onClicked={() => commoncallback("download_pcl")} >PCL</SyncButton>
                </Card.Content>
            </Card>
            <Card className='w-full'>
                <Card.Header>
                    <Card.Title>教程</Card.Title>
                    <Card.Description>新手必看</Card.Description>
                </Card.Header>
                <Separator />
                <Card.Content className='flex flex-row flex-wrap gap-3'>
                    <Button isDisabled>添加外部登录</Button>
                    <Button isDisabled>组装客户端</Button>
                </Card.Content>
            </Card>
            <Card className='w-full'>
                <Card.Header>
                    <Card.Title>其他</Card.Title>
                    <Card.Description>一些实用网址</Card.Description>
                </Card.Header>
                <Separator />
                <Card.Content className='flex flex-row flex-wrap gap-3'>
                    <Button onClick={() => openUrl("https://www.mcmod.cn/")}>MC百科</Button>
                </Card.Content>
            </Card>
        </div>
    );
};

export default Utilities;
