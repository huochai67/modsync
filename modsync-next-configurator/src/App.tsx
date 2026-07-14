import React, { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button, FieldError, Form, Input, Label, TextArea, TextField } from '@heroui/react';
import { CircleCheck, CloudUpload, Radio } from 'lucide-react';
import { FormState, MSConfig } from '@/types';
import HistoryItem from '@/components/HistoryItem';

const App: React.FC = () => {
  const [form, setForm] = useState<FormState>({ version: '', title: '', changelog: '', serverUrl: '', adds: [], subs: [], mods: [] });
  const [config, setConfig] = useState<MSConfig>({ base_url: '', release_info: [], title: '' });
  const [isSubmitting, setIsSubmitting] = useState(false);
  const history = config.release_info;
  const fetchConfig = useCallback(async () => {
    try { const loaded = await invoke<MSConfig>('get_config'); setConfig(loaded); const latest = loaded.release_info[loaded.release_info.length - 1]; setForm(current => ({ ...current, title: loaded.title, serverUrl: loaded.base_url, changelog: latest?.changelog ?? '', version: latest?.version ?? '' })); }
    catch (error) { alert('Failed to load history : ' + error); }
  }, []);
  useEffect(() => { fetchConfig(); }, [fetchConfig]);
  const handleSubmit = async (e: React.FormEvent) => { e.preventDefault(); if (!form.version || !form.changelog) { alert('Please fill in version and changelog'); return; } setIsSubmitting(true); try { await invoke('generate', { version: form.version, changelog: form.changelog, title: form.title, serverurl: form.serverUrl }); alert('Update published successfully!'); window.location.reload(); } catch (err) { alert('Submission failed!' + err); } finally { setIsSubmitting(false); } };
  return <div className="config-app flex h-screen overflow-hidden">
    <aside className="config-panel flex flex-col"><div className="config-brand"><div className="config-logo"><CloudUpload size={22}/></div><div><div className="config-kicker">MODSYNC NEXT</div><div className="config-title">发布配置器</div></div></div><p className="config-description">配置本次客户端更新内容，并生成可供更新器读取的发布信息。</p>
      <Form onSubmit={handleSubmit} className="form-card flex flex-col gap-5"><div className="form-caption">RELEASE DETAILS</div>
        <TextField isRequired name="baseurl" type="url" value={form.serverUrl} onChange={(value) => setForm({ ...form, serverUrl: value })}><Label>资源地址</Label><Input placeholder="https://..."/><FieldError/></TextField>
        <TextField isRequired name="title" type="text" value={form.title} onChange={(value) => setForm({ ...form, title: value })}><Label>客户端名称</Label><Input placeholder="MS-UPDATER"/><FieldError/></TextField>
        <div className="grid grid-cols-2 gap-3"><TextField isRequired name="version" type="text" value={form.version} onChange={(value) => setForm({ ...form, version: value })}><Label>新版本</Label><Input placeholder="v2.1.0"/><FieldError/></TextField><TextField isDisabled name="date" type="text" value={new Date().toLocaleDateString()}><Label>发布日期</Label><Input/></TextField></div>
        <div><Label>更新说明</Label><TextArea rows={4} value={form.changelog} onChange={(event) => setForm({ ...form, changelog: event.target.value })} placeholder="本次版本带来了什么？"/></div>
        <Button isPending={isSubmitting} className="publish-button mt-1" type="submit">{isSubmitting ? '正在发布…' : '发布新版本'}</Button>
      </Form><div className="status-box"><span className="status-dot"/><Radio size={14}/> 系统已连接 · 服务在线</div>
    </aside>
    <main className="history-pane flex-1"><div className="history-wrap"><header className="flex items-end justify-between"><div><div className="eyebrow">RELEASE HISTORY</div><h1 className="page-heading">发布记录</h1><p className="page-description">查看每次客户端更新的版本信息与变更详情。</p></div><div className="release-summary"><CircleCheck size={14}/>{history[history.length - 1]?.version || '暂无版本'}</div></header>
      <div className="timeline">{history.length === 0 ? <div className="text-center py-20 rounded-2xl border border-dashed text-muted">暂无发布记录。</div> : history.map((release, idx) => <div key={idx} className="timeline-row"><span className="timeline-dot"/><HistoryItem release={release}/></div>)}</div>
    </div></main>
  </div>;
};
export default App;
