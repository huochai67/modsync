import React from "react";
import { useNavigate } from "react-router-dom";
import { ArrowRight, Archive, Calendar, FileDown, GitBranch } from "lucide-react";
import { Button } from "@heroui/react";
import { RuntimeContext } from "@/runtimecontext";
import { formatBytes } from "@/utils";

const Dashboard: React.FC = () => {
  const navigate = useNavigate();
  const runtime = React.useContext(RuntimeContext);
  const lastrelease = runtime.release_info[runtime.release_info.length - 1];

  if (!lastrelease) return null;
  return <div className="page-wrap space-y-6">
    <header><div className="eyebrow">Release overview</div><h1 className="page-heading">准备好更新了吗？</h1><p className="page-description">最新客户端内容已经整理完毕，确认差异后即可开始同步。</p></header>
    <section className="surface hero-card">
      <div className="flex flex-col md:flex-row gap-7 md:items-start md:justify-between relative z-10">
        <div><div className="version-pill"><GitBranch size={14}/>{lastrelease.version}</div><h2 className="release-title">{lastrelease.changelog}</h2><p className="release-date flex items-center gap-2"><Calendar size={14}/>{lastrelease.date} 发布</p></div>
        <Button className="primary-cta" onClick={() => navigate("/diffs")}>查看并同步 <ArrowRight size={17}/></Button>
      </div>
      <div className="stat-grid relative z-10">
        <div className="stat"><div className="stat-label">更新大小</div><div className="stat-value">{formatBytes(lastrelease.size, true)}</div></div>
        <div className="stat"><div className="stat-label">新增内容</div><div className="stat-value text-emerald-300">+ {lastrelease.adds?.length || 0}</div></div>
        <div className="stat"><div className="stat-label">移除内容</div><div className="stat-value text-rose-300">− {lastrelease.subs?.length || 0}</div></div>
      </div>
    </section>
    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div className="surface quick-card"><div className="quick-icon"><FileDown size={21}/></div><div><div className="font-semibold">客户端工具</div><p className="page-description mt-1">下载服务器列表、键位配置与启动器</p></div></div>
      <div className="surface quick-card"><div className="quick-icon"><Archive size={21}/></div><div><div className="font-semibold">安全备份</div><p className="page-description mt-1">被移除的 MOD 会保存在 .minecraft/bakmods</p></div></div>
    </div>
  </div>;
};
export default Dashboard;
