
import React from 'react';
import { Github, Code2, Cpu, Shield } from 'lucide-react';

import { RuntimeContext } from '../runtimecontext';

const About: React.FC = () => {
    const runtime = React.useContext(RuntimeContext);
    console.log("Runtime in About:", runtime);

    return (
        <div className="max-w-4xl mx-auto space-y-12 animate-in fade-in zoom-in-95 duration-500">
            <section className="text-center space-y-4">
                <div className="inline-block p-4 bg-blue-600/20 rounded-3xl mb-4 border border-blue-500/20">
                    <Github className="text-blue-400" size={48} />
                </div>
                <h1 className="text-4xl font-extrabold text-white">GitLens Pro Visualizer</h1>
                <p className="text-xl text-slate-400 max-w-2xl mx-auto">
                    A high-performance git diff inspection tool designed for developers who demand depth in their version control insights.
                </p>
            </section>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
                <div className="p-8 bg-slate-900 border border-slate-800 rounded-2xl space-y-4">
                    <div className="p-3 bg-emerald-500/10 w-fit rounded-xl border border-emerald-500/20">
                        <Code2 className="text-emerald-400" size={24} />
                    </div>
                    <h3 className="text-xl font-bold text-white">Advanced Diffs</h3>
                    <p className="text-slate-400 text-sm leading-relaxed">
                        Analyze line-by-line changes with syntax highlighting and semantic grouping for better code reviews.
                    </p>
                </div>
                <div className="p-8 bg-slate-900 border border-slate-800 rounded-2xl space-y-4">
                    <div className="p-3 bg-purple-500/10 w-fit rounded-xl border border-purple-500/20">
                        <Shield className="text-purple-400" size={24} />
                    </div>
                    <h3 className="text-xl font-bold text-white">Integrity Checks</h3>
                    <p className="text-slate-400 text-sm leading-relaxed">
                        Automatic MD5 and SHA-256 checksum comparison ensures your binary and text files are exactly what you expect.
                    </p>
                </div>
                <div className="p-8 bg-slate-900 border border-slate-800 rounded-2xl space-y-4">
                    <div className="p-3 bg-blue-500/10 w-fit rounded-xl border border-blue-500/20">
                        <Cpu className="text-blue-400" size={24} />
                    </div>
                    <h3 className="text-xl font-bold text-white">Zero Overhead</h3>
                    <p className="text-slate-400 text-sm leading-relaxed">
                        Optimized parsing engine handles massive repositories with thousands of changed files without breaking a sweat.
                    </p>
                </div>
            </div>

            <footer className="pt-12 border-t border-slate-800 text-center">
                <p className="text-slate-500 text-sm font-medium">ModSync {runtime.version}</p>
                <p className="text-slate-600 text-xs mt-1">{runtime.buildinfo}</p>
                <p className="text-slate-600 text-xs mt-1">Developed with React & Tailwind</p>
            </footer>
        </div>
    );
};

export default About;
