<script lang="ts">
	import { X, Terminal, Trash2, RefreshCw } from '@lucide/svelte';
	import { invoke } from '@tauri-apps/api/core';

	let { isOpen = $bindable(false) } = $props();

	function closeLogs() {
		isOpen = false;
	}

	type LogLevel = 'debug' | 'info' | 'warn' | 'error';
	
	interface LogEntry {
		id: string;
		timestamp: string;
		level: LogLevel;
		source: string;
		message: string;
	}

	let logs = $state<LogEntry[]>([]);
	let isPolling = $state(false);
	let logsContainer = $state<HTMLDivElement | null>(null);
	let pollInterval: ReturnType<typeof setInterval>;
	let autoScroll = $state(true);

	async function fetchLogs() {
		try {
			// Limit to 200 to not overwhelm UI if things get crazy
			const fetchedLogs = await invoke<LogEntry[]>('logs_tail', { limit: 200 });
			logs = fetchedLogs;
			
			if (autoScroll && logsContainer) {
				setTimeout(() => {
					if (logsContainer) {
						logsContainer.scrollTop = logsContainer.scrollHeight;
					}
				}, 50);
			}
		} catch (e) {
			console.error("Failed to fetch logs:", e);
		}
	}

	async function clearLogs() {
		try {
			await invoke('logs_clear');
			logs = [];
		} catch (e) {
			console.error("Failed to clear logs:", e);
		}
	}

	$effect(() => {
		if (isOpen) {
			fetchLogs();
			isPolling = true;
			pollInterval = setInterval(fetchLogs, 2000);
		} else {
			isPolling = false;
			clearInterval(pollInterval);
		}
		
		return () => {
			clearInterval(pollInterval);
		};
	});

	function handleScroll() {
		if (logsContainer) {
			const { scrollTop, scrollHeight, clientHeight } = logsContainer;
			// If we scroll up, disable auto-scroll
			if (scrollHeight - scrollTop - clientHeight > 50) {
				autoScroll = false;
			} else {
				autoScroll = true;
			}
		}
	}

	// Helpers for log styling
	function getLevelColor(level: LogLevel) {
		switch (level) {
			case 'error': return 'text-red-500';
			case 'warn': return 'text-amber-500';
			case 'info': return 'text-blue-400';
			case 'debug': return 'text-muted-foreground/50';
			default: return 'text-foreground';
		}
	}
</script>

{#if isOpen}
	<div class="fixed inset-0 z-200 flex items-center justify-center bg-background/80 backdrop-blur-md animate-in fade-in duration-200 font-mono">
		<!-- Modal Container -->
		<div class="w-full max-w-5xl h-[85vh] max-h-[800px] bg-background/95 border border-border shadow-2xl flex flex-col relative overflow-hidden animate-in zoom-in-95 duration-300 slide-in-from-bottom-8 rounded-[4px]">
			
			<!-- Decorative Top Bar (Scanner effect) -->
			<div class="absolute top-0 left-0 w-full h-px bg-primary/20">
				<div class="absolute top-0 left-0 h-px w-32 bg-primary/80 animate-[scan_3s_ease-in-out_infinite_alternate]"></div>
			</div>

			<!-- Header -->
			<div class="h-12 border-b border-border/60 flex items-center justify-between px-4 shrink-0 bg-muted/30 dark:bg-muted/10">
				<div class="flex items-center gap-3">
					<Terminal class="w-4 h-4 text-primary" />
					<h2 class="text-[11px] font-bold tracking-[0.3em] uppercase text-foreground/80">Daemon System Logs</h2>
					
					{#if isPolling}
						<div class="flex items-center gap-1.5 ml-4 px-2 py-0.5 rounded-sm bg-primary/10 border border-primary/20">
							<span class="w-1.5 h-1.5 rounded-full bg-primary animate-pulse"></span>
							<span class="text-[9px] text-primary uppercase tracking-widest font-bold">Live Stream</span>
						</div>
					{:else}
						<div class="flex items-center gap-1.5 ml-4 px-2 py-0.5 rounded-sm bg-muted border border-border">
							<span class="w-1.5 h-1.5 rounded-full bg-muted-foreground"></span>
							<span class="text-[9px] text-muted-foreground uppercase tracking-widest font-bold">Offline</span>
						</div>
					{/if}
				</div>
				<div class="flex items-center gap-1">
					<button onclick={fetchLogs} class="p-1.5 hover:bg-muted text-muted-foreground hover:text-foreground rounded-sm transition-colors group relative" title="Force Refresh">
						<RefreshCw class="w-4 h-4 {isPolling && 'group-active:animate-spin'}" />
					</button>
					<button onclick={clearLogs} class="p-1.5 hover:bg-red-500/20 text-muted-foreground hover:text-red-400 rounded-sm transition-colors group relative" title="Clear Logs">
						<Trash2 class="w-4 h-4" />
					</button>
					<div class="w-px h-4 bg-border/50 mx-1"></div>
					<button onclick={closeLogs} class="p-1.5 hover:bg-red-500/20 text-muted-foreground hover:text-red-400 rounded-sm transition-colors group">
						<X class="w-4 h-4 group-hover:rotate-90 transition-transform duration-300" />
					</button>
				</div>
			</div>

			<!-- Log Output Area -->
			<div 
				bind:this={logsContainer}
				onscroll={handleScroll}
				class="flex-1 overflow-y-auto p-4 bg-card/50 text-[12px] font-mono leading-relaxed tracking-wide scrollbar-thin scrollbar-track-transparent scrollbar-thumb-muted-foreground/20"
			>
				{#if logs.length === 0}
					<div class="flex flex-col items-center justify-center h-full text-muted-foreground/30 gap-4 opacity-50 select-none">
						<Terminal class="w-12 h-12" />
						<div class="text-[10px] uppercase tracking-[0.3em]">No Log Telemetry Data Detected</div>
					</div>
				{:else}
					<div class="space-y-1">
						{#each logs as log (log.id)}
							<div class="flex gap-3 hover:bg-muted/50 dark:hover:bg-white/5 px-2 py-1 -mx-2 rounded-sm transition-colors group">
								<div class="text-muted-foreground/60 shrink-0 w-[140px] tabular-nums font-bold">
									[{log.timestamp}]
								</div>
								
								<div class={`w-14 uppercase font-bold tracking-wider text-[10px] shrink-0 pt-0.5 ${getLevelColor(log.level)}`}>
									{log.level}
								</div>
								
								<div class="w-24 text-primary/70 shrink-0 truncate text-[11px] pt-px" title={log.source}>
									{log.source}
								</div>
								
								<div class={`flex-1 text-wrap ${log.level === 'error' ? 'text-red-400/90 font-medium' : 'text-foreground/80'}`}>
									{log.message}
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div class="h-8 border-t border-border/60 bg-muted/30 dark:bg-muted/10 flex items-center justify-between px-3 text-[10px] text-muted-foreground uppercase tracking-widest shrink-0">
				<div>ZCLAW / SYS_LOGS / V.0.4.1</div>
				<div class="flex items-center gap-3">
					<label class="flex items-center gap-1.5 cursor-pointer hover:text-foreground transition-colors">
						<input type="checkbox" bind:checked={autoScroll} class="accent-primary w-3 h-3 bg-transparent border-border rounded-[2px]" />
						<span>Auto-Scroll Tail</span>
					</label>
					<span>{logs.length} EXTRIES</span>
				</div>
			</div>
			
			<style>
				@keyframes scan {
					0% { transform: translateX(0); }
					100% { transform: translateX(100vw); }
				}
			</style>
		</div>
	</div>
{/if}
