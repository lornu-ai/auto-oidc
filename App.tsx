
import React, { useState, useMemo } from 'react';
import { GoogleGenAI } from "@google/genai";
import { COMPONENTS } from './constants';
import { HubType } from './types';

const ShieldIcon = () => (
  <svg className="w-5 h-5 text-sky-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
  </svg>
);

const CodeIcon = () => (
  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
  </svg>
);

const App: React.FC = () => {
  const [activeHub, setActiveHub] = useState<HubType>(HubType.OIDC);
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>('worker');
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [aiInsight, setAiInsight] = useState<string | null>(null);
  const [viewMode, setViewMode] = useState<'map' | 'code'>('map');

  const selectedNode = useMemo(() => {
    return Object.values(COMPONENTS).find(n => n.id === selectedNodeId);
  }, [selectedNodeId]);

  const handleAnalyze = async () => {
    setIsAnalyzing(true);
    setAiInsight(null);
    try {
      const ai = new GoogleGenAI({ apiKey: process.env.API_KEY || '' });
      const response = await ai.models.generateContent({
        model: 'gemini-3-flash-preview',
        contents: `Explain the security advantages of using a Cloudflare Worker as a Token Exchange Relay for ${activeHub}. Specifically, how it enables sovereign identity across AWS, GCP, and Azure without a traditional VPN.`,
      });
      setAiInsight(response.text);
    } catch (error) {
      setAiInsight("Security audit failed. Check network connection.");
    } finally {
      setIsAnalyzing(false);
    }
  };

  const workerCode = `// OIDC Hub Implementation
export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    if (url.pathname === "/.well-known/jwks.json") {
      return Response.json({ keys: [...] });
    }
    // Token Exchange Logic
    const token = await exchange(request.headers.get("Auth"));
    return Response.json({ sovereign_token: token });
  }
};`;

  return (
    <div className="min-h-screen bg-[#020617] text-slate-100 flex flex-col font-sans selection:bg-sky-500/30">
      <div className="fixed inset-0 bg-[linear-gradient(to_right,#1e293b_1px,transparent_1px),linear-gradient(to_bottom,#1e293b_1px,transparent_1px)] bg-[size:4rem_4rem] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)] pointer-events-none opacity-20" />

      <nav className="relative z-10 border-b border-slate-800 bg-slate-950/50 backdrop-blur-md px-8 py-4 flex justify-between items-center">
        <div className="flex items-center gap-3">
          <ShieldIcon />
          <div>
            <h1 className="text-xl font-bold tracking-tight text-white uppercase">Sovereign Relay</h1>
            <p className="text-[9px] font-mono text-sky-400 uppercase tracking-[0.2em]">Distributed OIDC Trust Engine</p>
          </div>
        </div>

        <div className="flex bg-slate-900/80 rounded-full p-1 border border-slate-800 gap-1">
          <button 
            onClick={() => setViewMode('map')}
            className={`px-4 py-1.5 rounded-full text-[10px] font-bold transition-all flex items-center gap-2 ${viewMode === 'map' ? 'bg-sky-500 text-white shadow-lg' : 'text-slate-400'}`}
          >
            TOPOLOGY
          </button>
          <button 
            onClick={() => setViewMode('code')}
            className={`px-4 py-1.5 rounded-full text-[10px] font-bold transition-all flex items-center gap-2 ${viewMode === 'code' ? 'bg-sky-500 text-white shadow-lg' : 'text-slate-400'}`}
          >
            <CodeIcon /> WORKER_LOGIC
          </button>
        </div>
      </nav>

      <main className="relative z-10 flex-1 p-8 grid grid-cols-1 lg:grid-cols-12 gap-8 max-w-[1600px] mx-auto w-full">
        <section className="lg:col-span-8 flex flex-col gap-4">
          <div className="flex justify-between items-center px-2">
            <h2 className="text-xs font-mono text-slate-400 uppercase tracking-widest flex items-center gap-2">
              <span className="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse"></span> Endpoint: {activeHub}
            </h2>
            <div className="flex gap-2">
              <button onClick={() => setActiveHub(HubType.OIDC)} className={`text-[9px] font-bold px-3 py-1 rounded border ${activeHub === HubType.OIDC ? 'bg-white text-black' : 'border-slate-800 text-slate-500'}`}>OIDC</button>
              <button onClick={() => setActiveHub(HubType.AUTOMATION)} className={`text-[9px] font-bold px-3 py-1 rounded border ${activeHub === HubType.AUTOMATION ? 'bg-white text-black' : 'border-slate-800 text-slate-500'}`}>AUTO</button>
            </div>
          </div>
          
          <div className="flex-1 glass rounded-[2.5rem] border border-slate-800 flex items-center justify-center p-8 relative overflow-hidden">
            {viewMode === 'map' ? (
              <svg viewBox="0 0 950 400" className="w-full h-auto drop-shadow-2xl">
                <defs>
                  <filter id="glow" x="-20%" y="-20%" width="140%" height="140%">
                    <feGaussianBlur stdDeviation="4" result="blur" />
                    <feComposite in="SourceGraphic" in2="blur" operator="over" />
                  </filter>
                </defs>
                <g className="opacity-20">
                  <path d="M 160 220 L 760 220" stroke="#38bdf8" strokeWidth="1" fill="none" strokeDasharray="8 8" />
                </g>
                {[
                  { id: 'user', x: 100, y: 220, icon: '👤', label: 'Identity' },
                  { id: 'zt', x: 320, y: 220, icon: '🛡️', label: 'Access' },
                  { id: 'iam', x: 320, y: 80, icon: '🔑', label: 'IAM' },
                  { id: 'worker', x: 580, y: 220, icon: '⚡', label: 'Relay' },
                  { id: 'dest', x: 820, y: 220, icon: '☁️', label: 'Cloud' },
                ].map((n) => (
                  <g key={n.id} className="cursor-pointer" onClick={() => setSelectedNodeId(n.id === 'dest' ? 'automation_hub' : n.id)}>
                    <circle cx={n.x} cy={n.y} r="45" className={`transition-all duration-300 ${selectedNodeId === n.id ? 'fill-sky-500/20 stroke-sky-400 stroke-2' : 'fill-slate-900 stroke-slate-800'}`} filter={selectedNodeId === n.id ? 'url(#glow)' : ''} />
                    <text x={n.x} y={n.y - 5} textAnchor="middle" className="text-xl pointer-events-none">{n.icon}</text>
                    <text x={n.x} y={n.y + 20} textAnchor="middle" className="fill-slate-400 text-[9px] font-bold uppercase tracking-[0.1em] pointer-events-none">{n.label}</text>
                  </g>
                ))}
              </svg>
            ) : (
              <div className="w-full h-full p-8 font-mono text-sm overflow-auto text-sky-300">
                <pre className="animate-in fade-in slide-in-from-left-4 duration-500">
                  {workerCode}
                </pre>
              </div>
            )}
          </div>
        </section>

        <section className="lg:col-span-4 flex flex-col gap-6">
          <div className="glass rounded-[2rem] border border-slate-800 p-8">
            <h3 className="text-[10px] font-black text-sky-500 uppercase tracking-widest mb-6">Component Deep Dive</h3>
            {selectedNode ? (
              <div className="animate-in fade-in duration-300">
                <h4 className="text-2xl font-bold text-white mb-2">{selectedNode.name}</h4>
                <p className="text-slate-400 text-sm leading-relaxed mb-6 font-light">{selectedNode.description}</p>
                <div className="space-y-2">
                  <div className="flex justify-between text-[10px] font-mono border-b border-slate-800/50 pb-2">
                    <span className="text-slate-500">PROTOCOL</span>
                    <span className="text-sky-400">OIDC / JWT_RELAY</span>
                  </div>
                  <div className="flex justify-between text-[10px] font-mono border-b border-slate-800/50 pb-2">
                    <span className="text-slate-500">RUNTIME</span>
                    <span className="text-sky-400">CLOUDFLARE_V8</span>
                  </div>
                </div>
              </div>
            ) : (
              <p className="text-slate-500 text-xs italic">Select architectural node...</p>
            )}
          </div>

          <div className="glass rounded-[2rem] border border-slate-800 p-8 bg-gradient-to-br from-slate-900 to-black">
            <div className="flex justify-between items-center mb-6">
              <h3 className="text-[10px] font-black text-emerald-500 uppercase tracking-widest">Sovereign Audit</h3>
              <button 
                onClick={handleAnalyze} 
                disabled={isAnalyzing}
                className="text-[9px] font-bold px-4 py-1 bg-emerald-500 text-black rounded-full hover:bg-emerald-400 transition-colors disabled:opacity-50"
              >
                {isAnalyzing ? 'AUDITING...' : 'RUN AUDIT'}
              </button>
            </div>
            <div className="text-sm text-slate-300 font-light leading-relaxed max-h-[300px] overflow-y-auto pr-2 custom-scrollbar">
              {aiInsight || "Initiate Gemini-3 Security Review to evaluate the sovereign identity layer."}
            </div>
          </div>
        </section>
      </main>

      <footer className="p-8 border-t border-slate-900 text-[9px] font-mono text-slate-600 flex justify-between uppercase tracking-[0.2em]">
        <span>SYS_CORE: 0xFF91 // ZONE: GLOBAL_EDGE</span>
        <span className="text-sky-500/50">Federating trust across the decentralized cloud</span>
      </footer>

      <style>{`
        .glass { background: rgba(15, 23, 42, 0.4); backdrop-filter: blur(24px); }
        .custom-scrollbar::-webkit-scrollbar { width: 4px; }
        .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(56, 189, 248, 0.2); border-radius: 10px; }
      `}</style>
    </div>
  );
};

export default App;
