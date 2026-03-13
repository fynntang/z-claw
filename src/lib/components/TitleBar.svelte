<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { Bell, Minus, Moon, Settings, Square, Sun, X } from '@lucide/svelte';
	import { onMount } from 'svelte';
	import { showSettings } from '$lib/stores/app';
	import { mode as darkMode, toggleMode } from 'mode-watcher';
	import { Button } from '$lib/components/ui/button';

	const appWindow = getCurrentWindow();

	let isMaximized = $state(false);
	let hasNotification = $state(false);

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
		// TODO: 打开通知面板
	}

	function openSettings() {
		showSettings.set(true);
	}
</script>

<svelte:head>
	<link rel="icon" href="/icon.ico" type="image/x-icon" />
</svelte:head>

<div class="titlebar" data-tauri-drag-region>
	<div class="titlebar-drag" data-tauri-drag-region>
		<img src="/icon.ico" alt="ZClaw" class="app-icon" />
		<span class="app-title">ZClaw</span>
	</div>

	<div class="titlebar-actions">
		<!-- 通知按钮 -->
		<Button
			variant="ghost"
			size="icon"
			class="action-btn"
			onclick={toggleNotification}
			title="通知"
		>
			<Bell size={16} />
			{#if hasNotification}
				<span class="notification-dot"></span>
			{/if}
		</Button>

		<!-- Dark Mode 切换 -->
		<Button
			variant="ghost"
			size="icon"
			class="action-btn"
			onclick={toggleMode}
			title={darkMode.current === 'dark' ? '切换到亮色模式' : '切换到暗色模式'}
		>
			{#if darkMode.current === 'dark'}
				<Sun size={16} class="theme-icon icon-sun" />
			{:else}
				<Moon size={16} class="theme-icon icon-moon" />
			{/if}
		</Button>

		<!-- 设置按钮 -->
		<Button variant="ghost" size="icon" class="action-btn" onclick={openSettings} title="设置">
			<Settings size={16} />
		</Button>

		<div class="separator"></div>

		<!-- 窗口控制 -->
		<Button
			variant="ghost"
			size="icon"
			class="action-btn control"
			onclick={minimize}
			title="最小化"
		>
			<Minus size={16} />
		</Button>
		<Button
			variant="ghost"
			size="icon"
			class="action-btn control"
			onclick={toggleMaximize}
			title={isMaximized ? '还原' : '最大化'}
		>
			<Square size={12} />
		</Button>
		<Button
			variant="ghost"
			size="icon"
			class="action-btn control close"
			onclick={close}
			title="关闭"
		>
			<X size={16} />
		</Button>
	</div>
</div>

<style>
	.titlebar {
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0 4px;
		user-select: none;
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		z-index: 9999;
		backdrop-filter: blur(12px);
		transition:
			background 0.2s ease,
			border-color 0.2s ease;
	}

	/* Dark mode */
	:global(html.dark) .titlebar {
		background: linear-gradient(180deg, rgba(24, 24, 27, 0.95) 0%, rgba(18, 18, 20, 0.98) 100%);
		border-bottom: 1px solid rgba(39, 39, 42, 0.5);
	}

	/* Light mode */
	:global(html:not(.dark)) .titlebar {
		background: linear-gradient(180deg, rgba(255, 255, 255, 0.98) 0%, rgba(250, 250, 250, 1) 100%);
		border-bottom: 1px solid rgba(228, 228, 231, 1);
	}

	.titlebar-drag {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 0 8px;
		flex: 1;
		height: 100%;
	}

	.app-icon {
		width: 18px;
		height: 18px;
		object-fit: contain;
	}

	.app-title {
		font-size: 12px;
		font-weight: 600;
		color: #a1a1aa;
		letter-spacing: 0.02em;
		transition: color 0.2s ease;
	}

	:global(html:not(.dark)) .app-title {
		color: #52525b;
	}

	.titlebar-actions {
		display: flex;
		align-items: center;
		height: 100%;
	}

	/* Unified action button style using UI library Button */
	:global(.action-btn) {
		width: 40px !important;
		height: 32px !important;
		border-radius: 6px !important;
		color: #71717a !important;
		position: relative;
	}

	:global(html.dark) :global(.action-btn:hover) {
		background: rgba(63, 63, 70, 0.5) !important;
		color: #e4e4e7 !important;
	}

	:global(html:not(.dark)) :global(.action-btn:hover) {
		background: rgba(228, 228, 231, 1) !important;
		color: #18181b !important;
	}

	/* Theme toggle icon visibility */
	:global(.theme-icon) {
		position: absolute;
		transition:
			opacity 0.2s,
			transform 0.2s;
	}

	:global(.icon-sun) {
		opacity: 1;
		transform: rotate(0deg);
	}

	:global(.icon-moon) {
		opacity: 0;
		transform: rotate(-90deg);
	}

	:global(html.dark) :global(.icon-sun) {
		opacity: 0;
		transform: rotate(90deg);
	}

	:global(html.dark) :global(.icon-moon) {
		opacity: 1;
		transform: rotate(0deg);
	}

	/* Notification dot */
	.notification-dot {
		position: absolute;
		top: 6px;
		right: 8px;
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: #ef4444;
		box-shadow: 0 0 4px rgba(239, 68, 68, 0.5);
	}

	/* Separator */
	.separator {
		width: 1px;
		height: 16px;
		background: rgba(63, 63, 70, 0.5);
		margin: 0 4px;
		transition: background 0.2s ease;
	}

	:global(html:not(.dark)) .separator {
		background: rgba(228, 228, 231, 1);
	}

	/* Window control buttons - slightly wider */
	:global(.action-btn.control) {
		width: 46px !important;
	}

	/* Close button hover - red */
	:global(.action-btn.close:hover) {
		background: #dc2626 !important;
		color: white !important;
	}
</style>
