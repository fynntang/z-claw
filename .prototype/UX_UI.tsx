import React, { useState } from 'react';
import {
    Settings,
    HelpCircle,
    Plus,
    Bell,
    Moon,
    Minimize2,
    Square,
    X,
    MoreHorizontal,
    MessageSquare,
    Hash,
    Send,
    FileText,
    User,
    Slack,
    ChevronRight,
    Sparkles,
    Paperclip,
    Download,
    FileCode,
    FileImage,
    File,
    Search,
    Clock
} from 'lucide-react';

const App = () => {
    const [rightPanelTab, setRightPanelTab] = useState('agent'); // 'agent' 或 'file'
    const [showRightPanel, setShowRightPanel] = useState(true);
    const [activeAgentId, setActiveAgentId] = useState(1);
    const [activeListItem, setActiveListItem] = useState('chat-1');

    const [agents] = useState([
        { id: 1, color: 'bg-blue-500', name: '苏东坡', icon: '🤖', description: 'AI coworker agent' },
        { id: 2, color: 'bg-red-500', name: '李白', icon: '🍷', description: '创意写作专家' },
        { id: 3, color: 'bg-orange-500', name: '杜甫', icon: '✍️', description: '逻辑分析助手' },
        { id: 4, color: 'bg-green-500', name: '王维', icon: '🌿', description: '视觉艺术顾问' },
    ]);

    const activeAgent = agents.find(a => a.id === activeAgentId);

    // 模拟文件数据
    const mockFiles = [
        { name: '红烧肉短视频脚本.docx', size: '24 KB', type: 'doc', date: '10分钟前' },
        { name: '东坡肉历史考据.pdf', size: '1.2 MB', type: 'pdf', date: '1小时前' },
        { name: '配乐素材_古风.mp3', size: '4.5 MB', type: 'audio', date: '昨天' },
        { name: '美食缩略图_01.jpg', size: '800 KB', type: 'image', date: '2天前' },
    ];

    const mockMessages = [
        { id: 1, role: 'assistant', content: '您好，主人。我是苏东坡，您的 AI 协作伙伴。今天有什么可以帮您的？', time: '10:00' },
        { id: 2, role: 'user', content: '帮我策划一下红烧肉的短视频脚本，要带有文化底蕴。', time: '10:05' },
        { id: 3, role: 'assistant', content: '没问题。红烧肉又名“东坡肉”。我们可以从《食猪肉诗》切入：\n\n“黄州好猪肉，价贱如泥土。贵者不肯吃，贫者不解煮。”\n\n脚本结构建议：\n1. 开场：慢动作拍摄五花肉的纹理。\n2. 旁白：引用诗句，讲述贬谪黄州时的心境。\n3. 过程：强调“慢着火，少着水，火候足时它自美”。', time: '10:06' },
    ];

    return (
        <div className="flex h-screen w-full bg-[#0a0a0a] text-gray-300 font-sans selection:bg-orange-500/30 overflow-hidden p-2">
            <div className="flex flex-1 border border-white/10 rounded-2xl overflow-hidden bg-[#121212]">

                {/* 第一级侧边栏 - Agent 切换器 */}
                <aside className="w-20 border-r border-white/5 flex flex-col items-center py-4 gap-6 bg-[#0d0d0d] z-30">
                    <div className="mb-2">
                        <div className="w-10 h-10 bg-gradient-to-tr from-orange-500 to-yellow-500 rounded-xl flex items-center justify-center font-bold text-black text-xs shadow-lg shadow-orange-500/20">ZC</div>
                    </div>

                    <div className="flex flex-col gap-4 overflow-y-auto custom-scrollbar px-2">
                        {agents.map(agent => (
                            <div
                                key={agent.id}
                                onClick={() => setActiveAgentId(agent.id)}
                                className={`relative group p-1 rounded-2xl transition-all duration-300 cursor-pointer ${activeAgentId === agent.id ? 'ring-2 ring-blue-500/50' : 'hover:ring-2 hover:ring-white/10'}`}
                            >
                                <div className={`w-12 h-12 ${agent.color} rounded-2xl flex items-center justify-center text-xl shadow-lg transition-transform group-active:scale-95`}>
                                    {agent.icon}
                                </div>
                                {activeAgentId === agent.id && (
                                    <div className="absolute -left-3 top-1/2 -translate-y-1/2 w-1.5 h-6 bg-blue-500 rounded-r-full shadow-[0_0_10px_rgba(59,130,246,0.5)]" />
                                )}
                            </div>
                        ))}

                        <button className="w-12 h-12 border-2 border-dashed border-white/10 rounded-2xl flex items-center justify-center hover:bg-white/5 hover:border-white/20 transition-all group">
                            <Plus size={20} className="group-hover:rotate-90 transition-transform duration-300 text-gray-500 group-hover:text-white" />
                        </button>
                    </div>

                    <div className="mt-auto flex flex-col items-center gap-6 pb-2">
                        <Settings className="cursor-pointer text-gray-500 hover:text-white transition-colors" size={20} />
                        <HelpCircle className="cursor-pointer text-gray-500 hover:text-white transition-colors" size={20} />
                        <div className="w-8 h-8 rounded-full overflow-hidden border border-white/10 cursor-pointer hover:border-orange-500/50 transition-all">
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="avatar" />
                        </div>
                    </div>
                </aside>

                {/* 第二级侧边栏 - 频道/会话列表 */}
                <aside className="w-64 border-r border-white/5 flex flex-col bg-[#141414] z-20">
                    <div className="p-4 flex items-center justify-between border-b border-white/5">
                        <span className="text-xs font-bold tracking-widest text-gray-500 uppercase">工作区频道</span>
                        <Plus size={16} className="cursor-pointer hover:text-white" />
                    </div>

                    <div className="flex-1 overflow-y-auto p-2 space-y-1 custom-scrollbar">
                        <div
                            onClick={() => setActiveListItem('im-1')}
                            className={`p-2.5 rounded-xl flex items-center gap-3 cursor-pointer text-sm transition-all border border-transparent ${
                                activeListItem === 'im-1' ? 'bg-white/10 text-white border-white/10 shadow-inner' : 'hover:bg-white/5 text-gray-400'
                            }`}
                        >
                            <div className={`w-6 h-6 rounded flex items-center justify-center flex-shrink-0 ${activeListItem === 'im-1' ? 'bg-[#36C5F0]/20 text-[#36C5F0]' : 'bg-white/5 text-gray-500'}`}>
                                <Slack size={12} />
                            </div>
                            <span className="flex-1 truncate font-medium">Slack 知识库同步</span>
                        </div>

                        <div
                            onClick={() => setActiveListItem('im-2')}
                            className={`p-2.5 rounded-xl flex items-center gap-3 cursor-pointer text-sm transition-all border border-transparent ${
                                activeListItem === 'im-2' ? 'bg-white/10 text-white border-white/10 shadow-inner' : 'hover:bg-white/5 text-gray-400'
                            }`}
                        >
                            <Hash size={16} className={activeListItem === 'im-2' ? 'text-blue-400' : 'text-gray-500'} />
                            <span className="flex-1 truncate font-medium">红烧肉的工作流</span>
                        </div>

                        <div className="pt-4 pb-2 px-4">
                            <span className="text-[10px] font-bold text-gray-600 uppercase tracking-widest">最近会话</span>
                        </div>

                        <div
                            onClick={() => setActiveListItem('chat-1')}
                            className={`p-3 rounded-xl flex items-center justify-between group cursor-pointer transition-all shadow-lg ${
                                activeListItem === 'chat-1' ? 'bg-orange-600/90 text-white shadow-orange-900/20' : 'bg-white/5 hover:bg-white/10 text-gray-400'
                            }`}
                        >
                            <div className="flex items-center gap-3 overflow-hidden">
                                <div className={`w-8 h-8 rounded-lg flex items-center justify-center flex-shrink-0 ${activeListItem === 'chat-1' ? 'bg-white/20' : 'bg-black/40'}`}>
                                    <MessageSquare size={14} />
                                </div>
                                <div className="truncate">
                                    <div className="text-sm font-medium truncate">红烧肉的做法...</div>
                                    <div className={`text-[10px] ${activeListItem === 'chat-1' ? 'opacity-70 text-white' : 'text-gray-600'}`}>3 分钟前</div>
                                </div>
                            </div>
                            <MoreHorizontal size={14} className={`${activeListItem === 'chat-1' ? 'opacity-100' : 'opacity-0 group-hover:opacity-100'}`} />
                        </div>

                        {[2, 3].map(i => (
                            <div
                                key={i}
                                onClick={() => setActiveListItem(`chat-${i}`)}
                                className={`p-3 rounded-xl flex items-center justify-between group cursor-pointer transition-all border border-transparent ${
                                    activeListItem === `chat-${i}` ? 'bg-orange-600/90 text-white shadow-lg shadow-orange-900/20' : 'bg-white/5 hover:bg-white/10 text-gray-400 hover:border-white/5'
                                }`}
                            >
                                <div className="flex items-center gap-3 overflow-hidden">
                                    <div className={`w-8 h-8 rounded-lg flex items-center justify-center flex-shrink-0 ${activeListItem === `chat-${i}` ? 'bg-white/20' : 'bg-black/40'}`}>
                                        <MessageSquare size={14} />
                                    </div>
                                    <div className="truncate text-sm font-medium">历史对话记录 #{i}</div>
                                </div>
                                <MoreHorizontal size={14} className={`${activeListItem === `chat-${i}` ? 'opacity-100' : 'opacity-0 group-hover:opacity-100'}`} />
                            </div>
                        ))}
                    </div>
                </aside>

                {/* 主内容容器 */}
                <div className="flex-1 flex flex-col bg-[#111] overflow-hidden">

                    {/* 顶部工具栏 - Titlebar (保持全宽) */}
                    <header className="h-12 border-b border-white/5 flex items-center justify-between px-4 bg-[#111] z-30 shrink-0 select-none">
                        <div className="flex items-center gap-2">
                            <span className="text-lg">{activeAgent.icon}</span>
                            <span className="font-medium text-sm">{activeAgent.name}</span>
                            <span className="text-[10px] bg-green-500/10 text-green-500 px-1.5 py-0.5 rounded border border-green-500/20 ml-2">在线</span>
                        </div>

                        <div className="flex items-center gap-4">
                            <div className="flex items-center bg-black/40 rounded-lg p-1 border border-white/5">
                                <button
                                    onClick={() => { setRightPanelTab('file'); setShowRightPanel(true); }}
                                    className={`flex items-center gap-1.5 px-3 py-1 rounded-md text-xs transition-all ${showRightPanel && rightPanelTab === 'file' ? 'bg-[#222] text-white shadow-sm border border-white/5' : 'text-gray-500 hover:text-gray-300'}`}
                                >
                                    <FileText size={12} /> 文件
                                </button>
                                <button
                                    onClick={() => { setRightPanelTab('agent'); setShowRightPanel(!showRightPanel || rightPanelTab !== 'agent'); }}
                                    className={`flex items-center gap-1.5 px-3 py-1 rounded-md text-xs transition-all ${showRightPanel && rightPanelTab === 'agent' ? 'bg-orange-600/20 text-orange-500 shadow-sm border border-orange-500/20' : 'text-gray-500 hover:text-gray-300'}`}
                                >
                                    <User size={12} /> Agent {showRightPanel && rightPanelTab === 'agent' && <ChevronRight size={12} className="ml-1 animate-pulse" />}
                                </button>
                            </div>

                            <div className="flex items-center gap-3 text-gray-500 border-l border-white/10 pl-4">
                                <Bell size={15} className="hover:text-yellow-500 cursor-pointer transition-colors" />
                                <Moon size={15} className="hover:text-blue-400 cursor-pointer transition-colors" />

                                <div className="flex items-center gap-1 ml-2 border-l border-white/10 pl-3">
                                    <div className="p-1 hover:bg-white/5 rounded cursor-pointer transition-colors"><Minimize2 size={14}/></div>
                                    <div className="p-1 hover:bg-white/5 rounded cursor-pointer transition-colors"><Square size={12}/></div>
                                    <div className="p-1 hover:bg-red-500/80 hover:text-white rounded cursor-pointer transition-colors"><X size={15}/></div>
                                </div>
                            </div>
                        </div>
                    </header>

                    <div className="flex-1 flex flex-row relative overflow-hidden">

                        {/* 聊天主界面 (始终显示) */}
                        <main className="flex-1 flex flex-col relative z-10">
                            <div className="flex-1 overflow-y-auto p-6 space-y-8 custom-scrollbar">
                                {mockMessages.map((msg) => (
                                    <div key={msg.id} className={`flex gap-4 ${msg.role === 'user' ? 'flex-row-reverse' : ''}`}>
                                        <div className={`w-8 h-8 rounded-lg flex items-center justify-center flex-shrink-0 text-sm ${msg.role === 'assistant' ? activeAgent.color : 'bg-orange-600'}`}>
                                            {msg.role === 'assistant' ? activeAgent.icon : '👤'}
                                        </div>
                                        <div className={`max-w-[80%] flex flex-col ${msg.role === 'user' ? 'items-end' : ''}`}>
                                            <div className={`rounded-2xl p-4 text-sm leading-relaxed ${
                                                msg.role === 'assistant' ? 'bg-[#1a1a1a] border border-white/5 text-gray-300 shadow-xl' : 'bg-orange-600 text-white shadow-lg shadow-orange-900/20'
                                            }`}>
                                                <pre className="whitespace-pre-wrap font-sans">{msg.content}</pre>
                                            </div>
                                            <span className="text-[10px] text-gray-600 mt-2">{msg.time}</span>
                                        </div>
                                    </div>
                                ))}
                            </div>

                            <div className="p-6">
                                <div className="max-w-3xl mx-auto relative">
                                    <div className="bg-[#1a1a1a] border border-white/10 rounded-2xl p-4 shadow-2xl focus-within:border-orange-500/50 transition-all">
                    <textarea
                        className="w-full bg-transparent border-none outline-none text-sm resize-none placeholder:text-gray-600"
                        placeholder={`给 ${activeAgent.name} 发送消息...`}
                        rows="2"
                    ></textarea>
                                        <div className="flex justify-between items-center mt-2 pt-2 border-t border-white/5">
                                            <div className="flex gap-3">
                                                <button className="text-gray-500 hover:text-white transition-colors"><Paperclip size={18}/></button>
                                                <button className="text-gray-500 hover:text-white transition-colors"><Sparkles size={18}/></button>
                                            </div>
                                            <button className="bg-orange-600 hover:bg-orange-500 text-white p-2.5 rounded-xl shadow-lg shadow-orange-900/40 transition-all active:scale-95">
                                                <Send size={16} />
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </main>

                        {/* 右侧通用面板 (Agent 或 文件) */}
                        <aside
                            className={`bg-[#0f0f0f] flex flex-col transition-all duration-300 ease-in-out overflow-hidden h-full shrink-0 border-l border-white/5 ${
                                showRightPanel ? 'w-80 opacity-100' : 'w-0 opacity-0 border-none'
                            }`}
                        >
                            <div className="min-w-[320px] flex flex-col h-full">

                                {/* 面板内容切换渲染 */}
                                {rightPanelTab === 'agent' ? (
                                    /* Agent 详情内容 */
                                    <div className="flex-1 p-6 overflow-y-auto custom-scrollbar">
                                        <div className="flex flex-col items-center text-center mb-8">
                                            <div className={`w-20 h-20 ${activeAgent.color} rounded-3xl flex items-center justify-center text-4xl mb-4 shadow-2xl shadow-blue-500/20`}>
                                                {activeAgent.icon}
                                            </div>
                                            <h2 className="text-xl font-bold text-white">{activeAgent.name}</h2>
                                            <p className="text-xs text-gray-500 mt-1">{activeAgent.description}</p>
                                        </div>

                                        <div className="space-y-6 pb-10">
                                            <section>
                                                <h3 className="text-[10px] font-bold text-gray-500 uppercase tracking-widest mb-3">关于我</h3>
                                                <div className="bg-white/5 border border-white/5 rounded-2xl p-4 text-xs leading-relaxed text-gray-400">
                                                    <div className="flex justify-between mb-2"><span className="opacity-50">生日:</span><span>2026.03.01</span></div>
                                                    <div className="flex justify-between"><span className="opacity-50">风格:</span><span>sharp, resourceful</span></div>
                                                </div>
                                            </section>
                                            <section>
                                                <h3 className="text-[10px] font-bold text-gray-500 uppercase tracking-widest mb-3">我眼中的你</h3>
                                                <div className="bg-white/5 border border-white/5 rounded-2xl p-4 text-xs space-y-2 text-gray-400">
                                                    <div className="flex justify-between font-medium"><span>六哥</span><span className="opacity-50 text-[10px]">主人</span></div>
                                                </div>
                                            </section>
                                            <section>
                                                <h3 className="text-[10px] font-bold text-gray-500 uppercase tracking-widest mb-3">统计数据</h3>
                                                <div className="bg-white/5 border border-white/5 rounded-2xl p-4 text-xs space-y-4">
                                                    <div className="flex flex-col gap-1">
                                                        <div className="flex justify-between text-[10px] mb-1">
                                                            <span className="text-gray-300 font-medium">任务完成率</span>
                                                            <span>85%</span>
                                                        </div>
                                                        <div className="h-1.5 w-full bg-white/5 rounded-full overflow-hidden">
                                                            <div className="h-full bg-blue-500 w-[85%] rounded-full shadow-[0_0_8px_rgba(59,130,246,0.4)]"></div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </section>
                                        </div>
                                    </div>
                                ) : (
                                    /* 文件列表内容 - 现在和 Agent 详情风格完全一致 */
                                    <div className="flex-1 flex flex-col overflow-hidden">
                                        <div className="p-6 border-b border-white/5">
                                            <h3 className="text-sm font-bold text-white flex items-center gap-2">
                                                <FileText size={16} className="text-orange-500" /> 空间文件
                                            </h3>
                                            <div className="mt-4 relative">
                                                <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-600" />
                                                <input
                                                    type="text"
                                                    placeholder="搜索文档..."
                                                    className="w-full bg-white/5 border border-white/10 rounded-xl py-2 pl-9 pr-4 text-xs outline-none focus:border-orange-500/50 transition-all"
                                                />
                                            </div>
                                        </div>

                                        <div className="flex-1 p-4 overflow-y-auto custom-scrollbar space-y-3">
                                            {mockFiles.map((file, idx) => (
                                                <div key={idx} className="bg-white/5 border border-white/5 p-3 rounded-2xl hover:bg-white/10 hover:border-orange-500/20 transition-all group cursor-pointer">
                                                    <div className="flex items-center gap-3">
                                                        <div className="w-10 h-10 bg-black/40 rounded-xl flex items-center justify-center text-orange-500 shrink-0">
                                                            {file.type === 'doc' ? <FileText size={18}/> : file.type === 'image' ? <FileImage size={18}/> : <File size={18}/>}
                                                        </div>
                                                        <div className="flex-1 overflow-hidden">
                                                            <div className="text-xs font-medium text-gray-200 truncate">{file.name}</div>
                                                            <div className="flex items-center gap-2 mt-1">
                                                                <span className="text-[10px] text-gray-600">{file.size}</span>
                                                                <span className="w-1 h-1 rounded-full bg-gray-800"></span>
                                                                <span className="text-[10px] text-gray-600 flex items-center gap-1"><Clock size={10}/> {file.date}</span>
                                                            </div>
                                                        </div>
                                                        <Download size={12} className="text-gray-600 opacity-0 group-hover:opacity-100 hover:text-white transition-all shrink-0" />
                                                    </div>
                                                </div>
                                            ))}

                                            {/* 上传按钮固定在底部或作为列表项 */}
                                            <button className="w-full py-3 border border-dashed border-white/10 rounded-2xl flex items-center justify-center gap-2 text-xs text-gray-500 hover:bg-white/5 hover:border-white/20 hover:text-white transition-all">
                                                <Plus size={14}/> 上传新文件
                                            </button>
                                        </div>
                                    </div>
                                )}

                                {/* 面板底部控制 (可选) */}
                                <div className="p-4 border-t border-white/5 flex items-center justify-between text-[10px] text-gray-600">
                                    <span>存储空间: 1.2GB / 5GB</span>
                                    <button className="hover:text-orange-500 transition-colors" onClick={() => setShowRightPanel(false)}>收起面板</button>
                                </div>
                            </div>
                        </aside>
                    </div>
                </div>

            </div>
        </div>
    );
};

export default App;