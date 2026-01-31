
import React, { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button, Chip, FieldError, Form, Input, Label, Separator, TextArea, TextField } from '@heroui/react';
import { CircleCheck } from 'lucide-react';

import { FormState, MSConfig } from '@/types';
import HistoryItem from '@/components/HistoryItem';

const App: React.FC = () => {
    // State
    const [form, setForm] = useState<FormState>({
        version: '',
        title: '',
        changelog: '',
        serverUrl: '',
        adds: [],
        subs: [],
        mods: []
    });

    const [config, setConfig] = useState<MSConfig>({
        base_url: '',
        release_info: [],
        title: ''
    });
    const history = config.release_info;
    const [_isLoadingHistory, setIsLoadingHistory] = useState(true);
    // Load Config
    const fetchConfig = useCallback(async () => {
        setIsLoadingHistory(true);
        try {
            const config = await invoke<MSConfig>('get_config');
            setConfig(config)
            const lastrelease = config.release_info[config.release_info.length - 1];
            setForm({ ...form, title: config.title, serverUrl: config.base_url, changelog: lastrelease.changelog, version: lastrelease.version });
            console.log("Config fetched successfully.", config);
        } catch (error) {
            alert("Failed to load history : " + error);
        } finally {
            setIsLoadingHistory(false);
        }


    }, []);
    useEffect(() => {
        fetchConfig();
    }, []);


    const [isSubmitting, setIsSubmitting] = useState(false);
    const handleSubmit = async (e: React.FormEvent) => {
        console.log(e)
        e.preventDefault();
        if (!form.version || !form.changelog) {
            alert("Please fill in version and changelog");
            return;
        }

        setIsSubmitting(true);
        try {
            await invoke('generate', { version: form.version, changelog: form.changelog, title: form.title, serverurl: form.serverUrl });
            alert("Update published successfully!");
            window.location.reload();
        } catch (err) {
            alert("Submission failed!" + err);
        } finally {
            setIsSubmitting(false);
        }
    };

    return (
        <div className="min-h-screen bg-background flex flex-col md:flex-row">
            {/* Sidebar / Form Area */}
            <div className="w-1/3 bg-background-secondary border p-6 flex flex-col max-h-screen sticky top-0 overflow-y-auto">
                <div className="mb-8 overflow-hidden text-xs h-full">
                    <h1 className="text-2xl font-bold flex items-center">
                        <svg className="w-8 h-8 mr-2 text-accent" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
                        </svg>
                        MS Configurator <span className="text-accent font-bold ml-1">Next</span>
                    </h1>
                    <p className="text-muted text-sm mt-1">Version update information generator</p>
                </div>

                <Form onSubmit={handleSubmit} className="flex flex-col h-full space-y-6">
                    <TextField
                        isRequired
                        name="baseural"
                        type="url"
                        value={form.serverUrl}
                        onChange={(e) => setForm({ ...form, serverUrl: e })}
                    >
                        <Label>Base URL</Label>
                        <Input placeholder='https://...' />
                        <FieldError />
                    </TextField>
                    <TextField
                        isRequired
                        name="title"
                        type="text"
                        defaultValue='MS-UPDATER'
                        value={form.title}
                        onChange={(e) => setForm({ ...form, title: e })}
                    >
                        <Label>Title</Label>
                        <Input placeholder='MS-UPDATER' />
                        <FieldError />
                    </TextField>

                    <div className="grid grid-cols-2 gap-4">
                        <TextField
                            isRequired
                            name="version"
                            type="text"
                            value={form.version}
                            onChange={(e) => setForm({ ...form, version: e })}
                        >
                            <Label>New Version</Label>
                            <Input placeholder="e.g. v2.1.0" />
                            <FieldError />
                        </TextField>
                        <TextField
                            isDisabled
                            name="date"
                            type="text"
                            value={new Date().toLocaleDateString()}
                            onChange={(e) => setForm({ ...form, version: e })}
                        >
                            <Label>Release Date</Label>
                            <Input placeholder="e.g. 2024-04-27" />
                            <FieldError />
                        </TextField>
                    </div>

                    <div className='flex flex-col h-fit'>
                        <Label>Update Description (Changelog)</Label>
                        <TextArea
                            rows={3}
                            value={form.changelog}
                            onChange={(e) => setForm({ ...form, changelog: e.target.value })}
                            placeholder="What's new in this release?"
                        />
                    </div>

                    <div className='h-full'/>

                    <Button isPending={isSubmitting} className="min-h-12 w-full self-center" type='submit' >{isSubmitting ? (
                        <span className="flex items-center justify-center">
                            <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-foreground" fill="none" viewBox="0 0 24 24">
                                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                            </svg>
                            Publishing Update...
                        </span>
                    ) : 'Push New Release'}</Button>
                    <Separator />
                    <Label className='text-success'>System Connected • Status: Online</Label>
                </Form>
            </div>

            {/* Main Content Area / Visualization */}
            <div className="flex-1 p-6 md:p-10">
                <div className="max-w-4xl mx-auto">
                    <header className="flex items-center justify-between mb-8">
                        <div>
                            <h2 className="text-xl font-bold">更新日志</h2>
                            <p className="text-sm text-muted">可视化展示过去的更改和发布记录</p>
                        </div>
                        <Chip color="success" className='flex flex-row items-center mb-8 p-2 h-6'>
                            <CircleCheck width={12} />
                            {history[history.length - 1]?.version || '1.0.0'}
                        </Chip>
                    </header>

                    <div className="relative">
                        {/* Timeline Line */}
                        <div className="absolute left-6.5 top-6 bottom-6 w-0.5 bg-foreground hidden md:block"></div>

                        <div className="space-y-6">
                            {history.length === 0 ?
                                (
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
        </div>
    );
};

export default App;
