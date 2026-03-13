<script lang="ts">
    import { FileText, FileImage, File, Search, Clock, Download, Plus } from '@lucide/svelte';

    let { showRightPanel, rightPanelTab, activeAgentId, onToggle }: { showRightPanel: boolean, rightPanelTab: string, activeAgentId: number, onToggle: () => void } = $props();

    const agents = [
        { id: 1, color: 'bg-blue-600', name: '苏东坡', icon: '🤖', description: 'AI coworker agent' },
        { id: 2, color: 'bg-rose-600', name: '李白', icon: '🍷', description: '创意写作专家' },
        { id: 3, color: 'bg-amber-600', name: '杜甫', icon: '✍️', description: '逻辑分析助手' },
        { id: 4, color: 'bg-emerald-600', name: '王维', icon: '🌿', description: '视觉艺术顾问' },
    ];

    let activeAgent = $derived(agents.find(a => a.id === activeAgentId) || agents[0]);

    const mockFiles = [
        { name: '红烧肉短视频脚本.docx', size: '24 KB', type: 'doc', date: '10分钟前' },
        { name: '东坡肉历史考据.pdf', size: '1.2 MB', type: 'pdf', date: '1小时前' },
    ];
</script>

<aside
    class="right-panel flex flex-col transition-all duration-300 ease-in-out overflow-hidden h-full shrink-0 z-20 bg-[var(--app-surface)] border-l border-[var(--app-divider)]"
    style="width: {showRightPanel ? '300px' : '0'}; opacity: {showRightPanel ? '1' : '0'};"
>
    <div class="w-[300px] flex flex-col h-full shrink-0 px-4 pt-6 pb-4">
        {#if rightPanelTab === 'agent'}
            <div class="flex items-center gap-3 mb-6">
                <div class="w-12 h-12 rounded-xl {activeAgent.color} flex items-center justify-center text-2xl shadow-sm">
                    {activeAgent.icon}
                </div>
                <div class="flex flex-col min-w-0">
                    <h2 class="text-base font-semibold text-[var(--app-text)]">{activeAgent.name}</h2>
                    <p class="text-[13px] text-[var(--app-text-muted)] mt-0.5 truncate">{activeAgent.description}</p>
                </div>
            </div>

            <div class="flex-1 overflow-y-auto custom-scrollbar space-y-4 pr-1 pb-6">
                <section>
                    <h3 class="text-[12px] font-semibold text-[var(--app-text-muted)] uppercase tracking-wider mb-2">关于我</h3>
                    <div class="rounded-xl p-3.5 text-[13px] leading-relaxed text-[var(--app-text-secondary)] space-y-1.5 bg-[var(--app-surface-elevated)] border border-[var(--app-divider)]">
                        <div class="flex gap-2">生日: <span class="text-[var(--app-text)]">2026.03.01</span></div>
                        <div class="flex gap-2">风格: <span class="text-[var(--app-text)] leading-tight">sharp, resourceful, no-nonsense</span></div>
                    </div>
                </section>
                <section>
                    <h3 class="text-[12px] font-semibold text-[var(--app-text-muted)] uppercase tracking-wider mb-2">我眼中的你</h3>
                    <div class="rounded-xl p-3.5 text-[13px] leading-relaxed text-[var(--app-text-secondary)] space-y-1.5 bg-[var(--app-surface-elevated)] border border-[var(--app-divider)]">
                        <div class="flex justify-between items-center">姓名: <span class="text-[var(--app-text)]">六哥</span></div>
                        <div class="flex justify-between items-center">称呼: <span class="text-[var(--app-text)]">主人</span></div>
                        <div class="flex justify-between items-center">时区: <span class="text-[var(--app-text)]">Asia/Shanghai</span></div>
                        <div class="flex gap-2 items-start mt-2 border-t border-[var(--app-border)] pt-2">专注于: <span class="text-[var(--app-text)] flex-1 text-right line-clamp-3">coding, writing, data, design, marketing</span></div>
                    </div>
                </section>
                <section>
                    <h3 class="text-[12px] font-semibold text-[var(--app-text-muted)] uppercase tracking-wider mb-2">我的笔记</h3>
                    <div class="rounded-xl p-3.5 text-[13px] leading-relaxed text-[var(--app-text-secondary)] space-y-2 min-h-[80px] bg-[var(--app-surface-elevated)] border border-[var(--app-divider)]">
                        <div>当前项目: </div>
                        <div>工作流: </div>
                        <div>记忆系统: </div>
                        <div>工具链: </div>
                    </div>
                </section>
            </div>
        {:else}
            <div class="flex flex-col h-full">
                <h3 class="text-[15px] font-semibold text-[var(--app-text)] flex items-center gap-2 mb-3">
                    <FileText size={17} class="text-[var(--app-text-muted)]" /> 空间文件
                </h3>
                <div class="relative mb-4">
                    <Search size={14} class="absolute left-3 top-1/2 -translate-y-1/2 text-[var(--app-text-muted)]" />
                    <input
                        type="text"
                        placeholder="搜索文档..."
                        class="w-full bg-[var(--app-surface-elevated)] border border-[var(--app-divider)] rounded-xl py-2 pl-9 pr-4 text-[13px] outline-none focus:border-[var(--app-border-focus)] focus:ring-1 focus:ring-[var(--app-border-focus)] transition-colors text-[var(--app-text)] placeholder:text-[var(--app-text-muted)]"
                    />
                </div>

                <div class="flex-1 overflow-y-auto custom-scrollbar space-y-2 pr-1">
                    {#each mockFiles as file (file.name)}
                        <div class="group rounded-xl p-3 border border-[var(--app-divider)] hover:bg-[var(--app-surface-hover)] transition-colors cursor-pointer">
                            <div class="flex items-center gap-3">
                                <div class="w-10 h-10 rounded-lg bg-[var(--app-surface-elevated)] flex items-center justify-center text-[var(--app-text-muted)] shrink-0">
                                    {#if file.type === 'doc'}
                                        <FileText size={16}/>
                                    {:else if file.type === 'image'}
                                        <FileImage size={16}/>
                                    {:else}
                                        <File size={16}/>
                                    {/if}
                                </div>
                                <div class="flex-1 overflow-hidden min-w-0">
                                    <div class="text-[13px] font-medium text-[var(--app-text)] truncate">{file.name}</div>
                                    <div class="flex items-center gap-2 mt-1">
                                        <span class="text-[11px] text-[var(--app-text-muted)]">{file.size}</span>
                                        <span class="w-1 h-1 rounded-full bg-[var(--app-border)]"></span>
                                        <span class="text-[11px] text-[var(--app-text-muted)] flex items-center gap-1"><Clock size={10}/> {file.date}</span>
                                    </div>
                                </div>
                                <Download size={14} class="text-[var(--app-text-muted)] opacity-0 group-hover:opacity-100 hover:text-[var(--app-accent)] transition-opacity shrink-0" />
                            </div>
                        </div>
                    {/each}

                    <button class="w-full mt-3 py-3 border border-dashed border-[var(--app-divider)] rounded-xl flex items-center justify-center gap-2 text-[13px] text-[var(--app-text-muted)] hover:bg-[var(--app-accent-soft)] hover:border-[var(--app-accent)]/30 hover:text-[var(--app-accent)] transition-colors duration-200 focus:outline-none">
                        <Plus size={14}/> 上传新文件
                    </button>
                </div>
            </div>
        {/if}

        <div class="mt-auto pt-4 flex justify-end shrink-0">
            <button class="text-[12px] text-[var(--app-text-muted)] hover:text-[var(--app-accent)] transition-colors focus:outline-none" onclick={onToggle}>收起面板 →</button>
        </div>
    </div>
</aside>

<style>
    .custom-scrollbar::-webkit-scrollbar { width: 4px; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--app-divider); border-radius: 4px; }
</style>
