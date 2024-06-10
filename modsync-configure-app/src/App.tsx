import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ButtonGroup, Checkbox, FormControlLabel, TextField } from "@mui/material";

import "./App.css";
import { mb_error } from "./messagebox";
import { LoadingButton } from "@mui/lab";
import { Done, Refresh } from "@mui/icons-material";

type msconfig = {
  base_url: string,
  title: string,
  force_sync_server_list: boolean,
}

function App() {
  const [isreload, setisreload] = useState(false);
  const [isgenerate, setisgenerate] = useState(false);
  const [changelog, setChangelog] = useState('CHANGELOG');
  const [forcesyncsl, setforcesyncsl] = useState(false);
  const [serverurl, setserverurl] = useState("");
  const [title, settitle] = useState("");

  function reload() {
    setisreload(true);
    invoke<msconfig>('get_config')
      .then(result => {
        setforcesyncsl(result.force_sync_server_list);
        setserverurl(result.base_url);
        settitle(result.title);
        invoke<string>('get_changelog')
          .then(result => {
            setChangelog(result);
            setisreload(false);
          })
          .catch(error => {
            mb_error(error);
          })
      })
      .catch(error => {
        mb_error(error);
      })
  }
  useEffect(reload, [])

  return (
    <main className="w-screen h-screen rounded-xl border-4">
      <div className="flex flex-col h-full divide-y-4">
        <div className="grow">
          <textarea className="h-full w-full" value={changelog} readOnly />
        </div>
        <TextField label="ServerUrl" variant="outlined" value={serverurl} onChange={(e) => { setserverurl(e.target.value) }} />
        <TextField label="Title" variant="outlined" value={title} onChange={(e) => { settitle(e.target.value) }} />
        <div className="flex">
          <FormControlLabel control={<Checkbox checked={forcesyncsl} onClick={() => { setforcesyncsl(() => !forcesyncsl) }} />} label="Force Sync ServerList" />
          <div className="grow"></div>
          <ButtonGroup>
            <LoadingButton loading={isreload} endIcon={<Refresh/>} loadingPosition="end" variant="contained" onClick={reload}>Reload</LoadingButton>
            <LoadingButton loading={isgenerate} endIcon={<Done/>} loadingPosition="end" variant="contained" onClick={() => {
              setisgenerate(true)
              invoke('generate', {changelog : changelog, title : title, serverurl : serverurl, forceserverlist : forcesyncsl})
                .then(() => {
                  setisgenerate(false);
                })
                .catch(error => {
                  mb_error(error);
                })
            }}>Generate</LoadingButton>
          </ButtonGroup>
        </div>
      </div>
    </main>
  );
}

export default App;
