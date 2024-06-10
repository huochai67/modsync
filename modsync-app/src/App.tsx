import { invoke } from '@tauri-apps/api/core';
import { Alert, Button, ButtonGroup } from '@mui/material';
import { LoadingButton } from '@mui/lab';
import { Difference, Dns, Settings } from '@mui/icons-material';

import { useMemo, useState } from 'react';

import { mb_error, mb_info } from './messagebox';
import "./base.css";

function BTNSyncServerList() {
  const [okstate, setOkState] = useState(true);
  return (
    <LoadingButton endIcon={<Dns />} loadingPosition="end" variant="contained" loading={!okstate} onClick={() => {
      setOkState(false);
      invoke('download_serverlist').then(() => {
        mb_info("ok");
        setOkState(true);
      }).catch(mb_error);
    }}>
      SyncServerlist
    </LoadingButton>
  );
} function BTNSyncSetting() {
  const [okstate, setOkState] = useState(true);
  return (
    <LoadingButton variant="contained" endIcon={<Settings/>} loadingPosition="end"  loading={!okstate} onClick={() => {
      setOkState(false);
      invoke('download_options').then(() => {
        mb_info("ok");
        setOkState(true);
      }).catch(mb_error);
    }}>
      SyncOption
    </LoadingButton>
  );
}
function BTNShowConfict() {
  return (
    <Button variant="contained" endIcon={<Difference/>} onClick={() => {
      window.location.replace('ms.html')
    }}>
      ShowConfict
    </Button>
  );
}

function App() {
  const [changelog, setChangelog] = useState('CHANGELOG');

  useMemo(() => invoke<string>('get_changelog')
    .then(result => {
      setChangelog(result);
    })
    .catch(error => {
      mb_error(error);
    }), []);

  return (
    <main className="w-screen h-screen rounded-xl border-4">
      <div className="flex flex-col h-full divide-y-4">
        <Alert variant="filled" severity="warning">Internal Version.</Alert>
        <div className="grow">
          <textarea className="h-full w-full" value={changelog} readOnly />
        </div>
        <div className="flex">
          <div className="grow" />
          <ButtonGroup variant="contained" aria-label="Loading button group">
            <BTNSyncServerList />
            <BTNSyncSetting />
            <BTNShowConfict />
          </ButtonGroup>
        </div>
      </div>
    </main>
  );
}

export default App;
