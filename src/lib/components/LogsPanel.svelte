<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { LogEntry, LogLevel } from '$lib/types/logs';
	import { LOG_LEVELS } from '$lib/types/logs';

	let logs: LogEntry[] = $state([]);
	let loading = true;
	let error = '';
	let selectedLevel: LogLevel | '' = $state('');
	let autoRefresh = true;
	let refreshInterval: ReturnType<typeof setInterval> | null = null;

	onMount(async () => {
		await loadLogs();
		startAutoRefresh();
	});

	onDestroy(() => {
		stopAutoRefresh();
	});

	async function loadLogs() {
		try {
			const level = selectedLevel || undefined;
			logs = await invoke<LogEntry[]>('logs_tail', { limit: 100, level });
		} catch (e) {
			error = String(e);
		} finally {
			loading = false;
		}
	}

	function startAutoRefresh() {
		if (autoRefresh && !refreshInterval) {
			refreshInterval = setInterval(loadLogs, 3000);
		}
	}

	function stopAutoRefresh() {
		if (refreshInterval) {
			clearInterval(refreshInterval);
			refreshInterval = null;
		}
	}

	async function clearLogs() {
		try {
			await invoke('logs_clear');
			logs = [];
		} catch (e) {
			error = String(e);
		}
	}

	function getLevelColor(level: LogLevel): string {
		switch (level) {
			case 'debug':
				return 'text-zinc-500';
			case 'info':
				return 'text-blue-400';
			case 'warn':
				return 'text-yellow-400';
			case 'error':
				return 'text-red-400';
			default:
				return 'text-zinc-400';
		}
	}

	function getLevelBg(level: LogLevel): string {
		switch (level) {
			case 'debug':
				return 'bg-zinc-800';
			case 'info':
				return 'bg-blue-900/30';
			case 'warn':
				return 'bg-yellow-900/30';
			case 'error':
				return 'bg-red-900/30';
			default:
				return 'bg-zinc-800';
		}
	}

	let filteredLogs = $derived(selectedLevel ? logs.filter((l) => l.level === selectedLevel) : logs);

	$effect(() => {
		selectedLevel;
		loadLogs();
	});
</script>

<div class="flex h-full flex-col">
	<!-- Header -->
	<div class="flex items-center justify-between border-b border-zinc-800 px-4 py-3">
		<h2 class="text-lg font-semibold text-zinc-100">日志</h2>
		<div class="flex items-center gap-2">
			<label class="flex items-center gap-1 text-xs text-zinc-500">
				<input
					type="checkbox"
					bind:checked={autoRefresh}
					on:change={() => {
						if (autoRefresh) startAutoRefresh();
						else stopAutoRefresh();
					}}
				/>
				自动刷新
			</label>
			<button
				class="rounded bg-zinc-800 px-2 py-1 text-xs text-zinc-400 transition-colors hover:bg-zinc-700"
				on:click={loadLogs}
			>
				刷新
			</button>
			<button
				class="rounded bg-red-900/30 px-2 py-1 text-xs text-red-400 transition-colors hover:bg-red-900/50"
				on:click={clearLogs}
			>
				清空
			</button>
		</div>
	</div>

	<!-- Level Filter -->
	<div class="flex flex-wrap gap-1 border-b border-zinc-800 px-4 py-2">
		<button
			class="rounded px-2 py-1 text-xs transition-colors {!selectedLevel
				? 'bg-blue-600 text-white'
				: 'bg-zinc-800 text-zinc-400 hover:bg-zinc-700'}"
			on:click={() => (selectedLevel = '')}
		>
			全部
		</button>
		{#each LOG_LEVELS as level}
			<button
				class="rounded px-2 py-1 text-xs transition-colors {selectedLevel === level.id
					? 'bg-blue-600 text-white'
					: 'bg-zinc-800 text-zinc-400 hover:bg-zinc-700'}"
				on:click={() => (selectedLevel = level.id)}
			>
				{level.label}
			</button>
		{/each}
	</div>

	<!-- Content -->
	<div class="flex-1 overflow-y-auto">
		{#if loading}
			<div class="flex h-32 items-center justify-center">
				<div class="text-zinc-500">加载中...</div>
			</div>
		{:else if error}
			<div class="m-4 rounded border border-red-800 bg-red-900/20 p-4 text-sm text-red-400">
				{error}
			</div>
		{:else if filteredLogs.length === 0}
			<div class="flex h-32 items-center justify-center">
				<div class="text-zinc-500">暂无日志</div>
			</div>
		{:else}
			<div class="divide-y divide-zinc-800/50">
				{#each filteredLogs as log}
					<div class="px-4 py-2 transition-colors hover:bg-zinc-900/50 {getLevelBg(log.level)}">
						<div class="mb-1 flex items-center gap-2">
							<span class="font-mono text-xs {getLevelColor(log.level)} uppercase">
								{log.level}
							</span>
							<span class="text-xs text-zinc-600">{log.timestamp}</span>
							<span class="rounded bg-zinc-800 px-1 text-xs text-zinc-500">{log.source}</span>
						</div>
						<p class="font-mono text-sm break-all text-zinc-300">{log.message}</p>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>
