<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { invoke } from '@tauri-apps/api/core';
	import { Bell, Minus, Moon, Settings, Square, Sun, X } from '@lucide/svelte';
	import { onMount } from 'svelte';
	import { showSettings } from '$lib/stores/app';
	import { mode as darkMode, toggleMode } from 'mode-watcher';

	const appWindow = getCurrentWindow();
	let isMaximized = $state(false);
	let hasNotification = $state(true);
	let runtimeStatus = $state<{ provider: string; model: string } | null>(null);

	onMount(() => {
		let unlisten: (() => void) | undefined;
		appWindow.isMaximized().then((max) => { isMaximized = max; });
		appWindow.onResized(async () => { isMaximized = await appWindow.isMaximized(); }).then((fn) => { unlisten = fn; });
		invoke<{ provider: string; model: string }>('get_status').then((s) => { runtimeStatus = s; }).catch(() => { runtimeStatus = null; });
		return () => { unlisten?.(); };
	});

	async function minimize() { await appWindow.minimize(); }
	async function toggleMaximize() { await appWindow.toggleMaximize(); }
	async function close() { await appWindow.close(); }
	function openSettings() { showSettings.set(true); }
	function toggleNotification() { hasNotification = !hasNotification; }
</script>

<header class="title-bar h-12 shrink-0 flex items-center justify-between px-5 border-b border-[var(--app-divider)] bg-[var(--app-bg)]/80 backdrop-blur-md select-none z-50">
	<div class="flex items-center gap-4 min-w-0 flex-1" data-tauri-drag-region>
		<span class="text-[15px] font-semibold tracking-tight text-[var(--app-accent)]">ZClaw</span>
		{#if runtimeStatus}
			<span class="text-[13px] text-[var(--app-text-muted)] truncate tabular-nums">
				{runtimeStatus.provider === 'none' ? '未就绪' : `${runtimeStatus.provider} · ${runtimeStatus.model}`}
			</span>
		{/if}
	</div>

	<nav class="flex items-center shrink-0 gap-0.5" aria-label="应用操作">
		<button type="button" class="p-2 rounded-md text-[var(--app-text-muted)] hover:text-[var(--app-text)] hover:bg-[var(--app-surface-hover)] transition-colors duration-200 relative" onclick={toggleNotification} title="通知">
			<Bell size={16} />
			{#if hasNotification}
				<span class="absolute top-1.5 right-1.5 w-1.5 h-1.5 bg-[var(--app-accent)] rounded-full" aria-hidden="true"></span>
			{/if}
		</button>
		<button type="button" class="p-2 rounded-md text-[var(--app-text-muted)] hover:text-[var(--app-text)] hover:bg-[var(--app-surface-hover)] transition-colors duration-200" onclick={toggleMode} title="主题">
			{#if darkMode.current === 'dark'}<Sun size={16} />{:else}<Moon size={16} />{/if}
		</button>
		<button type="button" class="p-2 rounded-md text-[var(--app-text-muted)] hover:text-[var(--app-text)] hover:bg-[var(--app-surface-hover)] transition-colors duration-200" onclick={openSettings} title="设置">
			<Settings size={16} />
		</button>
		<div class="w-px h-5 bg-[var(--app-divider)] mx-2" aria-hidden="true"></div>
		<button type="button" class="p-2 rounded-md text-[var(--app-text-muted)] hover:text-[var(--app-text)] hover:bg-[var(--app-surface-hover)] transition-colors duration-200" onclick={minimize} title="最小化"><Minus size={14} /></button>
		<button type="button" class="p-2 rounded-md text-[var(--app-text-muted)] hover:text-[var(--app-text)] hover:bg-[var(--app-surface-hover)] transition-colors duration-200" onclick={toggleMaximize} title={isMaximized ? '还原' : '最大化'}><Square size={12} /></button>
		<button type="button" class="p-2 rounded-md text-[var(--app-text-muted)] hover:text-white hover:bg-red-600/90 transition-colors duration-200" onclick={close} title="关闭"><X size={14} /></button>
	</nav>
</header>
