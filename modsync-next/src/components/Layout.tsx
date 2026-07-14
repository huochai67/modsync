import React, { useCallback, useEffect } from "react";
import { useNavigate, useLocation } from "react-router-dom";
import {
  LayoutDashboard,
  FileDiff,
  Settings2,
  List,
  Layers,
  Radio,
} from "lucide-react";

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { Window } from "@tauri-apps/api/window";
import { openUrl } from "@tauri-apps/plugin-opener";
import { Button } from "@heroui/react";

import { RuntimeInfo } from "@/types";
import { RuntimeContext } from "@/runtimecontext";
import { MOCK_RUNTIME_INFO } from "@/mockData";

import { SiQq } from "@icons-pack/react-simple-icons";
import { SYNC_STATE_EVENT } from "@modsync/contracts";

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
    let disposed = false;
    let unlisten: UnlistenFn | undefined;

    const subscribe = async () => {
      const stopListening = await listen<boolean>(SYNC_STATE_EVENT, (event) => {
        if (!disposed) setIsSyncing(event.payload);
      });
      if (disposed) {
        stopListening();
        return;
      }
      unlisten = stopListening;
      try {
        const isSyncing = await invoke<boolean>("is_running");
        if (!disposed) setIsSyncing(isSyncing);
      } catch (error) {
        console.error("Failed to fetch task state", error);
      }
    };

    subscribe().catch((error) => {
      console.error("Failed to subscribe to sync state", error);
    });
    return () => {
      disposed = true;
      unlisten?.();
    };
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
    <div className="app-shell flex h-screen overflow-hidden">
      {/* Sidebar */}
      <aside className="sidebar flex flex-col">
        <div className="brand">
          <div className="brand-mark">
            <Layers size={21} />
          </div>
          <div className="brand-copy"><div className="brand-name">MS 更新器</div><div className="brand-subtitle">MODSYNC NEXT</div></div>
        </div>

        <nav className="flex-1 overflow-y-auto">
          <div className="nav-section">工作台</div>
          {navItems.map((item) => (
            <Button
              isDisabled={item.isDisabled}
              variant="ghost"
              key={item.path}
              onClick={() => navigate(item.path)}
              className={`nav-link w-full flex items-center gap-3 transition-all duration-200 ${
                location.pathname === item.path
                  ? "active"
                  : ""
              }`}
            >
              {item.icon}
              <span className="nav-label font-medium">{item.label}</span>
            </Button>
          ))}
        </nav>

        <div className="community">
          <p className="community-note">遇到问题？欢迎加入社区交流</p>
          <Button
            className="w-full h-10 rounded-xl"
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
      <main className="main-area flex-1 flex flex-col overflow-hidden relative">
        <header className="topbar">
          <div><div className="topbar-kicker">MODSYNC / CLIENT</div><div className="topbar-title">管理你的 Minecraft 客户端文件</div></div>
          <div className="connection"><span className="connection-dot" /><span>{is_syncing ? "正在同步" : "同步服务就绪"}</span><Radio size={15} /></div>
        </header>
        <div className="page-content flex-1">
          <RuntimeContext.Provider value={runtime}>
            {initialized ? (
              children
            ) : (
              <div className="task-empty surface h-full">
                <div><div className="empty-icon"><Layers size={23} /></div><div className="font-semibold">正在连接同步服务</div><div className="page-description mt-2">正在初始化运行时环境，请稍候…</div></div>
              </div>
            )}
          </RuntimeContext.Provider>
        </div>
      </main>
    </div>
  );
};

export default Layout;
