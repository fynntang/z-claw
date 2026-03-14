<script lang="ts">
	import { Minus, X, Zap, Bell, Moon, Settings } from '@lucide/svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import { toggleMode } from 'mode-watcher';
	import SettingsModal from './Settings.svelte';
	
	// Temporarily bypass strict typing for appWindow since we conditionally load it
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	let appWindow: any;
	
	let isSettingsOpen = $state(false);

	onMount(async () => {
		// Only import if running inside Tauri
		// @ts-expect-error __TAURI__ is injected by Tauri
		if (window.__TAURI__) {
			appWindow = getCurrentWindow();
		}
	});

	function minimize() {
		appWindow?.minimize();
	}

	function toggleMaximize() {
		appWindow?.toggleMaximize();
	}

	function closeWindow() {
		appWindow?.close();
	}
</script>

<div data-tauri-drag-region class="h-8 flex items-center justify-between bg-muted/80 dark:bg-black/40 backdrop-blur-xl border-b border-border/60 dark:border-white/5 select-none w-full relative z-100 font-mono text-muted-foreground transition-all">
	<!-- Top highlight line -->
	<div class="absolute top-0 left-0 w-full h-px bg-linear-to-r from-transparent via-primary/30 to-transparent opacity-50 pointer-events-none"></div>

	<!-- Left: Logo/Title -->
	<div data-tauri-drag-region class="flex items-center gap-2 pl-3 pointer-events-none h-full w-full">
		<Zap class="w-3.5 h-3.5 text-primary animate-pulse" />
		<span class="text-[9px] font-bold tracking-[0.3em] uppercase text-foreground/70">ZClaw // SYS_CORE</span>
	</div>
	
	<div class="flex h-full shrink-0">
		<!-- Action Group -->
		<div class="flex items-center gap-1 px-3 border-r border-border/60 dark:border-white/5 h-full mr-1">
			<button class="w-7 h-7 flex items-center justify-center rounded-sm hover:bg-foreground/10 text-muted-foreground/70 hover:text-foreground transition-all duration-300 relative group">
				<Bell class="w-3.5 h-3.5" />
				<!-- Notification dot placeholder -->
				<span class="absolute top-1.5 right-1.5 w-1.5 h-1.5 bg-primary rounded-full shadow-[0_0_5px_rgba(var(--color-primary),0.8)]"></span>
				
				<!-- Simple Tooltip -->
				<div class="absolute top-full left-1/2 -translate-x-1/2 mt-2 px-2 py-1 bg-popover/90 dark:bg-black/80 backdrop-blur-md border border-border/60 dark:border-white/10 text-[10px] text-popover-foreground rounded-sm opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none whitespace-nowrap z-50">
					Notifications
				</div>
			</button>

			<button 
				onclick={toggleMode}
				class="w-7 h-7 flex items-center justify-center rounded-sm hover:bg-foreground/10 text-muted-foreground/70 hover:text-foreground transition-all duration-300 relative group"
			>
				<Moon class="w-3.5 h-3.5" />
				
				<div class="absolute top-full left-1/2 -translate-x-1/2 mt-2 px-2 py-1 bg-popover/90 dark:bg-black/80 backdrop-blur-md border border-border/60 dark:border-white/10 text-[10px] text-popover-foreground rounded-sm opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none whitespace-nowrap z-50">
					Theme Toggle
				</div>
			</button>

			<button 
				onclick={() => { isSettingsOpen = true; }}
				class="w-7 h-7 flex items-center justify-center rounded-sm hover:bg-foreground/10 text-muted-foreground/70 hover:text-foreground transition-all duration-300 relative group"
			>
				<Settings class="w-3.5 h-3.5" />
				
				<div class="absolute top-full left-1/2 -translate-x-1/2 mt-2 px-2 py-1 bg-popover/90 dark:bg-black/80 backdrop-blur-md border border-border/60 dark:border-white/10 text-[10px] text-popover-foreground rounded-sm opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none whitespace-nowrap z-50">
					Settings
				</div>
			</button>
		</div>

		<!-- Window Controls -->
		<button onclick={minimize} class="w-11 flex items-center justify-center hover:bg-foreground/10 text-muted-foreground/70 hover:text-foreground transition-colors focus:outline-none">
			<Minus class="w-3.5 h-3.5" />
		</button>
		<button onclick={toggleMaximize} class="w-11 flex items-center justify-center hover:bg-foreground/10 text-muted-foreground/70 hover:text-foreground transition-colors focus:outline-none relative">
			<!-- Custom maximization square for a more "tech" look -->
			<div class="w-2.5 h-2.5 border-t border-l border-current absolute top-[10px] left-[14px]"></div>
			<div class="w-2.5 h-2.5 border-b border-r border-current absolute bottom-[10px] right-[14px]"></div>
		</button>
		<button onclick={closeWindow} class="w-11 flex items-center justify-center hover:bg-red-500/90 text-muted-foreground/70 hover:text-background transition-colors focus:outline-none">
			<X class="w-3.5 h-3.5" />
		</button>
	</div>
</div>

<SettingsModal bind:isOpen={isSettingsOpen} />
