<script lang="ts">
	import { Send, Menu, Plus, Settings, Sparkles, MessageSquareDot, Hash, Clock, Cpu, Bot, Activity, FolderGit2, Info, ChevronRight, FileCode2, TerminalSquare } from '@lucide/svelte';
	
	// Right sidebar state management
	let rightPanelActive = $state<'none' | 'files' | 'agent'>('none');

	function toggleRightPanel(panel: 'none' | 'files' | 'agent') {
		if (panel === 'none') {
			rightPanelActive = 'none';
		} else {
			rightPanelActive = rightPanelActive === panel ? 'none' : panel;
		}
	}

	type Message = { role: string; content: string; timestamp: string };
	let messages = $state<Message[]>([
		{ role: 'assistant', content: 'SYSTEM ONLINE. AWAITING INPUT.', timestamp: '00:00' }
	]);
	
	let input = $state('');
	
	function sendMessage() {
		if (!input.trim()) return;
		
		const timeString = new Date().toLocaleTimeString('en-US', { hour12: false, hour: '2-digit', minute: '2-digit' });
		messages = [...messages, { role: 'user', content: input, timestamp: timeString }];
		
		const userMsg = input;
		input = '';
		
		setTimeout(() => {
			messages = [...messages, { 
				role: 'assistant', 
				content: `ACKNOWLEDGED: Processing command "${userMsg}"...`, 
				timestamp: new Date().toLocaleTimeString('en-US', { hour12: false, hour: '2-digit', minute: '2-digit' }) 
			}];
		}, 800);
	}
	
	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			sendMessage();
		}
	}
</script>

<div class="flex h-full w-full bg-background text-foreground font-mono antialiased overflow-hidden selection:bg-primary selection:text-primary-foreground">
	<!-- Sidebar -->
	<aside class="w-64 border-r border-border bg-background/50 backdrop-blur-md hidden md:flex flex-col transition-all duration-300 z-20">
		<!-- Main Navigation Area -->
		<div class="flex-1 overflow-y-auto pt-6 px-3 space-y-6 scrollbar-none hover:scrollbar-thin">
			<!-- Agents Section -->
			<div class="space-y-1">
				<div class="flex items-center gap-2 px-3 mb-2 text-[10px] uppercase text-muted-foreground/60 font-bold tracking-[0.2em]">
					<Cpu class="w-3 h-3 text-primary/70" />
					<span>Agents 智能体</span>
				</div>
				<button class="w-full flex items-center gap-3 px-3 py-2.5 text-sm bg-primary/10 rounded-sm border-l-2 border-primary text-left transition-all shadow-[inset_0_0_10px_rgba(var(--color-primary),0.1)] group">
					<Bot class="w-4 h-4 text-primary group-hover:animate-pulse shrink-0" />
					<span class="truncate text-foreground font-medium">Core Intelligence</span>
					<span class="ml-auto flex h-2 w-2 rounded-full border border-primary bg-primary/20"></span>
				</button>
				<button class="w-full flex items-center gap-3 px-3 py-2.5 text-sm hover:bg-muted/50 rounded-sm border-l-2 border-transparent text-left text-muted-foreground hover:text-foreground transition-all group">
					<Sparkles class="w-4 h-4 shrink-0 group-hover:text-amber-400 transition-colors" />
					<span class="truncate">Coding Assistant</span>
				</button>
				<button class="w-full flex items-center gap-3 px-3 py-2.5 text-sm hover:bg-muted/50 rounded-sm border-l-2 text-left hover:text-primary transition-all text-primary/70 border border-dashed border-primary/20 mt-2">
					<Plus class="w-4 h-4 shrink-0" />
					<span class="truncate font-medium text-xs tracking-wider uppercase">Deploy New Agent</span>
				</button>
			</div>

			<!-- Sessions Section -->
			<div class="space-y-1">
				<div class="flex items-center gap-2 px-3 mb-2 text-[10px] uppercase text-muted-foreground/60 font-bold tracking-[0.2em]">
					<MessageSquareDot class="w-3 h-3 text-blue-400/70" />
					<span>Sessions 会话</span>
				</div>
				<button class="w-full flex items-center justify-between px-3 py-2.5 text-sm hover:bg-muted/50 rounded-sm border-l-2 border-transparent text-left text-muted-foreground hover:text-foreground transition-all group">
					<div class="flex items-center gap-3 truncate">
						<Activity class="w-4 h-4 shrink-0 text-blue-400/50 group-hover:text-blue-400 transition-colors" />
						<span class="truncate">SysOps Debugging</span>
					</div>
					<span class="text-[9px] font-mono text-muted-foreground/40">10m ago</span>
				</button>
				<button class="w-full flex items-center justify-between px-3 py-2.5 text-sm hover:bg-muted/50 rounded-sm border-l-2 border-transparent text-left text-muted-foreground hover:text-foreground transition-all group">
					<div class="flex items-center gap-3 truncate">
						<Activity class="w-4 h-4 shrink-0 text-blue-400/50 group-hover:text-blue-400 transition-colors" />
						<span class="truncate">Frontend Architecture</span>
					</div>
					<span class="text-[9px] font-mono text-muted-foreground/40">1h ago</span>
				</button>
			</div>

			<!-- IM Channels Section -->
			<div class="space-y-1">
				<div class="flex items-center gap-2 px-3 mb-2 text-[10px] uppercase text-muted-foreground/60 font-bold tracking-[0.2em] relative">
					<Hash class="w-3 h-3 text-purple-400/70" />
					<span>IM Channels 频道</span>
					<div class="absolute right-3 top-1/2 -translate-y-1/2 h-px w-12 bg-linear-to-r from-purple-500/30 to-transparent"></div>
				</div>
				<button class="w-full flex items-center justify-between px-3 py-2 text-sm hover:bg-muted/50 rounded-sm border-l-2 border-transparent text-left text-muted-foreground hover:text-foreground transition-all group">
					<span class="truncate pl-7 text-[13px]"># alerts-critical</span>
					<span class="px-1.5 py-0.5 rounded-sm bg-red-500/20 border border-red-500/30 text-[9px] text-red-400 font-bold">2</span>
				</button>
				<button class="w-full flex items-center justify-between px-3 py-2 text-sm hover:bg-muted/50 rounded-sm border-l-2 border-transparent text-left text-muted-foreground hover:text-foreground transition-all group">
					<span class="truncate pl-7 text-[13px]"># bot-general</span>
				</button>
				<button class="w-full flex items-center justify-between px-3 py-2 text-sm hover:bg-muted/50 rounded-sm border-l-2 border-transparent text-left text-muted-foreground hover:text-foreground transition-all group opacity-60">
					<span class="truncate pl-7 text-[13px]"># dev-logs</span>
				</button>
			</div>

			<!-- Cron/Scheduled Tasks Section -->
			<div class="space-y-1 pb-4">
				<div class="flex items-center justify-between px-3 mb-2 text-[10px] uppercase text-muted-foreground/60 font-bold tracking-[0.2em]">
					<div class="flex items-center gap-2">
						<Clock class="w-3 h-3 text-emerald-400/70" />
						<span>Cron 定时任务</span>
					</div>
				</div>
				<button class="w-full flex items-center justify-between px-3 py-2.5 text-sm hover:bg-muted/50 rounded-sm border-l-2 border-transparent text-left text-muted-foreground hover:text-foreground transition-all group">
					<div class="flex flex-col">
						<span class="truncate font-medium text-[13px]">Data Backup</span>
						<span class="text-[10px] font-mono text-muted-foreground/60">0 2 * * * (In 16h)</span>
					</div>
					<div class="w-1.5 h-1.5 rounded-full bg-emerald-500/50"></div>
				</button>
				<button class="w-full flex items-center justify-between px-3 py-2.5 text-sm hover:bg-muted/50 rounded-sm border-l-2 border-transparent text-left text-muted-foreground hover:text-foreground transition-all group">
					<div class="flex flex-col">
						<span class="truncate font-medium text-[13px]">Health Check</span>
						<span class="text-[10px] font-mono text-muted-foreground/60">*/5 * * * *</span>
					</div>
					<div class="w-1.5 h-1.5 rounded-full bg-emerald-500/50"></div>
				</button>
			</div>
		</div>
		
		<!-- Footer -->
		<div class="p-3 border-t border-border bg-background/30 backdrop-blur-md shrink-0">
			<button class="w-full flex items-center gap-3 px-3 py-2.5 text-sm text-muted-foreground hover:text-foreground hover:bg-muted/50 rounded-sm border border-transparent hover:border-border transition-all font-medium group">
				<Settings class="w-4 h-4 group-hover:rotate-90 transition-transform duration-500" />
				<span class="uppercase tracking-widest text-xs">System Config</span>
			</button>
		</div>
	</aside>

	<!-- Main Chat Area -->
	<main class="flex-1 flex flex-col relative h-full bg-[radial-gradient(ellipse_at_top_right,var(--color-muted)_0%,transparent_40%)] transition-all duration-300">
		<!-- Topbar / Chat Header -->
		<header class="h-16 border-b border-border/60 flex items-center justify-between px-4 md:px-6 bg-background/80 backdrop-blur-sm absolute top-0 w-full z-10 shrink-0 shadow-sm">
			<!-- Left: Agent Info & Session Details -->
			<div class="flex items-center gap-4">
				<button class="p-2 hover:bg-muted rounded-md md:hidden transition-colors text-muted-foreground mr-2">
					<Menu class="w-4 h-4" />
				</button>
				
				<!-- Agent Identity Badge -->
				<div class="w-10 h-10 rounded-sm bg-primary/10 border border-primary/30 flex items-center justify-center shadow-[inset_0_0_8px_rgba(var(--color-primary),0.2)]">
					<Bot class="w-5 h-5 text-primary" />
				</div>
				
				<div class="flex flex-col">
					<div class="flex items-center gap-2 mb-0.5">
						<span class="text-sm font-bold text-foreground tracking-wide">Core Intelligence</span>
						<span class="px-1.5 py-0.5 rounded-[2px] bg-primary/20 text-[9px] font-bold uppercase tracking-widest text-primary border border-primary/20">v0.1.9</span>
					</div>
					<div class="flex items-center gap-3 text-[11px] font-mono text-muted-foreground">
						<div class="flex items-center gap-1.5">
							<MessageSquareDot class="w-3 h-3 text-blue-400" />
							<span>SysOps Debugging</span>
						</div>
						<div class="w-px h-3 bg-border"></div>
						<div class="flex items-center gap-1.5">
							<Hash class="w-3 h-3 text-purple-400" />
							<span>CID: 8f92a1</span>
						</div>
					</div>
				</div>
			</div>
			
			<!-- Right: Panel Toggles -->
			<div class="flex items-center gap-2">
				<button 
					onclick={() => toggleRightPanel('files')}
					class="flex items-center gap-2 px-3 py-1.5 rounded-sm border transition-all text-xs font-medium uppercase tracking-wider
						{rightPanelActive === 'files' 
							? 'bg-primary/20 border-primary text-primary shadow-[0_0_10px_rgba(var(--color-primary),0.2)]' 
							: 'bg-muted/30 border-border text-muted-foreground hover:bg-muted hover:text-foreground'}"
				>
					<FolderGit2 class="w-3.5 h-3.5" />
					<span class="hidden sm:inline">Workspace</span>
				</button>
				<button 
					onclick={() => toggleRightPanel('agent')}
					class="flex items-center gap-2 px-3 py-1.5 rounded-sm border transition-all text-xs font-medium uppercase tracking-wider
						{rightPanelActive === 'agent' 
							? 'bg-amber-500/20 border-amber-500 text-amber-500 shadow-[0_0_10px_rgba(245,158,11,0.2)]' 
							: 'bg-muted/30 border-border text-muted-foreground hover:bg-muted hover:text-foreground'}"
				>
					<Info class="w-3.5 h-3.5" />
					<span class="hidden sm:inline">Agent Specs</span>
				</button>
			</div>
		</header>
		
		<!-- Message List -->
		<div class="flex-1 overflow-y-auto pt-20 pb-32 px-4 md:px-8 w-full max-w-4xl mx-auto scroll-smooth">
			<div class="flex flex-col space-y-8">
				{#each messages as msg (msg.timestamp)}
					<div class="flex flex-col {msg.role === 'user' ? 'items-end' : 'items-start'} animate-in fade-in slide-in-from-bottom-4 duration-500">
						<div class="flex items-center gap-2 mb-2 opacity-60 {msg.role === 'user' ? 'flex-row-reverse' : 'flex-row'}">
							<span class="text-[10px] font-bold uppercase tracking-[0.2em] {msg.role === 'user' ? 'text-foreground' : 'text-primary'}">
								{msg.role === 'user' ? 'GUEST' : 'ZCLAW'}
							</span>
							<span class="w-1 h-1 rounded-full bg-border"></span>
							<span class="text-[10px] tracking-wider">{msg.timestamp}</span>
						</div>
						
						<div class="relative group max-w-[90%] md:max-w-[85%]">
							<!-- Subtle glow effect behind AI messages -->
							{#if msg.role === 'assistant'}
								<div class="absolute -inset-1 bg-primary/5 blur-xl rounded-sm opacity-0 group-hover:opacity-100 transition-opacity duration-700"></div>
							{/if}
							
							<div class="relative px-5 py-4 text-[13px] md:text-sm leading-relaxed
								{msg.role === 'user' 
									? 'bg-foreground text-background shadow-md' 
									: 'bg-card/50 backdrop-blur-sm text-card-foreground border border-border shadow-sm'}">
								<p class="whitespace-pre-wrap">{msg.content}</p>
								
								<!-- Decorative corners for high-tech feel -->
								{#if msg.role === 'assistant'}
									<div class="absolute top-0 left-0 w-2 h-2 border-t border-l border-primary/50 -mt-px -ml-px"></div>
									<div class="absolute bottom-0 right-0 w-2 h-2 border-b border-r border-primary/50 -mb-px -mr-px"></div>
								{/if}
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
		
		<!-- Input Area -->
		<div class="absolute bottom-0 w-full bg-linear-to-t from-background via-background/95 to-transparent pt-12 pb-6 px-4">
			<div class="max-w-3xl mx-auto relative group">
				<!-- Input focus glow -->
				<div class="absolute -inset-0.5 bg-linear-to-r from-primary/30 via-primary/10 to-transparent blur-md opacity-0 group-focus-within:opacity-100 transition duration-700 pointer-events-none"></div>
				
				<form onsubmit={(e) => { e.preventDefault(); sendMessage(); }} class="relative flex items-end gap-2 bg-background/80 backdrop-blur-xl border border-border rounded-sm p-1.5 shadow-2xl transition-all focus-within:border-primary/50">
					<!-- Command prefix indicator for high-tech aesthetic -->
					<div class="h-10 flex items-center justify-center pl-3 pr-1 text-primary/70 shrink-0 pointer-events-none absolute left-0 bottom-1.5">
						<span class="font-bold">❯</span>
					</div>
					
					<textarea 
						bind:value={input}
						onkeydown={handleKeydown}
						placeholder="TYPE COMMAND OR QUERY..." 
						class="w-full max-h-40 min-h-11 bg-transparent resize-none border-0 focus:ring-0 py-3 pr-3 pl-8 text-[13px] md:text-sm placeholder:text-muted-foreground/40 placeholder:tracking-widest placeholder:uppercase outline-none scrollbar-thin overflow-y-auto"
						rows="1"
					></textarea>
					
					<div class="flex items-center justify-center p-1 shrink-0 h-11 w-11 z-10">
						<button 
							type="submit" 
							disabled={!input.trim()}
							class="w-9 h-9 flex items-center justify-center rounded-sm bg-foreground text-background hover:bg-primary hover:text-primary-foreground disabled:opacity-30 disabled:hover:bg-foreground disabled:hover:text-background transition-all"
						>
							<Send class="w-4 h-4" />
						</button>
					</div>
					
					<!-- Decorative tech border -->
					<div class="absolute bottom-0 right-0 w-4 h-px bg-primary pointer-events-none"></div>
					<div class="absolute bottom-0 right-0 w-px h-4 bg-primary pointer-events-none"></div>
				</form>
				<div class="text-center mt-3 text-[10px] text-muted-foreground/60 uppercase tracking-[0.2em] flex items-center justify-center gap-4">
					<span><kbd class="font-sans px-1 py-0.5 rounded-sm bg-muted/50 border border-border/50 text-[9px]">Shift</kbd>+<kbd class="font-sans px-1 py-0.5 rounded-sm bg-muted/50 border border-border/50 text-[9px]">Enter</kbd> newline</span>
					<span><kbd class="font-sans px-1 py-0.5 rounded-sm bg-muted/50 border border-border/50 text-[9px]">Enter</kbd> send</span>
				</div>
			</div>
		</div>
	</main>

	<!-- Right Sidebar Panel (Context View) -->
	{#if rightPanelActive !== 'none'}
		<aside class="w-80 border-l border-border bg-background/80 backdrop-blur-xl hidden lg:flex flex-col transition-all duration-300 z-20 animate-in slide-in-from-right-8 fade-in">
			<!-- Panel Header -->
			<div class="h-16 border-b border-border/50 flex items-center justify-between px-4 shrink-0 bg-muted/20">
				<div class="flex items-center gap-2 font-bold tracking-widest text-xs uppercase">
					{#if rightPanelActive === 'files'}
						<FolderGit2 class="w-4 h-4 text-primary" />
						<span class="text-primary">Project Files</span>
					{:else}
						<Info class="w-4 h-4 text-amber-500" />
						<span class="text-amber-500">Agent Details</span>
					{/if}
				</div>
				<button onclick={() => toggleRightPanel('none')} class="p-1.5 hover:bg-muted rounded-md transition-colors text-muted-foreground hover:text-foreground">
					<ChevronRight class="w-4 h-4" />
				</button>
			</div>

			<!-- Panel Content -->
			<div class="flex-1 overflow-y-auto p-4 scrollbar-none hover:scrollbar-thin">
				{#if rightPanelActive === 'files'}
					<div class="space-y-4">
						<div class="text-[10px] uppercase text-muted-foreground/60 font-bold tracking-[0.2em]">C:\workspace\z-claw</div>
						<div class="space-y-1 font-mono text-xs">
							<div class="flex items-center gap-2 px-2 py-1.5 rounded-sm hover:bg-muted text-foreground cursor-pointer group">
								<ChevronRight class="w-3 h-3 text-muted-foreground transition-transform group-hover:rotate-90" />
								<FolderGit2 class="w-3.5 h-3.5 text-blue-400" />
								<span>src</span>
							</div>
							<!-- Expanded folder mock -->
							<div class="pl-5 space-y-1">
								<div class="flex items-center gap-2 px-2 py-1.5 rounded-sm hover:bg-muted text-foreground cursor-pointer group">
									<ChevronRight class="w-3 h-3 text-muted-foreground transition-transform group-hover:rotate-90" />
									<FolderGit2 class="w-3.5 h-3.5 text-blue-400" />
									<span>routes</span>
								</div>
								<div class="pl-5 space-y-1 relative before:absolute before:left-2 before:top-0 before:h-full before:w-px before:bg-border">
									<div class="flex items-center gap-2 px-2 py-1.5 rounded-sm bg-primary/10 text-primary border border-primary/20 cursor-pointer relative before:absolute before:-left-3 before:top-1/2 before:-translate-y-1/2 before:w-3 before:h-px before:bg-border">
										<FileCode2 class="w-3.5 h-3.5 text-primary" />
										<span>+page.svelte</span>
									</div>
									<div class="flex items-center gap-2 px-2 py-1.5 rounded-sm hover:bg-muted text-muted-foreground hover:text-foreground cursor-pointer relative before:absolute before:-left-3 before:top-1/2 before:-translate-y-1/2 before:w-3 before:h-px before:bg-border">
										<FileCode2 class="w-3.5 h-3.5 text-orange-400" />
										<span>layout.css</span>
									</div>
								</div>
							</div>
							<div class="flex items-center gap-2 px-2 py-1.5 rounded-sm hover:bg-muted text-foreground cursor-pointer group">
								<ChevronRight class="w-3 h-3 text-muted-foreground" />
								<FolderGit2 class="w-3.5 h-3.5 text-blue-400" />
								<span>src-tauri</span>
							</div>
							<div class="flex items-center gap-2 px-2 py-1.5 rounded-sm hover:bg-muted text-foreground cursor-pointer group">
								<ChevronRight class="w-3 h-3 text-muted-foreground" />
								<FolderGit2 class="w-3.5 h-3.5 text-blue-400" />
								<span>crates</span>
							</div>
						</div>
					</div>
				{:else}
					<div class="space-y-6">
						<div class="flex flex-col items-center pb-4 border-b border-border/50 relative">
							<div class="absolute -top-10 -z-10 w-32 h-32 bg-amber-500/10 rounded-full blur-3xl"></div>
							<div class="w-16 h-16 rounded-md bg-amber-500/10 border border-amber-500/30 flex items-center justify-center mb-3 shadow-[0_0_15px_rgba(245,158,11,0.15)]">
								<Bot class="w-8 h-8 text-amber-500" />
							</div>
							<h3 class="text-sm font-bold tracking-wider text-foreground">Core Intelligence</h3>
							<p class="text-xs text-muted-foreground mt-1 text-center leading-relaxed">System administrator agent with full file-system and terminal execution capabilities.</p>
						</div>

						<div class="space-y-3">
							<div class="text-[10px] uppercase text-muted-foreground/80 font-bold tracking-[0.2em] mb-2 flex items-center gap-2">
								<TerminalSquare class="w-3 h-3" />
								<span>Allocated Resources</span>
							</div>
							
							<div class="bg-muted/30 border border-border rounded-sm p-3 grid grid-cols-2 gap-4">
								<div class="flex flex-col">
									<span class="text-[9px] text-muted-foreground uppercase opacity-70 mb-1">Model Engine</span>
									<span class="text-xs font-mono text-foreground font-medium truncate">claude-3-7-sonnet</span>
								</div>
								<div class="flex flex-col">
									<span class="text-[9px] text-muted-foreground uppercase opacity-70 mb-1">Context Window</span>
									<span class="text-xs font-mono text-amber-400 font-medium whitespace-nowrap">200K Tokens</span>
								</div>
								<div class="flex flex-col">
									<span class="text-[9px] text-muted-foreground uppercase opacity-70 mb-1">Mode</span>
									<span class="text-[10px] px-1.5 py-0.5 bg-green-500/20 text-green-400 rounded-sm w-max uppercase font-bold tracking-wider">Autonomous</span>
								</div>
								<div class="flex flex-col">
									<span class="text-[9px] text-muted-foreground uppercase opacity-70 mb-1">Response Time</span>
									<span class="text-xs font-mono text-foreground font-medium">~1.2s avg</span>
								</div>
							</div>
						</div>
					</div>
				{/if}
			</div>
		</aside>
	{/if}
</div>
