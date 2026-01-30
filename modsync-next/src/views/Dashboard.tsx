
import React from 'react';
import { useNavigate } from 'react-router-dom';
import {
  User,
  Calendar,
  Hash,
  GitBranch,
  ArrowRight,
  PlusCircle,
  MinusCircle,
  FileCode
} from 'lucide-react';
import { RuntimeContext } from '@/runtimecontext';
import { formatBytes } from '@/utils';
import { Button, Separator } from '@heroui/react';

const Dashboard: React.FC = () => {
  const navigate = useNavigate();

  const runtime = React.useContext(RuntimeContext);
  const lastrelease = runtime.release_info[runtime.release_info.length - 1];

  return (
    <div className="max-w-4xl mx-auto space-y-8 animate-in fade-in duration-500">
      <header className="flex flex-col gap-2">
        <h1 className="text-3xl font-bold">最新更新</h1>
        <p className="text-muted text-sm">:看看更新了什么东西）</p>
      </header>

      {/* Commit Card */}
      <div className=" bg-background-secondary border  rounded-2xl overflow-hidden shadow-2xl">
        <div className="p-8 space-y-6">
          <div className="flex flex-col md:flex-row md:items-start justify-between gap-6">
            <div className="space-y-4 flex-1">
              <div className="flex items-center gap-2 text-accent bg-background-tertiary w-fit px-3 py-1 rounded-full text-sm font-medium border">
                <GitBranch size={14} />
                <span>{lastrelease.version}</span>
              </div>
              <h2 className="text-2xl font-semibold leading-tight">
                {lastrelease.changelog}
              </h2>
            </div>
            <Button className="h-12 rounded-xl font-bold hover:translate-x-2 transition-transform" variant='primary' onClick={() => navigate('/diffs')}>
              对比文件
              <ArrowRight />
            </Button>
          </div>

          <Separator />
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-6 pt-6">
            <div className="flex items-center gap-3 ">
              <div className="p-2 bg-slate-800 rounded-lg">
                <User size={20} className=" text-muted" />
              </div>
              <div>
                <p className="text-xl text-muted uppercase font-bold tracking-wider">作者</p>
                <p className="font-medium">{"雪龙花❄"}</p>
              </div>
            </div>
            <div className="flex items-center gap-3 overflow-hidden">
              <div className="p-2 bg-slate-800 rounded-lg">
                <Calendar size={20} className=" text-muted" />
              </div>
              <div>
                <p className="text-xl text-muted uppercase font-bold tracking-wider">日期</p>
                <p className="font-medium text-sm text-ellipsis text-nowrap">{lastrelease.date}</p>
              </div>
            </div>

          </div>
        </div>

        <Separator />
        {/* Quick Stats Banner */}
        <div className="p-6 grid grid-cols-3 divide-x">
          <div className="text-center">
            <div className="flex items-center justify-center gap-2 mb-1">
              <FileCode size={16} className=" text-muted" />
              <span className="text-2xl font-bold ">{formatBytes(lastrelease.size, true)}</span>
            </div>
            <span className="text-xl text-muted uppercase font-bold tracking-widest">大小</span>
          </div>
          <div className="text-center">
            <div className="flex items-center justify-center gap-2 mb-1">
              <PlusCircle size={16} className="text-emerald-500" />
              <span className="text-2xl font-bold text-emerald-500">+ {lastrelease.adds?.length || 0}</span>
            </div>
            <span className="text-xl text-muted uppercase font-bold tracking-widest">新增</span>
          </div>
          <div className="text-center">
            <div className="flex items-center justify-center gap-2 mb-1">
              <MinusCircle size={16} className="text-rose-500" />
              <span className="text-2xl font-bold text-rose-500">- {lastrelease.subs?.length || 0}</span>
            </div>
            <span className="text-xl text-muted uppercase font-bold tracking-widest">删减</span>
          </div>
        </div>
      </div>

      {/* Actions Row */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <button className="flex items-center justify-between p-6 bg-background-secondary border  rounded-xl hover:bg-background-tertiary transition-all group">
          <div className="text-left">
            <h3 className="font-semibold ">单文件下载在实用功能中</h3>
            <p className="text-sm text-muted">服务器列表、键位等</p>
          </div>
          <div className="bg-slate-800 p-2 rounded-lg group-hover:bg-blue-600/20 transition-colors">
            <Hash size={20} className="text-muted group-hover:text-blue-400" />
          </div>
        </button>
        <button className="flex items-center justify-between p-6  bg-background-secondary border  rounded-xl hover:bg-background-tertiary transition-all group">
          <div className="text-left">
            <h3 className="font-semibold ">移除的MOD有备份</h3>
            <p className="text-sm text-muted">移除的mod在 .minecraft/bakmods 中</p>
          </div>
          <div className="bg-slate-800 p-2 rounded-lg group-hover:bg-purple-600/20 transition-colors">
            <Hash size={20} className="text-muted group-hover:text-purple-400" />
          </div>
        </button>
      </div>
    </div>
  );
};

export default Dashboard;
