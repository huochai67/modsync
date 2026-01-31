
import React, { useState, useEffect, useCallback } from 'react';
import { ReleaseInfo, FormState, MSConfig } from './types';
import HistoryItem from './components/HistoryItem';
import { invoke } from '@tauri-apps/api/core';

const App: React.FC = () => {
    // State
    const [form, setForm] = useState<FormState>({
        version: '1.0',
        changelog: '无可奉告',
        serverUrl: 'http://127.0.0.1:8086/',
        adds: [],
        subs: [],
        mods: []
    });

    const [history, setHistory] = useState<ReleaseInfo[]>([]);
    const [isLoadingHistory, setIsLoadingHistory] = useState(true);
    // Load History
    const fetchHistoryData = useCallback(async () => {
        setIsLoadingHistory(true);
        try {
            const config = await invoke<MSConfig | null>('get_config');
            setForm({ ...form, serverUrl: config?.base_url || form.serverUrl });
            setHistory(config?.release_info || []);
            console.log("Config fetched successfully.", config);
        } catch (error) {
            alert("Failed to load history : " + error);
        } finally {
            setIsLoadingHistory(false);
        }


    }, []);
    useEffect(() => {
        fetchHistoryData();
    }, []);


    const [isSubmitting, setIsSubmitting] = useState(false);
    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!form.version || !form.changelog) {
            alert("Please fill in version and changelog");
            return;
        }

        setIsSubmitting(true);
        try {
            await invoke('generate', { version: form.version, changelog: form.changelog, title: form.version, serverurl: form.serverUrl });
            alert("Update published successfully!");
            window.location.reload();
        } catch (err) {
            alert("Submission failed!" + err);
        } finally {
            setIsSubmitting(false);
        }
    };

    return (
        <div className="min-h-screen bg-slate-50 flex flex-col md:flex-row">
            {/* Sidebar / Form Area */}
            <div className="w-full md:w-112.5 bg-white border-r border-slate-200 p-6 flex flex-col max-h-screen sticky top-0 overflow-y-auto">
                <div className="mb-8">
                    <h1 className="text-2xl font-bold text-slate-900 flex items-center">
                        <svg className="w-8 h-8 mr-2 text-indigo-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
                        </svg>
                        ModSync Configurator <span className="text-indigo-600 font-bold ml-1">Next</span>
                    </h1>
                    <p className="text-slate-500 text-sm mt-1">Version update information generator</p>
                </div>

                <form onSubmit={handleSubmit} className="space-y-6">
                    <div>
                        <label className="block text-sm font-semibold text-slate-700 mb-1">Base URL</label>
                        <input
                            type="url"
                            value={form.serverUrl}
                            onChange={(e) => setForm({ ...form, serverUrl: e.target.value })}
                            className="w-full px-3 py-2 border border-slate-300 rounded-lg text-sm focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 outline-none transition-all"
                            placeholder="https://..."
                        />
                    </div>

                    <div className="grid grid-cols-2 gap-4">
                        <div>
                            <label className="block text-sm font-semibold text-slate-700 mb-1">New Version</label>
                            <input
                                type="text"
                                value={form.version}
                                onChange={(e) => setForm({ ...form, version: e.target.value })}
                                className="w-full px-3 py-2 border border-slate-300 rounded-lg text-sm focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 outline-none transition-all"
                                placeholder="e.g. v2.1.0"
                            />
                        </div>
                        <div>
                            <label className="block text-sm font-semibold text-slate-700 mb-1">Release Date</label>
                            <input
                                type="text"
                                value={new Date().toLocaleDateString()}
                                disabled
                                className="w-full px-3 py-2 bg-slate-50 border border-slate-200 text-slate-400 rounded-lg text-sm cursor-not-allowed"
                            />
                        </div>
                    </div>

                    <div>
                        <label className="block text-sm font-semibold text-slate-700 mb-1">Update Description (Changelog)</label>
                        <textarea
                            rows={3}
                            value={form.changelog}
                            onChange={(e) => setForm({ ...form, changelog: e.target.value })}
                            className="w-full px-3 py-2 border border-slate-300 rounded-lg text-sm focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 outline-none transition-all resize-none"
                            placeholder="What's new in this release?"
                        />
                    </div>

                    <button
                        type="submit"
                        disabled={isSubmitting}
                        className={`w-full py-3 rounded-xl font-bold text-white transition-all shadow-lg ${isSubmitting ? 'bg-indigo-400 cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700 shadow-indigo-200'}`}
                    >
                        {isSubmitting ? (
                            <span className="flex items-center justify-center">
                                <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
                                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                </svg>
                                Publishing Update...
                            </span>
                        ) : 'Push New Release'}
                    </button>
                </form>

                <div className="mt-auto pt-6 border-t border-slate-100 text-[10px] text-slate-400 uppercase tracking-widest font-bold">
                    System Connected • Status: Online
                </div>
            </div>

            {/* Main Content Area / Visualization */}
            <div className="flex-1 p-6 md:p-10">
                <div className="max-w-4xl mx-auto">
                    <header className="flex items-center justify-between mb-8">
                        <div>
                            <h2 className="text-xl font-bold text-slate-900">Release History</h2>
                            <p className="text-slate-500 text-sm">Visualizing past changes and deployment records</p>
                        </div>
                        <div className="flex space-x-2">
                            <div className="bg-white border border-slate-200 px-4 py-2 rounded-lg shadow-sm flex items-center">
                                <span className="w-2 h-2 bg-green-500 rounded-full mr-2"></span>
                                <span className="text-xs font-medium text-slate-600">{history[history.length - 1]?.version || '1.0.0'}</span>
                            </div>
                        </div>
                    </header>

                    {isLoadingHistory ? (
                        <div className="space-y-4">
                            {[1, 2, 3].map(i => (
                                <div key={i} className="bg-white p-6 rounded-lg shadow-sm border border-slate-100 animate-pulse">
                                    <div className="h-4 bg-slate-100 rounded w-1/4 mb-4"></div>
                                    <div className="h-3 bg-slate-100 rounded w-3/4 mb-2"></div>
                                    <div className="h-3 bg-slate-100 rounded w-1/2"></div>
                                </div>
                            ))}
                        </div>
                    ) : (
                        <div className="relative">
                            {/* Timeline Line */}
                            <div className="absolute left-6.5 top-6 bottom-6 w-0.5 bg-slate-200 hidden md:block"></div>

                            <div className="space-y-6">
                                {history.length === 0 ? (
                                    <div className="text-center py-20 bg-white rounded-2xl border-2 border-dashed border-slate-200">
                                        <p className="text-slate-400">No releases found in history.</p>
                                    </div>
                                ) : (
                                    history.map((release, idx) => (
                                        <div key={idx} className="relative md:pl-14">
                                            {/* Timeline Dot */}
                                            <div className="absolute left-4.75 top-4 w-4 h-4 rounded-full border-4 border-slate-50 bg-indigo-500 hidden md:block z-10"></div>
                                            <HistoryItem release={release} />
                                        </div>
                                    ))
                                )}
                            </div>
                        </div>
                    )}

                    <footer className="mt-12 text-center text-slate-400 text-sm">
                        <p>© 2024 Version Update Generator Interface.</p>
                    </footer>
                </div>
            </div>
        </div>
    );
};

export default App;
