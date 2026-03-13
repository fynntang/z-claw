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

<!-- Changed to transparent backdrop and removed bottom border -->
<div class="h-[46px] shrink-0 flex items-center justify-between px-4 select-none z-50">
	<div class="flex items-center gap-3 flex-1 h-full" data-tauri-drag-region>
		<!-- Placeholder icon, styling ZClaw logo like the sketch (Orange 'Z') -->
		<div class="w-6 h-6 rounded flex items-center justify-center bg-gradient-to-tr from-orange-600 to-yellow-500 shadow-lg text-black font-bold text-xs pointer-events-none">Z</div>
		<span class="text-sm font-bold text-zinc-100 pointer-events-none tracking-widest">ZClaw</span>
		{#if runtimeStatus && (runtimeStatus.provider !== 'none' || runtimeStatus.model !== 'none')}
			<span class="text-xs text-zinc-500 pointer-events-none">
				{runtimeStatus.provider} · {runtimeStatus.model}
			</span>
		{:else if runtimeStatus}
			<span class="text-xs text-zinc-600 pointer-events-none">未就绪</span>
		{/if}
	</div>

	<div class="flex items-center">
		<!-- Action Buttons -->
		<div class="flex items-center gap-1.5 px-3">
			<button class="w-8 h-8 flex items-center justify-center rounded-lg text-zinc-400 hover:text-yellow-500 hover:bg-white/10 transition-colors relative" onclick={toggleNotification} title="通知">
				<Bell size={15} />
				{#if hasNotification}
					<span class="absolute top-[8px] right-[8px] w-1.5 h-1.5 bg-orange-500 rounded-full"></span>
				{/if}
			</button>
			
			<button class="w-8 h-8 flex items-center justify-center rounded-lg text-zinc-400 hover:text-blue-400 hover:bg-white/10 transition-colors" onclick={toggleMode} title={darkMode.current === 'dark' ? '切换到亮色模式' : '切换到暗色模式'}>
				{#if darkMode.current === 'dark'}
					<Sun size={15} />
				{:else}
					<Moon size={15} />
				{/if}
			</button>
			
			<button class="w-8 h-8 flex items-center justify-center rounded-lg text-zinc-400 hover:text-white hover:bg-white/10 transition-colors" onclick={openSettings} title="设置">
				<Settings size={15} />
			</button>
		</div>

		<div class="w-px h-4 bg-white/10 mx-2"></div>

		<!-- Window Controls -->
		<div class="flex items-center gap-1">
			<button class="w-9 h-8 flex items-center justify-center rounded-md text-zinc-400 hover:text-white hover:bg-white/10 transition-colors" onclick={minimize} title="最小化">
				<Minus size={15} />
			</button>
			<button class="w-9 h-8 flex items-center justify-center rounded-md text-zinc-400 hover:text-white hover:bg-white/10 transition-colors" onclick={toggleMaximize} title={isMaximized ? '还原' : '最大化'}>
				<Square size={13} />
			</button>
			<button class="w-9 h-8 flex items-center justify-center rounded-md text-zinc-400 hover:text-white hover:bg-red-500 transition-colors" onclick={close} title="关闭">
				<X size={15} />
			</button>
		</div>
	</div>
</div>
