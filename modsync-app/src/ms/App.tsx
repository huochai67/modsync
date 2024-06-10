import React, { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api/core';
import { mb_error, mb_info } from "../messagebox";
import { Button, ButtonGroup, Checkbox, Chip, Paper, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, Typography } from "@mui/material";
import { LoadingButton } from "@mui/lab";
import "../base.css"
import { ArrowBack, CloudDownload, CloudUpload, Done, Refresh } from "@mui/icons-material";


type MSMOD = {
  md5: string,
  path: string,
  size: number,
  url: string | null,
  modid: string | null,
  version: string | null,
}
type MODDiff = {
  select: boolean,
  name: string,
  local: MSMOD | null,
  remote: MSMOD | null,
}

function backtohome() {
  window.location.replace('/')
}

function App() {
  const [difflist, setdifflist] = useState(new Array<MODDiff>());
  const [isreloading, setisreloading] = React.useState(true);

  const [_count, setcount] = React.useState(1)
  function rerender() {
    setcount((c) => c + 1);
  }

  function reload() {
    setisreloading(true);
    invoke<MODDiff[]>('get_diff').then((value) => {
      setdifflist(value.map((value: MODDiff) => { return { select: true, name: value.name, local: value.local, remote: value.remote } }));
      setisreloading(false);
      if (value.length == 0) {
        mb_info("no update");
        backtohome();
      }
    }).catch(mb_error);
  }
  useEffect(reload, [])

  const [btnStartStatus, setbtnStartStatus] = React.useState(false);

  const columns = [
    { field: 'select', headerName: "Select" },
    { field: 'local', headerName: "Local" },
    { field: 'remote', headerName: "Remote" },
  ];

  function rendercell(value: MSMOD | null) {
    if (!value)
      return (<>None</>)
    return (
      <div className="flex flex-col space-y-2">
        <div className="flex flex-row h-4 space-x-2">
          <Chip color="primary" label="Path" sx={{ height: '20px', width: "80px" }} /><Typography variant="body2" gutterBottom>{value.path}</Typography>
        </div>
        <div className="flex flex-row h-4 space-x-2">
          <Chip color="primary" label="ModId" sx={{ height: '20px', width: "80px" }} /><Typography variant="body2" gutterBottom>{value.modid}</Typography>
        </div>
        <div className="flex flex-row h-4 space-x-2">
          <Chip color="primary" label="Version" sx={{ height: '20px', width: "80px" }} /><Typography variant="body2" gutterBottom>{value.version}</Typography>
        </div>
        <div className="flex flex-row h-4 space-x-2">
          <Chip color="primary" label="Md5" sx={{ height: '20px', width: "80px" }} /><Typography variant="body2" gutterBottom>{value.md5}</Typography>
        </div>
      </div>
    )
  }


  return (
    <main className="w-screen h-screen rounded-xl border-4">
      <div className="flex flex-col h-full divide-y-4">
        <div className="grow w-full overflow-auto">
          <TableContainer component={Paper}>
            <Table id="table">
              <TableHead>
                <TableRow id="header">
                  {columns.map((col) => {
                    if (col.field == 'select')
                      return <TableCell id="select" className="w-4">
                        <Checkbox defaultChecked onChange={(_, checked) => { difflist.forEach((value) => { value.select = checked }); rerender(); }} />
                      </TableCell>
                    else if (col.field == "local")
                      return <TableCell id="local" className=" w-2/5" align='left'><Typography variant="body2" gutterBottom><CloudDownload />  Local</Typography></TableCell>
                    else if (col.field == "remote")
                      return <TableCell id="remote" align='left'><Typography variant="body2" gutterBottom><CloudUpload />  Remote</Typography></TableCell>
                  })}
                </TableRow>
              </TableHead>
              <TableBody>
                {difflist.map((row) => {
                  return (
                    <TableRow>
                      <TableCell align="center">
                        <Checkbox className="w-4" checked={row.select} onClick={() => { row.select = !row.select; rerender(); }} />
                      </TableCell>
                      <TableCell align="left" className=" w-2/5">{rendercell(row.local)}</TableCell>
                      <TableCell align="left">{rendercell(row.remote)}</TableCell>
                    </TableRow>)
                })}
              </TableBody>
            </Table>
          </TableContainer>
        </div>
        <div className="flex">
          <div className="grow" />
          <ButtonGroup variant="contained" aria-label="Loading button group" disabled={btnStartStatus}>
            <Button onClick={backtohome} endIcon={<ArrowBack />}>Back</Button>
            <LoadingButton loading={isreloading} endIcon={<Refresh />} loadingPosition="end" onClick={reload}>Refresh</LoadingButton>
            <LoadingButton loading={isreloading || btnStartStatus} loadingPosition="end" endIcon={<Done />} onClick={() => {
              setbtnStartStatus(true);
              let sendlist = new Array()
              difflist.forEach((value) => {
                if (value.select) sendlist.push({ name: value.name, local: value.local, remote: value.remote });
              })
              invoke<MODDiff[]>('apply_diff', {
                diffs: sendlist,
              }).then(() => {
                setbtnStartStatus(false);
                window.location.replace('dl.html')
              }).catch(mb_error);
            }}>Sync</LoadingButton>
          </ButtonGroup>
        </div>
      </div>
    </main>
  );
}

export default App;
