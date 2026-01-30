
import React, { useCallback, useEffect } from 'react';
import { MOCK_MOD_DIFFS } from '@/mockData';
import { MODDiff } from '@/types';
import { Button, ListBox, Separator } from '@heroui/react';
import { invoke } from '@tauri-apps/api/core';
import ModDiffListItem from '@/components/ModDiffListItem';

const DiffViewer: React.FC = () => {
  const [diff, setDiff] = React.useState<Array<MODDiff>>(MOCK_MOD_DIFFS);
  const [initialized, setInitialized] = React.useState(false);

  // Load ModDiff
  const fetchModDiff = useCallback(async () => {
    setInitialized(false);
    try {
      const initialized = await invoke<boolean>('is_init');
      if (!initialized) {
        await invoke<void>('init_runtime');
      }

      const moddiffs = await invoke<MODDiff[]>('get_diff');
      setDiff(moddiffs)
    } catch (error) {
      alert("Failed to load runtime : " + error);
    } finally {
      setInitialized(true);
    }
  }, []);
  useEffect(() => {
    fetchModDiff();
  }, []);

  const [selectKeys, setSelectKeys] = React.useState<string | Set<string>>("all");
  const onBtnSyncClicked = async () => {
    if (!initialized) return;
    if (diff.length === 0) return;

    console.log("Syncing selected items:", selectKeys);
    let selected_diffs: MODDiff[] = [];
    if (selectKeys === "all") {
      selected_diffs = diff;
    } else if (selectKeys instanceof Set) {
      selected_diffs = diff.filter((d) => selectKeys.has(d.name));
    } else {
      selected_diffs = diff.filter((d) => d.name === selectKeys);
    }
    console.log("同步以下差异项：", selected_diffs);
    await invoke<void>('apply_diff', { diffs: selected_diffs });
  }

  return (
    <div className="w-full h-full space-y-2">
      <div className="flex flex-row justify-between">
        <div>
          <h1 className="text-3xl font-bold text-foreground tracking-tight">文件对比</h1>
          <p className="text-muted mt-1">详细显示每一个差异</p>
        </div>
        <div className="flex flex-col items-center space-y-1">
          <div className='w-full flex justify-end'>
            <Button onClick={onBtnSyncClicked} isDisabled={initialized}>开始同步</Button></div>
          <span className="text-sm text-muted">{diff.length}条差异</span>
        </div>
      </div>
      <Separator />

      {initialized ? diff.length > 0 ?
        <div className="bg-background-tertiary border rounded-xl overflow-hidden">
          <div className="overflow-x-auto">
            <ListBox selectionMode="multiple" aria-label='l' defaultSelectedKeys={"all"} onSelectionChange={(keys) => { setSelectKeys(keys as string | Set<string>) }}>
              {diff.map((diff) => <ModDiffListItem key={diff.name} moddiff={diff} />)}
            </ListBox>
          </div>
        </div>
        :
        <div className="flex items-center justify-center h-64">
          <span className="text-muted">没有检测到差异</span>
        </div>
        :
        <div className="flex items-center justify-center h-64">
          <span className="text-muted">加载中...</span>
        </div>
      }
    </div>
  );
};

export default DiffViewer;
