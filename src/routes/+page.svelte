<script lang="ts">
	/** Single-agent chat UI — one assistant (ZeroClaw), one conversation area, optional tools panel. */
	import { Send, Menu, Settings, Bot, TerminalSquare, Loader2, Wrench, Info, ChevronRight } from '@lucide/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { tick, onMount } from 'svelte';
	import SystemLogs from '$lib/components/SystemLogs.svelte';
	import SettingsModal from '$lib/components/Settings.svelte';

	let showToolsPanel = $state(false);
	function toggleToolsPanel() {
		showToolsPanel = !showToolsPanel;
	}

	type Message = { role: string; content: string; timestamp: string };
	let messages = $state<Message[]>([
		{ role: 'assistant', content: '你好，我是 ZClaw。有什么可以帮你的？', timestamp: new Date().toLocaleTimeString('zh-CN', { hour12: false, hour: '2-digit', minute: '2-digit' }) }
	]);
	
	let isLogsOpen = $state(false);
	let isSettingsOpen = $state(false);

	// Backend stats
	interface RuntimeStatus {
		running: boolean;
		provider: string;
		model: string;
	}

	interface ToolInfo {
		name: string;
		description: string;
		category: string;
	}

	let agentStatus = $state<RuntimeStatus>({ running: false, provider: 'loading...', model: 'loading...' });
	let agentTools = $state<ToolInfo[]>([]);

	onMount(async () => {
		try {
			agentStatus = await invoke<RuntimeStatus>('get_status');
			agentTools = await invoke<ToolInfo[]>('tools_list');
		} catch (e) {
			console.error("Failed to load backend status:", e);
		}
	});

	let input = $state('');
	let isWaiting = $state(false);
	let currentSessionId = $state<string | null>(null);
	let messagesContainer: HTMLDivElement;

	async function scrollToBottom() {
		await tick();
		if (messagesContainer) {
			messagesContainer.scrollTop = messagesContainer.scrollHeight;
		}
	}

	async function sendMessage() {
		if (!input.trim() || isWaiting) return;
		
		const timeString = new Date().toLocaleTimeString('en-US', { hour12: false, hour: '2-digit', minute: '2-digit' });
		const userMsg = input.trim();
		messages = [...messages, { role: 'user', content: userMsg, timestamp: timeString }];
		
		input = '';
		isWaiting = true;
		await scrollToBottom();
		
		try {
			// Read current config to pass along to backend
			interface ConfigModel {
				api_key: string;
				api_url: string;
				provider: string;
				model: string;
			}
			const config = await invoke<ConfigModel>('config_get');

			interface ChatResponse {
				content: string;
				success: boolean;
				error?: string;
				session_id?: string;
			}

			// Invoke the backend chat functionality
			const response = await invoke<ChatResponse>('chat', {
				message: userMsg,
				sessionId: currentSessionId,
				apiKey: config.api_key,
				apiUrl: config.api_url,
				model: config.model
			});
			
			const resTime = new Date().toLocaleTimeString('en-US', { hour12: false, hour: '2-digit', minute: '2-digit' });

			if (response.success) {
				if (response.session_id) {
					currentSessionId = response.session_id;
				}
				messages = [...messages, { 
					role: 'assistant', 
					content: response.content, 
					timestamp: resTime 
				}];
			} else {
				messages = [...messages, { 
					role: 'assistant', 
					content: `ERROR: ${response.error || 'Unknown failure'}`, 
					timestamp: resTime 
				}];
			}
		} catch (e) {
			const resTime = new Date().toLocaleTimeString('en-US', { hour12: false, hour: '2-digit', minute: '2-digit' });
			messages = [...messages, { 
				role: 'assistant', 
				content: `FATAL ERROR: ${String(e)}`, 
				timestamp: resTime 
			}];
		} finally {
			isWaiting = false;
			await scrollToBottom();
		}
	}
	
	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			sendMessage();
		}
	}
</script>

<div class="flex h-full w-full bg-background text-foreground antialiased overflow-hidden selection:bg-primary selection:text-primary-foreground">
	<!-- Left rail: single-agent branding + Logs / Settings -->
	<aside class="w-16 shrink-0 border-r border-border bg-card/50 flex flex-col items-center py-4 gap-2 z-20">
		<div class="w-10 h-10 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center mb-2" title="ZClaw">
			<Bot class="w-5 h-5 text-primary" />
		</div>
		<span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">ZClaw</span>
		<div class="flex-1"></div>
		<button
			type="button"
			onclick={() => { isLogsOpen = true; }}
			class="p-2.5 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted transition-colors"
			title="日志"
		>
			<TerminalSquare class="w-5 h-5" />
		</button>
		<button
			type="button"
			onclick={() => { isSettingsOpen = true; }}
			class="p-2.5 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted transition-colors"
			title="配置"
		>
			<Settings class="w-5 h-5" />
		</button>
	</aside>

	<!-- Main Chat Area -->
	<main class="flex-1 flex flex-col relative h-full min-w-0">
		<header class="h-14 border-b border-border flex items-center justify-between px-4 md:px-6 bg-background/95 backdrop-blur-sm shrink-0">
			<div class="flex items-center gap-3 min-w-0">
				<button type="button" class="p-2 hover:bg-muted rounded-lg md:hidden text-muted-foreground" aria-label="菜单">
					<Menu class="w-4 h-4" />
				</button>
				<span class="text-base font-semibold text-foreground truncate">ZClaw</span>
				<span class="text-xs text-muted-foreground font-mono truncate hidden sm:inline">{agentStatus.provider} · {agentStatus.model}</span>
			</div>
			<button
				type="button"
				onclick={toggleToolsPanel}
				class="flex items-center gap-2 px-3 py-2 rounded-lg text-sm font-medium transition-colors {showToolsPanel ? 'bg-primary/15 text-primary' : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
			>
				<Wrench class="w-4 h-4" />
				<span class="hidden sm:inline">工具与状态</span>
			</button>
		</header>
		
		<!-- Message List -->
		<div bind:this={messagesContainer} class="flex-1 overflow-y-auto py-6 pb-32 px-4 md:px-8 w-full max-w-3xl mx-auto scroll-smooth">
			<div class="flex flex-col gap-6">
				{#each messages as msg (msg.timestamp + msg.content.substring(0, 10))}
					<div class="flex flex-col {msg.role === 'user' ? 'items-end' : 'items-start'}">
						<div class="flex items-center gap-2 mb-1.5 text-muted-foreground text-xs {msg.role === 'user' ? 'flex-row-reverse' : 'flex-row'}">
							<span class="font-medium {msg.role === 'user' ? 'text-foreground' : 'text-primary'}">
								{msg.role === 'user' ? 'You' : 'ZClaw'}
							</span>
							<span class="tabular-nums">{msg.timestamp}</span>
						</div>
						<div
							class="max-w-[85%] rounded-xl px-4 py-3 text-sm leading-relaxed {msg.role === 'user'
								? 'bg-primary text-primary-foreground'
								: 'bg-muted/50 border border-border'}"
						>
							<p class="whitespace-pre-wrap {msg.content.startsWith('ERROR:') || msg.content.startsWith('FATAL ERROR:') ? 'text-destructive font-medium' : ''}">{msg.content}</p>
						</div>
					</div>
				{/each}

				{#if isWaiting}
					<div class="flex flex-col items-start">
						<div class="flex items-center gap-2 mb-1.5 text-xs text-muted-foreground">
							<span class="font-medium text-primary">ZClaw</span>
							<span class="animate-pulse">思考中…</span>
						</div>
						<div class="rounded-xl px-4 py-3 bg-muted/50 border border-border flex items-center gap-2">
							<Loader2 class="w-4 h-4 text-primary animate-spin shrink-0" />
							<span class="text-sm text-muted-foreground">正在执行</span>
						</div>
					</div>
				{/if}
			</div>
		</div>
		
		<!-- Input -->
		<div class="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-background to-transparent pt-8 pb-6 px-4">
			<form
				onsubmit={(e) => { e.preventDefault(); sendMessage(); }}
				class="max-w-3xl mx-auto flex items-end gap-2 rounded-xl border border-border bg-card/80 backdrop-blur-sm p-2 shadow-lg focus-within:ring-2 focus-within:ring-ring focus-within:border-transparent transition-shadow"
			>
				<textarea
					bind:value={input}
					onkeydown={handleKeydown}
					disabled={isWaiting}
					placeholder={isWaiting ? '正在处理…' : '输入消息，Enter 发送'}
					class="flex-1 min-h-11 max-h-36 resize-none bg-transparent border-0 py-2.5 px-3 text-sm placeholder:text-muted-foreground focus:outline-none disabled:opacity-50 rounded-lg"
					rows="1"
				></textarea>
				<button
					type="submit"
					disabled={!input.trim() || isWaiting}
					class="shrink-0 w-10 h-10 flex items-center justify-center rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 disabled:opacity-40 transition-colors"
					aria-label="发送"
				>
					<Send class="w-4 h-4" />
				</button>
			</form>
			<p class="text-center mt-2 text-xs text-muted-foreground">
				<kbd class="px-1 py-0.5 rounded bg-muted border border-border text-[10px]">Enter</kbd> 发送
				<kbd class="px-1 py-0.5 rounded bg-muted border border-border text-[10px] ml-2">Shift+Enter</kbd> 换行
			</p>
		</div>
	</main>

	<!-- Right panel: tools & status (single agent) -->
	{#if showToolsPanel}
		<aside class="w-72 shrink-0 border-l border-border bg-card/50 flex flex-col z-10">
			<div class="h-12 border-b border-border flex items-center justify-between px-4 shrink-0">
				<span class="text-sm font-semibold text-foreground flex items-center gap-2">
					<Info class="w-4 h-4 text-primary" />
					状态与工具
				</span>
				<button type="button" onclick={toggleToolsPanel} class="p-1.5 hover:bg-muted rounded-lg text-muted-foreground" aria-label="关闭">
					<ChevronRight class="w-4 h-4" />
				</button>
			</div>
			<div class="flex-1 overflow-y-auto p-4 space-y-5">
				<div class="rounded-lg border border-border bg-muted/20 p-3 space-y-2">
					<h4 class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">运行时</h4>
					<dl class="grid grid-cols-[auto_1fr] gap-x-3 gap-y-1.5 text-sm">
						<dt class="text-muted-foreground">Provider</dt>
						<dd class="font-mono truncate" title={agentStatus.provider}>{agentStatus.provider}</dd>
						<dt class="text-muted-foreground">Model</dt>
						<dd class="font-mono truncate" title={agentStatus.model}>{agentStatus.model}</dd>
						<dt class="text-muted-foreground">状态</dt>
						<dd>
							<span class="text-xs font-medium px-1.5 py-0.5 rounded {agentStatus.running ? 'bg-green-500/15 text-green-600 dark:text-green-400' : 'bg-muted text-muted-foreground'}">
								{agentStatus.running ? '运行中' : '就绪'}
							</span>
						</dd>
					</dl>
				</div>
				<div>
					<h4 class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-2 flex items-center gap-1.5">
						<Wrench class="w-3.5 h-3.5" />
						工具 ({agentTools.length})
					</h4>
					<ul class="space-y-2">
						{#each agentTools as tool (tool.name)}
							<li class="rounded-lg border border-border/60 bg-background/50 p-2.5">
								<div class="flex items-center justify-between gap-2 mb-0.5">
									<span class="text-xs font-mono font-medium text-foreground truncate">{tool.name}</span>
									<span class="text-[10px] text-muted-foreground shrink-0">{tool.category}</span>
								</div>
								<p class="text-xs text-muted-foreground leading-snug line-clamp-2">{tool.description}</p>
							</li>
						{/each}
					</ul>
				</div>
			</div>
		</aside>
	{/if}
</div>

<!-- Render System Logs Modal -->
<SystemLogs bind:isOpen={isLogsOpen} />
<SettingsModal bind:isOpen={isSettingsOpen} />
