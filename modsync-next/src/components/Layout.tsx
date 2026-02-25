import React, { useCallback, useEffect } from "react";
import { useNavigate, useLocation } from "react-router-dom";
import {
  LayoutDashboard,
  FileDiff,
  Settings2,
  List,
  Layers,
} from "lucide-react";

import { invoke } from "@tauri-apps/api/core";
import { Window } from "@tauri-apps/api/window";
import { openUrl } from "@tauri-apps/plugin-opener";
import { Button, Separator } from "@heroui/react";

import { RuntimeInfo } from "@/types";
import { RuntimeContext } from "@/runtimecontext";
import { MOCK_RUNTIME_INFO } from "@/mockData";

import { SiQq } from "@icons-pack/react-simple-icons";

interface LayoutProps {
  children: React.ReactNode;
}
const Layout: React.FC<LayoutProps> = ({ children }) => {
  const [runtime, setruntime] = React.useState<RuntimeInfo>(MOCK_RUNTIME_INFO);
  const [initialized, setInitialized] = React.useState(false);

  // Load Runtime
  const fetchRuntime = useCallback(async () => {
    setInitialized(false);
    try {
      const initialized = await invoke<boolean>("is_init");
      if (!initialized) {
        await invoke<void>("init_runtime");
      }

      const runtime = await invoke<RuntimeInfo>("get_runtime");
      console.log("Runtime Info:", runtime);
      Window.getCurrent().setTitle(`${runtime.title}`);
      setruntime(runtime);
    } catch (error) {
      alert("Failed to load runtime : " + error);
    } finally {
      setInitialized(true);
    }
  }, []);
  useEffect(() => {
    fetchRuntime();
  }, []);

  const [is_syncing, setIsSyncing] = React.useState<boolean>(false);
  useEffect(() => {
    setInterval(async () => {
      try {
        let is_syncing = await invoke<boolean>("is_running");
        console.log(is_syncing);
        setIsSyncing(is_syncing);
      } catch (error) {
        alert("Failed to fetch TaskInfo: " + error);
        //exit here
      }
    }, 1000); // 1000 milliseconds = 1 second
  }, []);

  const navigate = useNavigate();
  const location = useLocation();

  const navItems = [
    { label: "首页", icon: <LayoutDashboard size={20} />, path: "/" },
    {
      label: "文件差异",
      icon: <FileDiff size={20} />,
      path: "/diffs",
      isDisabled: is_syncing,
    },
    { label: "任务管理", icon: <List size={20} />, path: "/taskmanager" },
    { label: "实用功能", icon: <Settings2 size={20} />, path: "/utilities" },
    { label: "更新日志", icon: <List size={20} />, path: "/changelog" },
    // { label: '关于', icon: <Info size={20} />, path: '/about' },
    // { label: '测试页面', icon: <ShipWheel size={20} />, path: '/test' },
  ];

  return (
    <div className="flex h-screen overflow-hidden ">
      {/* Sidebar */}
      <aside className="w-64 border-r border bg-background-secondary flex flex-col">
        <div className="p-6 border-b flex items-center gap-3">
          <div className="bg-accent p-2 rounded-lg">
            <Layers className="text-white" size={24} />
          </div>
          <span className="font-bold text-xl tracking-tight text-white">
            MS更新器
          </span>
        </div>

        <nav className="flex-1 p-4 space-y-2 overflow-y-auto">
          {navItems.map((item) => (
            <Button
              isDisabled={item.isDisabled}
              variant="ghost"
              key={item.path}
              onClick={() => navigate(item.path)}
              className={`w-full flex items-center gap-3 px-4 py-3 rounded-lg transition-all duration-200 ${
                location.pathname === item.path
                  ? "bg-accent-soft text-accent border"
                  : "text-muted"
              }`}
            >
              {item.icon}
              <span className="font-medium">{item.label}</span>
            </Button>
          ))}
        </nav>

        <Separator />
        <div className="p-4">
          <Button
            className="w-full h-10"
            variant="outline"
            onClick={() => {
              openUrl("https://qm.qq.com/q/dIp82HmC6Q");
            }}
          >
            <SiQq size={18} />
            加入QQ群
          </Button>
        </div>
      </aside>

      {/* Main Content */}
      <main className="flex-1 flex flex-col overflow-hidden relative">
        {/* Top Header Decor */}
        {/* <div className="absolute top-0 left-0 right-0 h-1 bg-linear-to-r from-blue-600 via-indigo-600 to-purple-600"></div> */}
        <div className="flex-1 overflow-y-auto p-8">
          <RuntimeContext.Provider value={runtime}>
            {initialized ? (
              children
            ) : (
              <div className="flex items-center justify-center h-full">
                <div className="text-accent">
                  正在初始化运行时环境，请稍候...
                </div>
              </div>
            )}
          </RuntimeContext.Provider>
        </div>
      </main>
    </div>
  );
};

export default Layout;
