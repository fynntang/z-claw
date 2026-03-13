<script lang="ts">
    import { Plus, Hash, MessageSquare, MoreHorizontal, Slack } from '@lucide/svelte';

    let { activeListItem = $bindable() }: { activeListItem: string } = $props();

    const channels = [
        { id: 'im-1', title: 'Slack 知识库同步', icon: Slack, color: 'text-[#36C5F0]', bg: 'bg-[#36C5F0]/20' },
        { id: 'im-2', title: '红烧肉的工作流', icon: Hash, color: 'text-indigo-400', bg: 'bg-indigo-400/10' }
    ];

    const chats = [
        { id: 'chat-1', title: '红烧肉的做法...', time: '输入中...' },
        { id: 'chat-2', title: '红烧肉的做法...', time: '输入中...' },
        { id: 'chat-3', title: '红烧肉的做法...', time: '输入中...' },
        { id: 'chat-4', title: '红烧肉的做法...', time: '输入中...' },
        { id: 'chat-5', title: '红烧肉的做法...', time: '输入中...' },
    ];
</script>

<aside class="channel-sidebar w-[260px] zclaw-surface flex flex-col z-20 shrink-0 relative overflow-hidden">
    <div class="px-4 py-3 flex items-center justify-between border-b border-[var(--app-border)]">
        <span class="text-[11px] font-semibold tracking-wider text-[var(--app-text-muted)] uppercase">对话</span>
        <button class="w-8 h-8 flex items-center justify-center rounded-lg text-[var(--app-text-muted)] hover:text-[var(--app-accent)] hover:bg-[var(--app-surface-hover)] transition-colors focus:outline-none" title="新对话">
            <Plus size={15} />
        </button>
    </div>

    <div class="flex-1 overflow-y-auto px-3 py-3 space-y-1.5 custom-scrollbar">
        {#each chats as chat (chat.id)}
            <button
                class="group w-full text-left px-3 py-2.5 rounded-xl flex items-center justify-between gap-2 transition-all {activeListItem === chat.id ? 'bg-[var(--app-accent-soft)] text-[var(--app-text)] border border-[var(--app-accent)]/30' : 'bg-transparent hover:bg-[var(--app-surface-hover)] text-[var(--app-text-secondary)] border border-transparent'}"
                onclick={() => activeListItem = chat.id}
            >
                <div class="flex items-center gap-3 overflow-hidden min-w-0">
                    <MessageSquare size={15} class="shrink-0 {activeListItem === chat.id ? 'text-[var(--app-accent)]' : 'text-[var(--app-text-muted)]'}" />
                    <div class="truncate text-left">
                        <div class="text-[13px] font-medium truncate">{chat.title}</div>
                        <div class="text-[11px] mt-0.5 {activeListItem === chat.id ? 'text-[var(--app-text-secondary)]' : 'text-[var(--app-text-muted)]'}">{chat.time}</div>
                    </div>
                </div>
                <MoreHorizontal size={14} class="shrink-0 opacity-0 group-hover:opacity-100 transition-opacity text-[var(--app-text-muted)]" />
            </button>
        {/each}

        <div class="h-3"></div>
        {#each channels as channel (channel.id)}
            {@const Icon = channel.icon}
            <button
                class="channel-item group w-full text-left px-3 py-2.5 rounded-xl flex items-center gap-3 transition-all {activeListItem === channel.id ? 'bg-[var(--app-accent-soft)] border border-[var(--app-accent)]/30' : 'bg-transparent hover:bg-[var(--app-surface-hover)] border border-transparent'}"
                onclick={() => activeListItem = channel.id}
            >
                <div class="w-8 h-8 rounded-lg flex items-center justify-center shrink-0 {activeListItem === channel.id ? 'bg-[var(--app-accent-muted)] text-[var(--app-accent)]' : channel.bg + ' ' + channel.color}">
                    <Icon size={14} />
                </div>
                <div class="flex-1 overflow-hidden min-w-0">
                    <div class="text-[13px] font-medium truncate {activeListItem === channel.id ? 'text-[var(--app-text)]' : 'text-[var(--app-text-secondary)]'}">{channel.title}</div>
                </div>
                <MoreHorizontal size={14} class="shrink-0 opacity-0 group-hover:opacity-100 transition-opacity text-[var(--app-text-muted)]" />
            </button>
        {/each}
    </div>
</aside>

<style>
    .custom-scrollbar::-webkit-scrollbar { width: 4px; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--app-border); border-radius: 4px; }
</style>
