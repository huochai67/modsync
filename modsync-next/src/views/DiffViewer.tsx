import React, { useCallback, useEffect } from "react";
import { MOCK_MOD_DIFFS } from "@/mockData";
import { MODDiff, TaskRunSummary } from "@/types";
import { Button, Checkbox, Label, ListBox } from "@heroui/react";
import { RefreshCw } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import ModDiffListItem from "@/components/ModDiffListItem";
import { useNavigate } from "react-router-dom";

const DiffViewer: React.FC = () => {
  const navigate = useNavigate();

  const [diff, setDiff] = React.useState<Array<MODDiff>>(MOCK_MOD_DIFFS);
  const [initialized, setInitialized] = React.useState(false);

  // Load ModDiff
  const fetchModDiff = useCallback(async () => {
    setInitialized(false);
    try {
      const moddiffs = await invoke<MODDiff[]>("get_diff");
      setDiff(moddiffs);
    } catch (error) {
      alert("Failed to load runtime : " + error);
    } finally {
      setInitialized(true);
    }
  }, []);
  useEffect(() => {
    fetchModDiff();
  }, []);

  const [syncConfigPack, setSyncConfigPack] = React.useState<boolean>(true);
  const [selectKeys, setSelectKeys] = React.useState<string | Set<string>>(
    "all",
  );
  const onBtnSyncClicked = async () => {
    if (!initialized) return;

    let selected_diffs: MODDiff[] = [];
    if (selectKeys === "all") {
      selected_diffs = diff;
    } else if (selectKeys instanceof Set) {
      selected_diffs = diff.filter((d) => selectKeys.has(d.name));
    } else {
      selected_diffs = diff.filter((d) => d.name === selectKeys);
    }
    try {
      const summary = await invoke<TaskRunSummary>("apply_diff", {
        diffs: selected_diffs,
        backup: true,
        syncConfigPack,
      });
      alert(summary.failed === 0 ? `同步完成：${summary.succeeded} 项成功。` : `同步完成：${summary.succeeded} 项成功，${summary.failed} 项失败。`);
      navigate("/taskmanager");
    } catch (error) {
      alert("同步失败：" + error);
      navigate("/taskmanager");
    }
  };

  return (
    <div className="page-wrap h-full space-y-6">
      <div className="flex flex-row justify-between items-end">
        <div>
          <div className="eyebrow">Sync preview</div>
          <h1 className="page-heading">文件差异</h1>
          <p className="page-description">选择要处理的项目，确认后将同步到本地客户端。</p>
        </div>
        <div className="flex flex-col items-center space-y-1">
          <div className="w-full flex justify-end">
            <Button
              onClick={onBtnSyncClicked}
              isDisabled={
                !initialized || (diff.length === 0 && !syncConfigPack)
              }
            >
              <RefreshCw size={16}/> 开始同步
            </Button>
          </div>
          <div className="flex items-center gap-3">
            <Checkbox
              id="sync-config-pack"
              isSelected={syncConfigPack}
              onChange={(e) => {
                setSyncConfigPack(e);
              }}
            >
              <Checkbox.Control>
                <Checkbox.Indicator />
              </Checkbox.Control>
            </Checkbox>
            <Label htmlFor="sync-config-pack">同步配置包</Label>
          </div>
          {/* <span className="text-sm text-muted">{diff.length}条差异</span> */}
        </div>
      </div>
      {initialized ? (
        diff.length !== 0 ? (
          <div className="surface list-surface">
            <div className="list-header"><span>已检测到 {diff.length} 项变更</span><span>默认已全选</span></div>
            <div className="overflow-x-auto">
              <ListBox
                selectionMode="multiple"
                aria-label="l"
                defaultSelectedKeys={"all"}
                onSelectionChange={(keys) => {
                  setSelectKeys(keys as string | Set<string>);
                }}
              >
                {diff.map((diff) => (
                  <ModDiffListItem key={diff.name} moddiff={diff} />
                ))}
              </ListBox>
            </div>
          </div>
        ) : (
          <div className="flex items-center justify-center h-64">
            <span className="text-muted">没有检测到差异</span>
          </div>
        )
      ) : (
        <div className="flex items-center justify-center h-64">
          <span className="text-muted">加载中...</span>
        </div>
      )}
    </div>
  );
};

export default DiffViewer;
