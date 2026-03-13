<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { invoke } from '@tauri-apps/api/core';
	import { Bell, Minus, Moon, Settings, Square, Sun, X } from '@lucide/svelte';
	import { onMount } from 'svelte';
	import { showSettings } from '$lib/stores/app';
	import { mode as darkMode, toggleMode } from 'mode-watcher';
	import { Button } from '$lib/components/ui/button';

	const appWindow = getCurrentWindow();

	let isMaximized = $state(false);
	let hasNotification = $state(true);
	/** 运行时状态：provider / model，来自 get_status */
	let runtimeStatus = $state<{ provider: string; model: string } | null>(null);

	onMount(() => {
		let unlisten: (() => void) | undefined;

		appWindow.isMaximized().then((maximized) => {
			isMaximized = maximized;
		});

		appWindow
			.onResized(async () => {
				isMaximized = await appWindow.isMaximized();
			})
			.then((fn) => {
				unlisten = fn;
			});

		invoke<{ provider: string; model: string }>('get_status')
			.then((s) => {
				runtimeStatus = { provider: s.provider, model: s.model };
			})
			.catch(() => {
				runtimeStatus = null;
			});

		return () => {
			unlisten?.();
		};
	});

	async function minimize() {
		await appWindow.minimize();
	}

	async function toggleMaximize() {
		await appWindow.toggleMaximize();
	}

	async function close() {
		await appWindow.close();
	}

	function toggleNotification() {
		hasNotification = !hasNotification;
	}

	function openSettings() {
		showSettings.set(true);
	}
</script>

<style>
	.title-bar {
		background: rgba(17, 17, 19, 0.72);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		border-bottom: 1px solid var(--app-border);
	}
</style>

<!-- 毛玻璃标题栏，与设计系统一致 -->
<div class="title-bar h-[48px] shrink-0 flex items-center justify-between px-4 select-none z-50">
	<div class="flex items-center gap-3 flex-1 h-full min-w-0" data-tauri-drag-region>
		<div class="w-7 h-7 rounded-lg flex items-center justify-center bg-[var(--app-accent)]/90 text-[var(--app-bg)] font-bold text-sm tracking-tight pointer-events-none shadow-lg shadow-[var(--app-accent)]/20">Z</div>
		<span class="text-sm font-semibold text-[var(--app-text)] tracking-tight pointer-events-none">ZClaw</span>
		{#if runtimeStatus && (runtimeStatus.provider !== 'none' || runtimeStatus.model !== 'none')}
			<span class="text-xs text-[var(--app-text-muted)] pointer-events-none truncate">
				{runtimeStatus.provider} · {runtimeStatus.model}
			</span>
		{:else if runtimeStatus}
			<span class="text-xs text-[var(--app-text-muted)]/70 pointer-events-none">未就绪</span>
		{/if}
	</div>

	<div class="flex items-center shrink-0">
		<div class="flex items-center gap-0.5 px-2">
			<button class="w-8 h-8 flex items-center justify-center rounded-lg text-[var(--app-text-secondary)] hover:text-[var(--app-accent)] hover:bg-[var(--app-surface-hover)] transition-colors relative" onclick={toggleNotification} title="通知">
				<Bell size={15} />
				{#if hasNotification}
					<span class="absolute top-[10px] right-[10px] w-1.5 h-1.5 bg-[var(--app-accent)] rounded-full"></span>
				{/if}
			</button>
			<button class="w-8 h-8 flex items-center justify-center rounded-lg text-[var(--app-text-secondary)] hover:text-[var(--app-accent)] hover:bg-[var(--app-surface-hover)] transition-colors" onclick={toggleMode} title={darkMode.current === 'dark' ? '切换到亮色模式' : '切换到暗色模式'}>
				{#if darkMode.current === 'dark'}
					<Sun size={15} />
				{:else}
					<Moon size={15} />
				{/if}
			</button>
			<button class="w-8 h-8 flex items-center justify-center rounded-lg text-[var(--app-text-secondary)] hover:text-[var(--app-accent)] hover:bg-[var(--app-surface-hover)] transition-colors" onclick={openSettings} title="设置">
				<Settings size={15} />
			</button>
		</div>
		<div class="w-px h-5 bg-[var(--app-border)] mx-1"></div>
		<div class="flex items-center gap-0.5">
			<button class="w-8 h-8 flex items-center justify-center rounded-md text-[var(--app-text-secondary)] hover:text-[var(--app-text)] hover:bg-[var(--app-surface-hover)] transition-colors" onclick={minimize} title="最小化">
				<Minus size={14} />
			</button>
			<button class="w-8 h-8 flex items-center justify-center rounded-md text-[var(--app-text-secondary)] hover:text-[var(--app-text)] hover:bg-[var(--app-surface-hover)] transition-colors" onclick={toggleMaximize} title={isMaximized ? '还原' : '最大化'}>
				<Square size={12} />
			</button>
			<button class="w-8 h-8 flex items-center justify-center rounded-md text-[var(--app-text-secondary)] hover:text-white hover:bg-red-500/90 transition-colors" onclick={close} title="关闭">
				<X size={14} />
			</button>
		</div>
	</div>
</div>
