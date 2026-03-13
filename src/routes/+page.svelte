<script lang="ts">
	import { Send, Menu, Plus, Settings, MessageSquare, Code, Terminal, Zap } from '@lucide/svelte';
	
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

<div class="flex h-screen w-full bg-background text-foreground font-mono antialiased overflow-hidden selection:bg-primary selection:text-primary-foreground">
	<!-- Sidebar -->
	<aside class="w-64 border-r border-border bg-background/50 backdrop-blur-md hidden md:flex flex-col transition-all duration-300 z-20">
		<!-- Header -->
		<div class="h-14 border-b border-border flex items-center justify-between px-4 shrink-0">
			<div class="flex items-center gap-2 font-bold tracking-tighter text-lg uppercase">
				<Zap class="w-4 h-4 text-primary" />
				<span>ZClaw</span>
			</div>
			<button class="p-2 hover:bg-muted rounded-md transition-colors text-muted-foreground hover:text-foreground">
				<Menu class="w-4 h-4" />
			</button>
		</div>

		<!-- New Chat Action -->
		<div class="p-4">
			<button class="w-full flex items-center justify-center gap-2 px-3 py-2.5 bg-primary hover:bg-primary/90 text-primary-foreground font-bold rounded-sm border border-primary/20 transition-all text-sm tracking-wider uppercase shadow-[0_0_15px_rgba(var(--color-primary),0.3)] hover:shadow-[0_0_20px_rgba(var(--color-primary),0.5)]">
				<Plus class="w-4 h-4" />
				New Session
			</button>
		</div>
		
		<!-- History List -->
		<div class="flex-1 overflow-y-auto px-2 py-2 space-y-1 scrollbar-none hover:scrollbar-thin">
			<div class="text-[10px] uppercase text-muted-foreground/60 font-bold tracking-[0.2em] mb-3 px-3 mt-2">Active Nodes</div>
			
			<button class="w-full flex items-center gap-3 px-3 py-2.5 text-sm bg-muted rounded-sm border-l-2 border-primary text-left transition-all">
				<Terminal class="w-4 h-4 text-primary shrink-0" />
				<span class="truncate text-foreground font-medium">System Analysis</span>
			</button>
			<button class="w-full flex items-center gap-3 px-3 py-2.5 text-sm hover:bg-muted/50 rounded-sm border-l-2 border-transparent text-left text-muted-foreground hover:text-foreground transition-all">
				<Code class="w-4 h-4 shrink-0" />
				<span class="truncate">Frontend Implementation</span>
			</button>
			<button class="w-full flex items-center gap-3 px-3 py-2.5 text-sm hover:bg-muted/50 rounded-sm border-l-2 border-transparent text-left text-muted-foreground hover:text-foreground transition-all">
				<MessageSquare class="w-4 h-4 shrink-0" />
				<span class="truncate">Refactoring layout.css</span>
			</button>
		</div>
		
		<!-- Footer -->
		<div class="h-14 border-t border-border flex items-center px-4 shrink-0">
			<button class="flex items-center gap-3 text-sm text-muted-foreground hover:text-foreground transition-colors w-full font-medium">
				<Settings class="w-4 h-4" />
				<span class="uppercase tracking-widest text-xs">Config</span>
			</button>
		</div>
	</aside>

	<!-- Main Chat Area -->
	<main class="flex-1 flex flex-col relative h-full bg-[radial-gradient(ellipse_at_top_right,var(--color-muted)_0%,transparent_40%)]">
		<!-- Topbar -->
		<header class="h-14 border-b border-border/60 flex items-center justify-between px-4 bg-background/80 backdrop-blur-sm absolute top-0 w-full z-10 shrink-0">
			<div class="flex items-center gap-3">
				<button class="p-2 hover:bg-muted rounded-md md:hidden transition-colors text-muted-foreground">
					<Menu class="w-4 h-4" />
				</button>
				<div class="flex flex-col">
					<span class="text-[10px] font-bold leading-none uppercase tracking-[0.2em] text-primary mb-1">Target Model</span>
					<span class="text-sm font-semibold leading-none text-foreground/90">claude-3-7-sonnet</span>
				</div>
			</div>
			<div class="flex items-center gap-2 text-[10px] text-muted-foreground uppercase tracking-widest font-bold bg-muted/40 px-2.5 py-1.5 rounded-sm border border-border/50">
				<span class="w-2 h-2 rounded-full bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.6)] animate-pulse"></span>
				<span class="hidden sm:inline">Online</span>
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
</div>
