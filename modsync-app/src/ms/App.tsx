import React, { useEffect, useState } from "react";
import { Selection } from '@react-types/shared';
import { invoke } from '@tauri-apps/api/core';
import { Button, ButtonGroup, Card, Table, TableHeader, TableColumn, TableBody, TableRow, TableCell, Chip, CardBody, Spinner, Tooltip, Divider } from "@nextui-org/react";
import { ArrowLeftIcon, ArrowPathIcon, CheckIcon } from '@heroicons/react/24/solid'
import clsx from 'clsx';
import { useTranslation } from 'react-i18next';

import { mb_error, mb_info } from "../messagebox";
import "../global.css";
import "../i18n"


type MSMOD = {
  md5: string,
  path: string,
  size: number,
  url: string | null,
  modid: string | null,
  version: string | null,
}
type MODDiff = {
  index: number,
  name: string,
  local: MSMOD | null,
  remote: MSMOD | null,
}

function backtohome() {
  window.location.replace('/')
}

function App() {
  const { t } = useTranslation();
  const [difflist, setdifflist] = useState(new Array<MODDiff>());
  const [isreloading, setisreloading] = React.useState(true);
  const [selectedKeys, setSelectedKeys] = React.useState<Selection>('all');
  const [btnStartStatus, setbtnStartStatus] = React.useState(false);
  function reload() {
    setisreloading(true);
    invoke<MODDiff[]>('get_diff').then((value) => {
      setdifflist(value.map((value: MODDiff, index) => { return { index, name: value.name, local: value.local, remote: value.remote } }));
      setisreloading(false);
      if (value.length == 0) {
        mb_info("no update");
        backtohome();
      }
    }).catch(mb_error);
  }
  useEffect(reload, [])

  let selecteddiffs = React.useMemo(() => {
    if (selectedKeys === 'all') {
      return difflist;
    } else {
      let ret = Array();
      selectedKeys.forEach((key) => {
        let index = -1;
        if (typeof key === "string") {
          index = parseInt(key, 10);
        } else {
          index = key
        }
        ret.push(difflist[index])
      });
      return ret;
    }
  }, [selectedKeys, difflist]);

  const SelectInfo = React.useCallback(() => {
    let sdelect = 0;
    let supdate = 0;
    let sdownload = 0;
    selecteddiffs.forEach((item) => {
      if (item) {
        if (item.local && item.remote)
          supdate++;
        if (item.local && !item.remote)
          sdelect++;
        if (!item.local && item.remote)
          sdownload++;
      }
    })
    return (
      <Card>
        <CardBody>
          <div className="flex flex-row space-x-1">
            <Chip color="secondary">{t("UPDATE") + supdate.toString()}</Chip>
            <Chip color="warning">{t("DELETE") + sdelect.toString()}</Chip>
            <Chip color="primary">{t("DOWNLOAD") + sdownload.toString()}</Chip>
          </div>
        </CardBody>
      </Card>
    )
  }, [selecteddiffs, difflist]);

  const renderCell = React.useCallback((diff: MODDiff, columnKey: string) => {
    function rendercellimpl(value: MSMOD | null, diff: MODDiff) {
      if (!value)
        return (<>{t("NONE")}</>)
      let diffpath = false, diffmd5 = false, diffmodid = false, diffversion = false;
      if (diff.local != null && diff.remote != null) {
        diffpath = diff.local.path != diff.remote.path;
        diffmd5 = diff.local.md5 != diff.remote.md5;
        diffmodid = diff.local.modid != diff.remote.modid;
        diffversion = diff.local.version != diff.remote.version;
      }

      function cell_renderrow(clip: string, lable: string | null, red: boolean) {
        return (
          <div className="flex flex-row space-x-2">
            <Chip size="sm" className="h-5 text-center" classNames={{ content: "w-14" }} color={red ? "danger" : "primary"}>{clip}</Chip >
            <Tooltip placement="top-start" content={lable} delay={1000}><p className={clsx("flex items-center w-[30vw] overflow-hidden text-nowrap", { "text-red-700": red })}>{lable}</p></Tooltip>
          </div>
        )
      }

      return (
        <Card>
          <CardBody className="space-y-1">
            {cell_renderrow("PATH", value.path, diffpath)}
            {cell_renderrow("MD5", value.md5, diffmd5)}
            {cell_renderrow("MODID", value.modid, diffmodid)}
            {cell_renderrow("VER", value.version, diffversion)}
          </CardBody>
        </Card>
      )
    }

    switch (columnKey) {
      case "local":
        return rendercellimpl(diff.local, diff);
      case "remote":
        return rendercellimpl(diff.remote, diff);
      default:
        return <>UNKNOWN</>;
    }
  }, []);

  return (
    <div className="flex flex-col h-full border-4 divide-y-4 divide-background border-background text-foreground bg-background">
      <div className="grow w-full overflow-auto">
        <Table aria-label="Difflist table"
          selectionMode="multiple"
          selectedKeys={selectedKeys}
          onSelectionChange={setSelectedKeys}>
          <TableHeader>
            <TableColumn key={"local"}>{t("LOCAL")}</TableColumn>
            <TableColumn key={"remote"}>{t("REMOTE")}</TableColumn>
          </TableHeader>
          <TableBody items={difflist} isLoading={isreloading || btnStartStatus} loadingContent={<Spinner label="Loading..." />}>
            {(item) => (
              <TableRow key={item.index}>
                {(columnKey) => <TableCell className="w-[50vw]">{renderCell(item, columnKey.toString())}</TableCell>}
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>
      <Divider />
      <div className="flex h-14">
        <SelectInfo />
        <div className="grow" />
        <div className='w-[50vw] flex items-center justify-end'>
          <ButtonGroup variant="solid" color="primary" isDisabled={btnStartStatus || isreloading}>
            <Button className="w-1/3" endContent={<ArrowLeftIcon className="size-4" />} isLoading={isreloading} onClick={backtohome} >{t("BACK")}</Button>
            <Button className="w-1/3" endContent={<ArrowPathIcon className="size-4" />} isLoading={isreloading} onClick={reload}>{t("RELOAD")}</Button>
            <Button className="w-1/3" endContent={<CheckIcon className="size-4" />} isLoading={isreloading || btnStartStatus} onClick={() => {
              setbtnStartStatus(true);
              let sendlist = new Array()
              selecteddiffs.forEach((value) => {
                sendlist.push({ name: value.name, local: value.local, remote: value.remote });
              })
              invoke<MODDiff[]>('apply_diff', {
                diffs: sendlist,
              }).then(() => {
                setbtnStartStatus(false);
                window.location.replace('dl.html')
              }).catch(mb_error);
            }}>{t("SYNC")}</Button>
          </ButtonGroup>
        </div>
      </div>
    </div>
  );
}

export default App;