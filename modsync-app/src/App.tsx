import { invoke } from '@tauri-apps/api/core';
import { Window } from '@tauri-apps/api/window'
import { Button, ButtonGroup } from "@nextui-org/button";
import { useMemo, useState } from 'react';
import clsx from "clsx";
import { Cog6ToothIcon, ServerStackIcon, Square2StackIcon, SunIcon } from '@heroicons/react/24/solid';

import { mb_error, mb_info } from './messagebox';
import { Divider } from '@nextui-org/react';
import "./global.css";
import { getConfig, setConfig } from './config';

function BTNSyncServerList() {
  const [okstate, setOkState] = useState(true);
  return (
    <Button endContent={<ServerStackIcon className='size-4' />} isLoading={!okstate} onClick={() => {
      setOkState(false);
      invoke('download_serverlist').then(() => {
        mb_info("ok");
        setOkState(true);
      }).catch(mb_error);
    }}>
      SyncMPlist
    </Button>
  );
} function BTNSyncSetting() {
  const [okstate, setOkState] = useState(true);
  return (
    <Button endContent={<Cog6ToothIcon className='size-4' />} isLoading={!okstate} onClick={() => {
      setOkState(false);
      invoke('download_options').then(() => {
        mb_info("ok");
        setOkState(true);
      }).catch(mb_error);
    }}>
      SyncOption
    </Button>
  );
}
function BTNShowConfict() {
  return (
    <Button endContent={<Square2StackIcon className='size-4' />} onClick={() => {
      window.location.replace('ms.html')
    }}>
      ShowConfict
    </Button>
  );
}

function App() {
  const [dark, setdark] = useState(false);
  useMemo(() => {
    setdark(getConfig().darkmode);
  }, []);

  const [changelog, setChangelog] = useState('CHANGELOG');
  const [init, setinit] = useState(false);

  useMemo(() => invoke<string>('get_title').then(result => {
    Window.getCurrent().setTitle(result);
  }), [])
  useMemo(() => invoke<string>('get_changelog')
    .then(result => {
      setChangelog(result);
      setinit(true);
    })
    .catch(error => {
      mb_error(error);
    }), []);

  return (
    <div className={clsx("flex flex-col h-full border-4 divide-y-4 divide-background border-background text-foreground bg-background", { "dark": dark })}>
      <div className="grow w-full">
        <textarea className="h-full w-full resize-none" value={changelog} readOnly />
      </div>
      <Divider />
      <div className="flex">
        <Button isIconOnly aria-label="Dark" onClick={() => {
          setConfig({ darkmode: !dark });
          setdark((d) => !d);
        }}><SunIcon /></Button>
        <div className="grow" />
        <ButtonGroup className='w-[40vw]' color='primary' variant="solid" isDisabled={!init}>
          <BTNSyncServerList />
          <BTNSyncSetting />
          <BTNShowConfict />
        </ButtonGroup>
      </div>
    </div>
  );
}

export default App;
