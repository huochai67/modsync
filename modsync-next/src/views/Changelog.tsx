import React from "react";

import { RuntimeContext } from "../runtimecontext";
import HistoryItem from "@/components/HistoryItem";
import { Chip } from "@heroui/react";
import { CircleCheck } from "lucide-react";

const Changelog: React.FC = () => {
  const runtime = React.useContext(RuntimeContext);
  const history = runtime.release_info;
  return (
    <div className="page-wrap">
      <div>
        <header className="flex items-end justify-between mb-8">
          <div>
            <div className="eyebrow">Release history</div><h1 className="page-heading">更新日志</h1>
            <p className="page-description">浏览客户端过去的更改和发布记录。</p>
          </div>
          <Chip
            color="success"
            className="flex flex-row items-center mb-8 p-2 h-6"
          >
            <CircleCheck width={12} />
            {history[history.length - 1]?.version || "1.0.0"}
          </Chip>
        </header>

        <div className="relative">
          {/* Timeline Line */}
          <div className="absolute left-6.5 top-6 bottom-6 w-0.5 bg-foreground hidden md:block"></div>

          <div className="space-y-6">
            {history.length === 0 ? (
              <div className="text-center py-20 rounded-2xl border-2 border-dashed">
                <p className="text-muted">No releases found in history.</p>
              </div>
            ) : (
              history.map((release, idx) => (
                <div key={idx} className="relative md:pl-14">
                  {/* Timeline Dot */}
                  <div className="absolute left-4.75 top-4 w-4 h-4 rounded-full border-4 border-foreground bg-accent hidden md:block z-10"></div>
                  <HistoryItem release={release} />
                </div>
              ))
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default Changelog;
