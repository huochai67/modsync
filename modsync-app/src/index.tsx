import { invoke } from '@tauri-apps/api/core';
import { Window } from '@tauri-apps/api/window'
import { Button, ButtonGroup } from "@nextui-org/button";
import { useEffect, useState } from 'react';
import { Cog6ToothIcon, ServerStackIcon, Square2StackIcon } from '@heroicons/react/24/solid';
import { useTranslation } from 'react-i18next';

import { mb_error, mb_info } from './utils/messagebox';
import { Divider, Snippet } from '@nextui-org/react';
import "./utils/i18n"

import "./global.css";


function BTNSyncServerList() {
  const { t } = useTranslation();
  const [okstate, setOkState] = useState(true);
  return (
    <Button endContent={<ServerStackIcon className='size-4' />} isLoading={!okstate} onClick={() => {
      setOkState(false);
      invoke('download_serverlist').then(() => {
        mb_info(t('SYNCMPLDONE'));
        setOkState(true);
      }).catch(mb_error);
    }}>
      {t('SYNCMPL')}
    </Button>
  );
} function BTNSyncSetting() {
  const { t } = useTranslation();
  const [okstate, setOkState] = useState(true);
  return (
    <Button endContent={<Cog6ToothIcon className='size-4' />} isLoading={!okstate} onClick={() => {
      setOkState(false);
      invoke('download_options').then(() => {
        mb_info(t('SYNCOPTIONSDONE'));
        setOkState(true);
      }).catch(mb_error);
    }}>
      {t('SYNCOPTIONS')}
    </Button>
  );
}
function BTNShowConfict() {
  const { t } = useTranslation();
  return (
    <Button endContent={<Square2StackIcon className='size-4' />} onClick={() => {
      window.location.replace('ms.html')
    }}>
      {t('SHOWCONFLICT')}
    </Button>
  );
}

export function Page() {
  const [changelog, setChangelog] = useState('CHANGELOG');
  const [init, setinit] = useState(false);

  useEffect(() => {
    invoke<string>('get_title')
      .then(result => {
        Window.getCurrent().setTitle(result);
      })
  }, []);

  useEffect(() => {
    invoke<string>('get_changelog')
      .then(result => {
        setChangelog(result);
        setinit(true);
      })
      .catch(error => {
        mb_error(error);
      })
  }, []);

  return (
    <div className="flex flex-col h-full border-4 divide-y-4  divide-background border-background text-foreground bg-background">
      <div className="grow w-full">
        <textarea className="h-full w-full resize-none" value={changelog} readOnly />
      </div>
      <Divider />
      <div className="flex h-14">
        <div className='flex items-center space-x-2'>
          <Snippet>https://github.com/huochai67/modsync</Snippet>
        </div>
        <div className="grow" />
        <div className='w-[50vw] flex items-center justify-end'>
          <ButtonGroup color='primary' variant="solid" isDisabled={!init}>
            <BTNSyncServerList />
            <BTNSyncSetting />
            <BTNShowConfict />
          </ButtonGroup>
        </div>
      </div>
    </div>
  );
}

export default Page;
