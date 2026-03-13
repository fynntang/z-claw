<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { ToolInfo } from '$lib/types/tools';
	import { TOOL_CATEGORIES } from '$lib/types/tools';

	let tools: ToolInfo[] = $state([]);
	let loading = $state(true);
	let error = $state('');
	let selectedCategory = $state('');

	onMount(async () => {
		await loadTools();
	});

	async function loadTools() {
		loading = true;
		error = '';
		try {
			tools = await invoke<ToolInfo[]>('tools_list');
		} catch (e) {
			error = String(e);
		} finally {
			loading = false;
		}
	}

	let filteredTools = $derived(
		selectedCategory ? tools.filter((t) => t.category === selectedCategory) : tools
	);

	let groupedTools = $derived(
		filteredTools.reduce(
			(acc, tool) => {
				if (!acc[tool.category]) {
					acc[tool.category] = [];
				}
				acc[tool.category].push(tool);
				return acc;
			},
			{} as Record<string, ToolInfo[]>
		)
	);

	function getCategoryInfo(categoryId: string) {
		return TOOL_CATEGORIES.find((c) => c.id === categoryId) || { label: categoryId, icon: '📦' };
	}
</script>

<div class="flex h-full flex-col">
	<!-- Header -->
	<div class="flex items-center justify-between border-b border-zinc-800 px-4 py-3">
		<h2 class="text-lg font-semibold text-zinc-100">工具</h2>
		<span class="text-xs text-zinc-500">{tools.length} 个可用</span>
	</div>

	<!-- Category Filter -->
	<div class="flex flex-wrap gap-1 border-b border-zinc-800 px-4 py-2">
		<button
			class="rounded px-2 py-1 text-xs transition-colors {!selectedCategory
				? 'bg-blue-600 text-white'
				: 'bg-zinc-800 text-zinc-400 hover:bg-zinc-700'}"
			onclick={() => (selectedCategory = '')}
		>
			全部
		</button>
		{#each TOOL_CATEGORIES as cat}
			{@const count = tools.filter((t) => t.category === cat.id).length}
			{#if count > 0}
				<button
					class="rounded px-2 py-1 text-xs transition-colors {selectedCategory === cat.id
						? 'bg-blue-600 text-white'
						: 'bg-zinc-800 text-zinc-400 hover:bg-zinc-700'}"
					onclick={() => (selectedCategory = cat.id)}
				>
					{cat.icon}
					{cat.label}
				</button>
			{/if}
		{/each}
	</div>

	<!-- Content -->
	<div class="flex-1 overflow-y-auto p-4">
		{#if loading}
			<div class="flex h-32 items-center justify-center">
				<div class="text-zinc-500">加载中...</div>
			</div>
		{:else if error}
			<div class="rounded border border-red-800 bg-red-900/20 p-4 text-sm text-red-400">
				{error}
			</div>
		{:else}
			{#each Object.entries(groupedTools) as [category, categoryTools]}
				<div class="mb-4">
					<div class="mb-2 flex items-center gap-2">
						<span class="text-lg">{getCategoryInfo(category).icon}</span>
						<span class="text-sm font-medium text-zinc-300">{getCategoryInfo(category).label}</span>
						<span class="text-xs text-zinc-600">({categoryTools.length})</span>
					</div>
					<div class="space-y-2">
						{#each categoryTools as tool}
							<div
								class="rounded border border-zinc-800 bg-zinc-900/50 p-3 transition-colors hover:border-zinc-700"
							>
								<div class="flex items-start justify-between">
									<code class="font-mono text-sm text-blue-400">{tool.name}</code>
								</div>
								<p class="mt-1 text-xs text-zinc-500">{tool.description}</p>
							</div>
						{/each}
					</div>
				</div>
			{/each}
		{/if}
	</div>
</div>
