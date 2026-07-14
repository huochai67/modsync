import React from "react";

import { RuntimeContext } from "../runtimecontext";

import { Button } from "@heroui/react";
import { invoke } from "@tauri-apps/api/core";
import SyncButton from "@/components/SyncButton";
import { openUrl } from "@tauri-apps/plugin-opener";

const Utilities: React.FC = () => {
  const runtime = React.useContext(RuntimeContext);

  const download_utility = async (utility: string) => {
    await invoke<void>("download_utility", { utility: utility });
    alert(`${utility} 下载完成！`);
  };

  return (
    <div className="page-wrap min-h-full flex flex-col gap-4 overflow-y-auto">
      <header className="mb-2"><div className="eyebrow">Client toolkit</div><h1 className="page-heading">实用功能</h1><p className="page-description">快速补齐客户端常用配置与启动器。</p></header>
      <section className="surface utility-card">
          <h2 className="utility-title">杂项下载</h2><p className="utility-desc">一键下载常用的客户端文件</p>
        <div className="flex flex-row flex-wrap gap-3 mt-5">
          <SyncButton
            isDisabled={!runtime.has_options}
            onClicked={() => download_utility("options")}
          >
            按键设置
          </SyncButton>
          <SyncButton
            isDisabled={!runtime.has_serverdat}
            onClicked={() => download_utility("serverdat")}
          >
            服务器列表
          </SyncButton>
          <SyncButton
            isDisabled={!runtime.has_hcml}
            onClicked={() => download_utility("hmcl")}
          >
            HMCL
          </SyncButton>
          <SyncButton
            isDisabled={!runtime.has_pclce}
            onClicked={() => download_utility("pclce")}
          >
            PCL
          </SyncButton>
        </div>
      </section>
      <section className="surface utility-card">
          <h2 className="utility-title">教程</h2><p className="utility-desc">新手必看的使用指南</p>
        <div className="flex flex-row flex-wrap gap-3 mt-5">
          <Button isDisabled>添加外部登录</Button>
          <Button isDisabled>组装客户端</Button>
        </div>
      </section>
      <section className="surface utility-card">
          <h2 className="utility-title">其他资源</h2><p className="utility-desc">常用的 Minecraft 社区网站</p>
        <div className="flex flex-row flex-wrap gap-3 mt-5">
          <Button onClick={() => openUrl("https://www.mcmod.cn/")}>
            MC百科
          </Button>
        </div>
      </section>
    </div>
  );
};

export default Utilities;
