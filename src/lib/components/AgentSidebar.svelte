<script lang="ts">
    import { Plus, Settings, HelpCircle } from '@lucide/svelte';

    let { activeAgentId = $bindable() }: { activeAgentId: number } = $props();

    const agents = [
        { id: 1, color: 'bg-blue-600', name: '苏东坡', icon: '🤖', description: 'AI coworker agent' },
        { id: 2, color: 'bg-red-800', name: '李白', icon: '🍷', description: '创意写作专家' },
        { id: 3, color: 'bg-orange-600', name: '杜甫', icon: '✍️', description: '逻辑分析助手' },
        { id: 4, color: 'bg-emerald-700', name: '王维', icon: '🌿', description: '视觉艺术顾问' },
    ];
</script>

<aside class="agent-sidebar w-[76px] shrink-0 zclaw-surface flex flex-col items-center py-4 gap-5 z-30 relative overflow-hidden">
    <div class="flex flex-col gap-3 overflow-y-auto px-2 flex-1 scrollbar-hide items-center w-full">
        {#each agents as agent (agent.id)}
            <button
                class="relative group flex items-center justify-center p-0.5 transition-all duration-200 rounded-2xl focus:outline-none {activeAgentId === agent.id ? '' : 'hover:bg-[var(--app-surface-hover)]'}"
                onclick={() => activeAgentId = agent.id}
                title={agent.name}
            >
                <div class="w-11 h-11 {agent.color} rounded-xl flex items-center justify-center text-xl shadow-md transition-all group-active:scale-95 {activeAgentId === agent.id ? 'ring-2 ring-[var(--app-accent)] ring-offset-2 ring-offset-[var(--app-surface)]' : 'opacity-90 hover:opacity-100'}">
                    {agent.icon}
                </div>
            </button>
        {/each}
        <button class="w-11 h-11 mt-1 rounded-xl border border-dashed border-[var(--app-border)] flex items-center justify-center hover:bg-[var(--app-accent-soft)] hover:border-[var(--app-accent)]/40 transition-all focus:outline-none" title="添加 Agent">
            <Plus size={20} class="text-[var(--app-text-muted)] hover:text-[var(--app-accent)] transition-colors" />
        </button>
    </div>
    <div class="mt-auto flex flex-col items-center gap-2 pb-2 shrink-0">
        <button class="w-9 h-9 flex items-center justify-center rounded-lg text-[var(--app-text-muted)] hover:text-[var(--app-accent)] hover:bg-[var(--app-surface-hover)] transition-colors focus:outline-none" title="设置">
            <Settings size={20} />
        </button>
        <button class="w-9 h-9 flex items-center justify-center rounded-lg text-[var(--app-text-muted)] hover:text-[var(--app-accent)] hover:bg-[var(--app-surface-hover)] transition-colors focus:outline-none" title="帮助">
            <HelpCircle size={20} />
        </button>
    </div>
</aside>

<style>
    .scrollbar-hide::-webkit-scrollbar {
        display: none;
    }
    .scrollbar-hide {
        -ms-overflow-style: none;
        scrollbar-width: none;
    }
</style>
