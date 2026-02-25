import React, { useState } from "react";
import { ReleaseInfo } from "../types";
import { formatBytes } from "@/utils";
import { Chip } from "@heroui/react";

interface HistoryItemProps {
  release: ReleaseInfo;
}

const HistoryItem: React.FC<HistoryItemProps> = ({ release }) => {
  const [expanded, setExpanded] = useState(false);

  return (
    <div className="bg-background-tertiary rounded-lg shadow-sm border overflow-hidden mb-4 transition-all">
      <div
        className="p-4 cursor-pointer flex items-center justify-between"
        onClick={() => setExpanded(!expanded)}
      >
        <div className="flex items-center space-x-4">
          <div className="bg-accent-hover w-16 text-center overflow-hidden p-2 rounded-lg">
            <span className=" font-bold text-sm tracking-tight">
              {release.version}
            </span>
          </div>
          <div>
            <h3 className="font-semibold text-sm truncate max-w-xs">
              {release.changelog}
            </h3>
            <p className="text-muted text-xs">{release.date}</p>
          </div>
        </div>

        <div className="flex items-center space-x-2">
          {release.adds && release.adds.length > 0 && (
            <Chip color="success">+{release.adds.length}</Chip>
          )}
          {release.subs && release.subs.length > 0 && (
            <Chip color="danger">-{release.subs.length}</Chip>
          )}
          {release.mods && release.mods.length > 0 && (
            <Chip color="accent">*{release.mods.length}</Chip>
          )}
          <span className="text-muted text-xs ml-2">
            {formatBytes(release.size)}
          </span>
          <svg
            className={`w-4 h-4 text-muted transition-transform ${expanded ? "rotate-180" : ""}`}
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M19 9l-7 7-7-7"
            />
          </svg>
        </div>
      </div>

      {expanded && (
        <div className="px-4 pb-4 pt-2 border-t bg-background-tertiary">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-xs mt-2">
            {release.adds && release.adds.length > 0 && (
              <div>
                <h4 className="font-bold mb-1 flex items-center">
                  <span className="mr-1 text-success">Added</span>
                  <span className="text-[10px] bg-success-hover px-1 rounded">
                    {release.adds.length}
                  </span>
                </h4>
                <ul className="text-muted space-y-1">
                  {release.adds.map((f, i) => (
                    <li key={i} className="truncate">
                      📄 {f}
                    </li>
                  ))}
                </ul>
              </div>
            )}
            {release.mods && release.mods.length > 0 && (
              <div>
                <h4 className="font-bold mb-1 flex items-center">
                  <span className="mr-1 text-accent">Modified</span>
                  <span className="text-[10px] bg-accent-hover px-1 rounded">
                    {release.mods.length}
                  </span>
                </h4>
                <ul className="text-muted space-y-1">
                  {release.mods.map((f, i) => (
                    <li key={i} className="truncate">
                      🔧 {f}
                    </li>
                  ))}
                </ul>
              </div>
            )}
            {release.subs && release.subs.length > 0 && (
              <div>
                <h4 className="font-bold mb-1 flex items-center">
                  <span className="mr-1 text-danger">Removed</span>
                  <span className="text-[10px] bg-danger-hover px-1 rounded">
                    {release.subs.length}
                  </span>
                </h4>
                <ul className="text-muted space-y-1">
                  {release.subs.map((f, i) => (
                    <li key={i} className="truncate">
                      🗑️ {f}
                    </li>
                  ))}
                </ul>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
};

export default HistoryItem;
