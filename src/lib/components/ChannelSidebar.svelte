<script lang="ts">
	import { Plus, Hash, MessageSquare, MoreHorizontal, Slack } from '@lucide/svelte';

	let { activeListItem = $bindable() }: { activeListItem: string } = $props();

	const channels = [
		{ id: 'im-1', title: 'Slack 知识库同步', icon: Slack, color: 'text-sky-400', bg: 'bg-sky-500/15' },
		{ id: 'im-2', title: '红烧肉的工作流', icon: Hash, color: 'text-violet-400', bg: 'bg-violet-500/15' }
	];

	const chats = [
		{ id: 'chat-1', title: '红烧肉的做法...', time: '输入中...' },
		{ id: 'chat-2', title: '红烧肉的做法...', time: '输入中...' },
		{ id: 'chat-3', title: '红烧肉的做法...', time: '输入中...' },
	];
</script>

<aside class="w-60 shrink-0 flex flex-col bg-[var(--app-bg)] border-r border-[var(--app-divider)]" aria-label="对话列表">
	<div class="flex items-center justify-between h-12 px-4 border-b border-[var(--app-divider)]">
		<span class="text-[11px] font-semibold text-[var(--app-text-muted)] uppercase tracking-widest">对话</span>
		<button type="button" class="p-2 rounded-lg text-[var(--app-text-muted)] hover:text-[var(--app-accent)] hover:bg-[var(--app-surface-hover)] transition-colors duration-200" title="新对话"><Plus size={14} /></button>
	</div>

	<div class="flex-1 overflow-y-auto py-3 min-h-0">
		{#each chats as chat (chat.id)}
			<button
				type="button"
				class="group relative w-full flex items-center gap-3 px-4 py-2.5 text-left transition-colors duration-200 {activeListItem === chat.id ? 'bg-[var(--app-surface-hover)]' : 'hover:bg-[var(--app-surface-hover)]'}"
				onclick={() => activeListItem = chat.id}
			>
				{#if activeListItem === chat.id}
					<span class="absolute left-0 top-1/2 -translate-y-1/2 w-0.5 h-6 bg-[var(--app-accent)] rounded-r" aria-hidden="true"></span>
				{/if}
				<MessageSquare size={15} class="shrink-0 {activeListItem === chat.id ? 'text-[var(--app-accent)]' : 'text-[var(--app-text-muted)]'}" />
				<div class="min-w-0 flex-1">
					<div class="text-[13px] font-medium text-[var(--app-text)] truncate">{chat.title}</div>
					<div class="text-[11px] text-[var(--app-text-muted)] mt-0.5">{chat.time}</div>
				</div>
				<MoreHorizontal size={14} class="shrink-0 opacity-0 group-hover:opacity-100 text-[var(--app-text-muted)] transition-opacity duration-200" />
			</button>
		{/each}

		<div class="h-px bg-[var(--app-divider)] my-3 mx-3" aria-hidden="true"></div>

		{#each channels as channel (channel.id)}
			{@const Icon = channel.icon}
			<button
				type="button"
				class="group relative w-full flex items-center gap-3 px-4 py-2.5 text-left transition-colors duration-200 {activeListItem === channel.id ? 'bg-[var(--app-surface-hover)]' : 'hover:bg-[var(--app-surface-hover)]'}"
				onclick={() => activeListItem = channel.id}
			>
				{#if activeListItem === channel.id}
					<span class="absolute left-0 top-1/2 -translate-y-1/2 w-0.5 h-6 bg-[var(--app-accent)] rounded-r" aria-hidden="true"></span>
				{/if}
				<div class="w-8 h-8 rounded-lg flex items-center justify-center shrink-0 {channel.bg} {channel.color}"><Icon size={13} /></div>
				<div class="min-w-0 flex-1 text-[13px] font-medium text-[var(--app-text)] truncate">{channel.title}</div>
				<MoreHorizontal size={14} class="shrink-0 opacity-0 group-hover:opacity-100 text-[var(--app-text-muted)] transition-opacity duration-200" />
			</button>
		{/each}
	</div>
</aside>
