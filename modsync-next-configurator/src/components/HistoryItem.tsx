
import React, { useState } from 'react';
import { ReleaseInfo } from '../types';
import Badge from './Badge';

interface HistoryItemProps {
    release: ReleaseInfo;
}

export const formatBytes = (bytes?: number) => {
    if (!bytes || bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

const HistoryItem: React.FC<HistoryItemProps> = ({ release }) => {
    const [expanded, setExpanded] = useState(false);

    return (
        <div className="bg-white rounded-lg shadow-sm border border-slate-200 overflow-hidden mb-4 transition-all hover:border-indigo-300">
            <div
                className="p-4 cursor-pointer flex items-center justify-between"
                onClick={() => setExpanded(!expanded)}
            >
                <div className="flex items-center space-x-4">
                    <div className="bg-indigo-50 p-2 rounded-lg">
                        <span className="text-indigo-600 font-bold text-sm tracking-tight">{release.version}</span>
                    </div>
                    <div>
                        <h3 className="text-slate-900 font-semibold text-sm truncate max-w-xs">{release.changelog}</h3>
                        <p className="text-slate-400 text-xs">{release.date}</p>
                    </div>
                </div>

                <div className="flex items-center space-x-2">
                    {release.adds && release.adds.length > 0 && <Badge type="add" count={release.adds.length} />}
                    {release.subs && release.subs.length > 0 && <Badge type="sub" count={release.subs.length} />}
                    {release.mods && release.mods.length > 0 && <Badge type="mod" count={release.mods.length} />}
                    <span className="text-slate-400 text-xs ml-2">{formatBytes(release.size)}</span>
                    <svg
                        className={`w-4 h-4 text-slate-400 transition-transform ${expanded ? 'rotate-180' : ''}`}
                        fill="none" viewBox="0 0 24 24" stroke="currentColor"
                    >
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
                    </svg>
                </div>
            </div>

            {expanded && (
                <div className="px-4 pb-4 pt-2 border-t border-slate-50 bg-slate-50/50">
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-xs mt-2">
                        {release.adds && release.adds.length > 0 && (
                            <div>
                                <h4 className="font-bold text-green-700 mb-1 flex items-center">
                                    <span className="mr-1">Added</span>
                                    <span className="text-[10px] bg-green-200 px-1 rounded">{release.adds.length}</span>
                                </h4>
                                <ul className="text-slate-600 space-y-1">
                                    {release.adds.map((f, i) => <li key={i} className="truncate">📄 {f}</li>)}
                                </ul>
                            </div>
                        )}
                        {release.mods && release.mods.length > 0 && (
                            <div>
                                <h4 className="font-bold text-blue-700 mb-1 flex items-center">
                                    <span className="mr-1">Modified</span>
                                    <span className="text-[10px] bg-blue-200 px-1 rounded">{release.mods.length}</span>
                                </h4>
                                <ul className="text-slate-600 space-y-1">
                                    {release.mods.map((f, i) => <li key={i} className="truncate">🔧 {f}</li>)}
                                </ul>
                            </div>
                        )}
                        {release.subs && release.subs.length > 0 && (
                            <div>
                                <h4 className="font-bold text-red-700 mb-1 flex items-center">
                                    <span className="mr-1">Removed</span>
                                    <span className="text-[10px] bg-red-200 px-1 rounded">{release.subs.length}</span>
                                </h4>
                                <ul className="text-slate-600 space-y-1">
                                    {release.subs.map((f, i) => <li key={i} className="truncate">🗑️ {f}</li>)}
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
