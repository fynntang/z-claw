<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <script src="https://cdn.tailwindcss.com"></script>
    <script src="https://unpkg.com/lucide@latest"></script>
    <style>
        @import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,400;0,700;1,400&family=Inter:wght@400;500;600;700&display=swap');

        body {
            margin: 0;
            padding: 0;
            overflow: hidden;
            font-family: 'Inter', sans-serif;
            background-color: #0c0c0e;
            color: #d4d4d8;
        }

        .mono { font-family: 'JetBrains Mono', monospace; }

        .custom-scrollbar::-webkit-scrollbar {
            width: 4px;
        }
        .custom-scrollbar::-webkit-scrollbar-track {
            background: transparent;
        }
        .custom-scrollbar::-webkit-scrollbar-thumb {
            background: #27272a;
            border-radius: 10px;
        }

        .glass {
            background: rgba(12, 12, 14, 0.7);
            backdrop-filter: blur(12px);
        }

        .agent-gradient {
            background: linear-gradient(135deg, #2563eb 0%, #7c3aed 100%);
        }

        .active-nav {
            background: rgba(37, 99, 235, 0.1);
            border-left: 2px solid #3b82f6;
        }
    </style>
</head>
<body>
<div id="app" class="flex h-screen w-full overflow-hidden">

    <!-- 1. 最左侧导航：机器人入口 -->
    <aside class="w-16 flex flex-col items-center py-6 border-r border-zinc-800/50 bg-black/40 z-50">
        <div class="mb-8">
            <img src="https://avatars.githubusercontent.com/u/267654535" class="w-10 h-10 rounded-xl shadow-lg shadow-blue-500/10" alt="Logo" />
        </div>

        <nav class="flex flex-col gap-4 flex-1 items-center">
            <!-- 机器人列表 (模拟图标) -->
            <button class="w-10 h-10 rounded-full flex items-center justify-center bg-blue-600 text-white shadow-lg ring-2 ring-blue-500/20">
                <i data-lucide="bot" class="w-5 h-5"></i>
            </button>
            <button class="w-10 h-10 rounded-full flex items-center justify-center bg-zinc-800 text-zinc-400 hover:text-zinc-200 transition">
                <i data-lucide="cpu" class="w-5 h-5"></i>
            </button>
            <button class="w-10 h-10 rounded-full flex items-center justify-center bg-zinc-800 text-zinc-400 hover:text-zinc-200 transition">
                <i data-lucide="code" class="w-5 h-5"></i>
            </button>
            <div class="w-8 h-px bg-zinc-800 my-2"></div>
            <button class="w-10 h-10 rounded-full flex items-center justify-center border border-dashed border-zinc-700 text-zinc-600 hover:border-zinc-500 transition">
                <i data-lucide="plus" class="w-5 h-5"></i>
            </button>
        </nav>

        <div class="flex flex-col gap-5 mt-auto text-zinc-600">
            <button onclick="toggleSettings()" class="hover:text-white transition-colors"><i data-lucide="settings" class="w-6 h-6"></i></button>
        </div>
    </aside>

    <!-- 2. 二级侧栏：任务与频道 -->
    <section id="sidebar-secondary" class="w-64 border-r border-zinc-800/50 bg-[#0c0c0e]/50 flex flex-col">
        <div class="p-4 border-b border-zinc-800/50">
            <h1 class="font-bold text-sm tracking-tight text-white">ZClaw Dashboard</h1>
        </div>

        <div class="flex-1 overflow-y-auto custom-scrollbar p-3 space-y-6">
            <!-- 定时任务 -->
            <div>
                <div class="flex items-center justify-between px-2 mb-2">
                        <span class="text-[10px] font-bold text-zinc-500 uppercase tracking-widest flex items-center gap-1.5">
                            <i data-lucide="calendar-clock" class="w-3 h-3"></i> 定时任务
                        </span>
                    <button class="text-zinc-600 hover:text-blue-400"><i data-lucide="plus" class="w-3 h-3"></i></button>
                </div>
                <div class="space-y-1">
                    <div class="flex items-center gap-2 p-2 rounded-lg hover:bg-zinc-900/50 text-xs text-zinc-400 cursor-pointer">
                        <div class="w-1.5 h-1.5 rounded-full bg-emerald-500"></div>
                        <span class="truncate flex-1">每日代码清理</span>
                        <span class="text-[9px] text-zinc-600">09:00</span>
                    </div>
                    <div class="flex items-center gap-2 p-2 rounded-lg hover:bg-zinc-900/50 text-xs text-zinc-400 cursor-pointer">
                        <div class="w-1.5 h-1.5 rounded-full bg-zinc-600"></div>
                        <span class="truncate flex-1">周五报表导出</span>
                        <span class="text-[9px] text-zinc-600">Paused</span>
                    </div>
                </div>
            </div>

            <!-- IM 频道 -->
            <div>
                <div class="flex items-center justify-between px-2 mb-2">
                        <span class="text-[10px] font-bold text-zinc-500 uppercase tracking-widest flex items-center gap-1.5">
                            <i data-lucide="hash" class="w-3 h-3"></i> IM 频道
                        </span>
                    <button class="text-zinc-600 hover:text-blue-400"><i data-lucide="plus" class="w-3 h-3"></i></button>
                </div>
                <div class="space-y-1">
                    <div class="flex items-center gap-2 p-2 rounded-lg bg-zinc-800/40 text-xs text-white cursor-pointer">
                        <i data-lucide="slack" class="w-3 h-3 text-purple-400"></i>
                        <span class="truncate flex-1"># dev-ops-monitoring</span>
                    </div>
                    <div class="flex items-center gap-2 p-2 rounded-lg hover:bg-zinc-900/50 text-xs text-zinc-400 cursor-pointer">
                        <i data-lucide="message-circle" class="w-3 h-3 text-blue-400"></i>
                        <span class="truncate flex-1">Telegram: Alpha-Group</span>
                    </div>
                </div>
            </div>

            <!-- 会话历史 -->
            <div>
                <div class="px-2 mb-2 text-[10px] font-bold text-zinc-500 uppercase tracking-widest">最近会话</div>
                <div class="space-y-1" id="session-list">
                    <!-- 动态加载 -->
                </div>
            </div>
        </div>
    </section>

    <!-- 3. 主交互区 -->
    <main class="flex-1 flex flex-col relative bg-[#09090b]">
        <!-- 聊天头部 -->
        <header class="h-14 border-b border-zinc-800/50 px-6 flex items-center justify-between bg-black/20 backdrop-blur-md">
            <div class="flex items-center gap-4">
                <div class="flex items-center gap-2.5">
                    <div class="w-8 h-8 rounded-lg agent-gradient flex items-center justify-center text-white shadow-lg">
                        <i data-lucide="bot" class="w-5 h-5"></i>
                    </div>
                    <div>
                        <div class="text-xs font-bold text-white leading-none">架构分析师</div>
                        <div class="text-[10px] text-zinc-500 mt-1">v0.1.2 · 运行中</div>
                    </div>
                </div>
            </div>

            <div class="flex items-center gap-4">
                <div class="flex gap-4 text-[10px] mono text-zinc-600">
                    <span class="flex items-center gap-1"><i data-lucide="cpu" class="w-3 h-3"></i> <span id="cpu-val">--</span>%</span>
                    <span class="flex items-center gap-1"><i data-lucide="database" class="w-3 h-3"></i> <span id="mem-val">--</span>MB</span>
                </div>
                <div class="w-px h-4 bg-zinc-800 mx-1"></div>
                <!-- Agent 详情按钮 -->
                <button onclick="toggleAgentInfo()" class="group flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-zinc-900/50 border border-zinc-800 text-xs text-zinc-400 hover:text-white transition">
                    <i data-lucide="info" class="w-3.5 h-3.5"></i> Agent 详情
                </button>
            </div>
        </header>

        <!-- 消息流容器 -->
        <div id="chat-container" class="flex-1 overflow-y-auto p-8 space-y-8 custom-scrollbar">
            <!-- 消息内容 -->
        </div>

        <!-- Agent 详情卡片 (绝对定位) -->
        <div id="agent-info-card" class="hidden absolute right-6 top-16 w-80 glass border border-zinc-800 rounded-2xl p-5 shadow-2xl z-40">
            <div class="flex items-center gap-3 mb-4">
                <div class="w-12 h-12 rounded-xl agent-gradient flex items-center justify-center text-white">
                    <i data-lucide="bot" class="w-7 h-7"></i>
                </div>
                <div>
                    <h3 class="text-sm font-bold text-white">架构分析师</h3>
                    <p class="text-[10px] text-zinc-500">轻量级 Rust 系统分析工具</p>
                </div>
            </div>
            <div class="space-y-3">
                <div class="text-[10px] uppercase font-bold text-zinc-600 tracking-wider">核心技能</div>
                <div class="flex flex-wrap gap-1.5">
                    <span class="px-2 py-1 rounded bg-blue-500/10 border border-blue-500/20 text-blue-400 text-[9px]">Tauri SDK</span>
                    <span class="px-2 py-1 rounded bg-purple-500/10 border border-purple-500/20 text-purple-400 text-[9px]">Rust Analyzer</span>
                    <span class="px-2 py-1 rounded bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-[9px]">MCP Engine</span>
                </div>
                <div class="pt-3 border-t border-zinc-800 space-y-2">
                    <div class="flex justify-between text-[10px]">
                        <span class="text-zinc-500">运行环境</span>
                        <span class="text-zinc-300">Local (ZeroClaw)</span>
                    </div>
                    <div class="flex justify-between text-[10px]">
                        <span class="text-zinc-500">关联频道</span>
                        <span class="text-zinc-300">Slack, Telegram</span>
                    </div>
                </div>
            </div>
        </div>

        <!-- 输入框区域 -->
        <div class="p-6">
            <div class="max-w-4xl mx-auto">
                <div class="relative bg-zinc-900/80 border border-zinc-800 rounded-2xl p-2 focus-within:border-zinc-700 transition-all shadow-2xl">
                        <textarea
                                id="chat-input"
                                placeholder="发送指令或询问 ZClaw..."
                                class="w-full bg-transparent border-none focus:ring-0 text-sm p-3 resize-none max-h-40 min-h-[44px] text-zinc-200 placeholder-zinc-700 outline-none"
                                rows="1"
                        ></textarea>

                    <div class="flex items-center justify-between px-2 pb-1 pt-1 border-t border-zinc-800/50 mt-1">
                        <div class="flex items-center gap-2">
                            <button class="p-1.5 text-zinc-600 hover:text-zinc-400 rounded-md transition"><i data-lucide="zap" class="w-3.5 h-3.5"></i></button>
                            <button class="p-1.5 text-zinc-600 hover:text-zinc-400 rounded-md transition"><i data-lucide="paperclip" class="w-3.5 h-3.5"></i></button>
                            <div class="h-3 w-px bg-zinc-800 mx-1"></div>
                            <span class="text-[10px] text-zinc-600 mono">MCP: Ready</span>
                        </div>
                        <button id="send-btn" class="bg-blue-600 hover:bg-blue-500 text-white px-4 py-1.5 rounded-xl text-xs font-bold transition shadow-lg shadow-blue-600/20 active:scale-95 flex items-center gap-1.5">
                            <span>发送</span>
                            <i data-lucide="send" class="w-3.5 h-3.5"></i>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </main>

    <!-- 4. 设置面板 (全屏遮罩/右侧滑出) -->
    <div id="settings-panel" class="hidden absolute inset-0 z-[100] flex items-center justify-center bg-black/60 backdrop-blur-sm">
        <div class="w-[800px] h-[600px] bg-[#0c0c0e] border border-zinc-800 rounded-3xl overflow-hidden shadow-2xl flex">
            <!-- 设置侧栏 -->
            <div class="w-48 border-r border-zinc-800/50 bg-black/20 p-6 space-y-2">
                <h2 class="text-sm font-bold text-white mb-6 px-3">设置中心</h2>
                <button class="w-full text-left px-3 py-2 rounded-lg text-xs font-medium text-blue-400 active-nav flex items-center gap-2">
                    <i data-lucide="layers" class="w-3.5 h-3.5"></i> 模型与 API
                </button>
                <button class="w-full text-left px-3 py-2 rounded-lg text-xs font-medium text-zinc-500 hover:text-zinc-300 flex items-center gap-2">
                    <i data-lucide="box" class="w-3.5 h-3.5"></i> MCP 服务
                </button>
                <button class="w-full text-left px-3 py-2 rounded-lg text-xs font-medium text-zinc-500 hover:text-zinc-300 flex items-center gap-2">
                    <i data-lucide="wand-2" class="w-3.5 h-3.5"></i> 技能
                </button>
                <button class="w-full text-left px-3 py-2 rounded-lg text-xs font-medium text-zinc-500 hover:text-zinc-300 flex items-center gap-2">
                    <i data-lucide="hash" class="w-3.5 h-3.5"></i> IM 频道
                </button>
                <button class="w-full text-left px-3 py-2 rounded-lg text-xs font-medium text-zinc-500 hover:text-zinc-300 flex items-center gap-2">
                    <i data-lucide="monitor" class="w-3.5 h-3.5"></i> 工作区
                </button>
                <div class="pt-4 border-t border-zinc-800 mt-4">
                    <button class="w-full text-left px-3 py-2 rounded-lg text-xs font-medium text-zinc-500 hover:text-zinc-300 flex items-center gap-2">
                        <i data-lucide="info" class="w-3.5 h-3.5"></i> 关于 ZClaw
                    </button>
                </div>
            </div>
            <!-- 设置内容区 -->
            <div class="flex-1 p-10 relative overflow-y-auto custom-scrollbar">
                <button onclick="toggleSettings()" class="absolute right-6 top-6 text-zinc-600 hover:text-white"><i data-lucide="x" class="w-6 h-6"></i></button>

                <div class="max-w-md">
                    <h3 class="text-xl font-bold text-white mb-8 text-white">模型与 API 配置</h3>

                    <div class="space-y-6">
                        <div class="space-y-2">
                            <label class="text-xs font-bold text-zinc-500 uppercase tracking-widest">默认 LLM 模型</label>
                            <select class="w-full bg-zinc-900 border border-zinc-800 rounded-xl p-3 text-sm text-zinc-300 focus:outline-none focus:border-blue-500">
                                <option>Local: Llama-3-8B (ZeroClaw Runtime)</option>
                                <option>Anthropic: Claude 3.5 Sonnet</option>
                                <option>OpenAI: GPT-4o-mini</option>
                            </select>
                        </div>

                        <div class="space-y-2">
                            <label class="text-xs font-bold text-zinc-500 uppercase tracking-widest">API Endpoint (Proxy)</label>
                            <input type="text" value="https://api.openai.com/v1" class="w-full bg-zinc-900 border border-zinc-800 rounded-xl p-3 text-sm text-zinc-300 focus:outline-none focus:border-blue-500" />
                        </div>

                        <div class="space-y-2">
                            <label class="text-xs font-bold text-zinc-500 uppercase tracking-widest">本地推理权重路径</label>
                            <div class="flex gap-2">
                                <input type="text" value="/users/zclaw/models/llama3-8b.gguf" class="flex-1 bg-zinc-900 border border-zinc-800 rounded-xl p-3 text-[10px] text-zinc-500 mono" />
                                <button class="bg-zinc-800 px-4 rounded-xl text-xs hover:bg-zinc-700">浏览</button>
                            </div>
                        </div>

                        <button class="w-full bg-blue-600 hover:bg-blue-500 text-white font-bold py-3 rounded-xl transition shadow-lg shadow-blue-600/20">保存全局配置</button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>

<script>
    // 模拟数据与逻辑
    const sessions = [
        { id: 1, title: '系统架构分析', preview: '如何优化 ZeroClaw 吞吐量？', time: '12:45' },
        { id: 2, title: 'IM 机器人联调', preview: 'Slack 签名验证失败问题排查', time: '昨天' }
    ];

    let messages = [
        { role: 'assistant', content: '您好，我是 ZClaw。当前已为您加载 [架构分析师] 核心。本地模型 Llama-3 已就绪，已连接 Slack #dev 频道。', time: '12:45' }
    ];

    function init() {
        renderSessions();
        renderMessages();
        lucide.createIcons();
        startResourceSimulation();

        const sendBtn = document.getElementById('send-btn');
        const input = document.getElementById('chat-input');
        sendBtn.onclick = sendMessage;
        input.onkeydown = (e) => (e.key === 'Enter' && !e.shiftKey) ? (e.preventDefault(), sendMessage()) : null;
    }

    function renderSessions() {
        const list = document.getElementById('session-list');
        list.innerHTML = sessions.map(s => `
                <div class="p-2.5 rounded-xl border border-transparent hover:border-zinc-800 hover:bg-zinc-900/30 cursor-pointer transition-all group">
                    <div class="text-[11px] font-semibold text-zinc-400 group-hover:text-zinc-200 transition-colors truncate">${s.title}</div>
                    <div class="text-[9px] text-zinc-700 mt-1">${s.time}</div>
                </div>
            `).join('');
    }

    function renderMessages() {
        const container = document.getElementById('chat-container');
        container.innerHTML = messages.map(msg => `
                <div class="flex items-start gap-4 ${msg.role === 'user' ? 'flex-row-reverse' : ''}">
                    <div class="w-8 h-8 rounded-lg flex items-center justify-center border ${msg.role === 'user' ? 'bg-zinc-800 border-zinc-700' : 'bg-blue-600/10 border-blue-500/20 shadow-sm'}">
                        <i data-lucide="${msg.role === 'user' ? 'user' : 'bot'}" class="w-4 h-4 ${msg.role === 'user' ? 'text-zinc-400' : 'text-blue-400'}"></i>
                    </div>
                    <div class="max-w-[75%] space-y-1">
                        <div class="px-4 py-3 rounded-2xl text-sm leading-relaxed ${msg.role === 'user' ? 'bg-blue-600 text-white shadow-lg shadow-blue-500/10' : 'bg-zinc-900/50 border border-zinc-800 text-zinc-200'}">
                            ${msg.content}
                        </div>
                        <div class="text-[10px] text-zinc-700 px-1 ${msg.role === 'user' ? 'text-right' : ''}">${msg.time}</div>
                    </div>
                </div>
            `).join('');
        lucide.createIcons();
        container.scrollTop = container.scrollHeight;
    }

    function sendMessage() {
        const input = document.getElementById('chat-input');
        const text = input.value.trim();
        if (!text) return;
        messages.push({ role: 'user', content: text, time: '现在' });
        input.value = '';
        renderMessages();
        setTimeout(() => {
            messages.push({ role: 'assistant', content: '正在调度本地 MCP 服务并检查关联频道状态...', time: '现在' });
            renderMessages();
        }, 800);
    }

    function toggleAgentInfo() {
        const card = document.getElementById('agent-info-card');
        card.classList.toggle('hidden');
    }

    function toggleSettings() {
        const panel = document.getElementById('settings-panel');
        panel.classList.toggle('hidden');
    }

    function startResourceSimulation() {
        setInterval(() => {
            document.getElementById('cpu-val').innerText = (Math.random() * 15 + 3).toFixed(1);
            document.getElementById('mem-val').innerText = Math.floor(Math.random() * 40 + 50);
        }, 3000);
    }

    window.onload = init;
</script>
</body>
</html>