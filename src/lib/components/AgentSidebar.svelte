<script lang="ts">
	import { Plus, Settings, HelpCircle } from '@lucide/svelte';

	let { activeAgentId = $bindable() }: { activeAgentId: number } = $props();

	const agents = [
		{ id: 1, color: 'bg-blue-600', name: '苏东坡', icon: '🤖', description: 'AI coworker agent' },
		{ id: 2, color: 'bg-rose-600', name: '李白', icon: '🍷', description: '创意写作专家' },
		{ id: 3, color: 'bg-amber-600', name: '杜甫', icon: '✍️', description: '逻辑分析助手' },
		{ id: 4, color: 'bg-emerald-600', name: '王维', icon: '🌿', description: '视觉艺术顾问' },
	];
</script>

<aside class="w-[72px] shrink-0 flex flex-col items-center py-4 bg-[var(--app-bg)] border-r border-[var(--app-divider)]" aria-label="Agent 列表">
	<div class="flex flex-col gap-1 flex-1">
		{#each agents as agent (agent.id)}
			<button
				class="relative flex items-center justify-center w-11 h-11 rounded-xl transition-colors duration-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-[var(--app-accent)]/50 {activeAgentId === agent.id ? 'bg-[var(--app-surface-hover)]' : 'hover:bg-[var(--app-surface-hover)]'}"
				onclick={() => activeAgentId = agent.id}
				title={agent.name}
			>
				{#if activeAgentId === agent.id}
					<span class="absolute left-0 top-1/2 -translate-y-1/2 w-0.5 h-6 bg-[var(--app-accent)] rounded-r" aria-hidden="true"></span>
				{/if}
				<span class="w-9 h-9 rounded-lg {agent.color} flex items-center justify-center text-lg">{agent.icon}</span>
			</button>
		{/each}
		<button type="button" class="w-11 h-11 rounded-xl border border-dashed border-[var(--app-divider)] flex items-center justify-center text-[var(--app-text-muted)] hover:border-[var(--app-accent)]/40 hover:text-[var(--app-accent)] transition-colors duration-200 focus:outline-none mt-2" title="添加 Agent">
			<Plus size={18} />
		</button>
	</div>
	<div class="flex flex-col gap-0.5 pt-3 border-t border-[var(--app-divider)]">
		<button type="button" class="w-11 h-11 rounded-xl flex items-center justify-center text-[var(--app-text-muted)] hover:text-[var(--app-text)] hover:bg-[var(--app-surface-hover)] transition-colors duration-200 focus:outline-none" title="设置"><Settings size={18} /></button>
		<button type="button" class="w-11 h-11 rounded-xl flex items-center justify-center text-[var(--app-text-muted)] hover:text-[var(--app-text)] hover:bg-[var(--app-surface-hover)] transition-colors duration-200 focus:outline-none" title="帮助"><HelpCircle size={18} /></button>
	</div>
</aside>
