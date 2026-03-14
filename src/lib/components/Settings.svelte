<script lang="ts">
	import { X, Server, Shield, Database, Cpu, Globe, KeyRound, Settings } from '@lucide/svelte';
	
	let { isOpen = $bindable(false) } = $props();

	function closeSettings() {
		isOpen = false;
	}

	let activeTab = $state('general');
</script>

{#if isOpen}
	<div class="fixed inset-0 z-200 flex items-center justify-center bg-background/80 dark:bg-black/60 backdrop-blur-sm animate-in fade-in duration-200 font-mono">
		<!-- Modal Container -->
		<div class="w-full max-w-3xl h-[80vh] max-h-[700px] bg-background/95 border border-primary/30 rounded-[4px] shadow-[0_0_30px_rgba(var(--color-primary),0.15)] flex flex-col relative overflow-hidden animate-in zoom-in-95 duration-300 slide-in-from-bottom-4">
			
			<!-- Decorative Cyber Elements -->
			<div class="absolute top-0 left-0 w-full h-1 bg-linear-to-r from-transparent via-primary/50 to-transparent"></div>
			<div class="absolute top-0 left-0 w-2 h-2 border-t-2 border-l-2 border-primary"></div>
			<div class="absolute top-0 right-0 w-2 h-2 border-t-2 border-r-2 border-primary"></div>
			<div class="absolute bottom-0 left-0 w-2 h-2 border-b-2 border-l-2 border-primary"></div>
			<div class="absolute bottom-0 right-0 w-2 h-2 border-b-2 border-r-2 border-primary"></div>

			<!-- Header -->
			<div class="h-14 border-b border-border/50 flex items-center justify-between px-6 shrink-0 bg-muted/10">
				<div class="flex items-center gap-3">
					<Settings class="w-5 h-5 text-primary" />
					<h2 class="text-sm font-bold tracking-[0.2em] uppercase text-foreground">System Configuration</h2>
				</div>
				<button onclick={closeSettings} class="p-1.5 hover:bg-red-500/20 text-muted-foreground hover:text-red-400 rounded-sm transition-colors group">
					<X class="w-4 h-4 group-hover:rotate-90 transition-transform duration-300" />
				</button>
			</div>

			<!-- Body: Sidebar + Content -->
			<div class="flex-1 flex overflow-hidden">
				<!-- Settings Nav -->
				<div class="w-48 border-r border-border/50 bg-muted/5 flex flex-col py-4 px-2 space-y-1">
					<div class="text-[10px] uppercase text-muted-foreground/50 font-bold tracking-[0.2em] px-3 mb-2">Categories</div>
					
					<button 
						onclick={() => activeTab = 'general'}
						class="flex items-center gap-3 px-3 py-2.5 text-xs rounded-sm transition-all focus:outline-none {activeTab === 'general' ? 'bg-primary/10 text-primary border-l-2 border-primary' : 'text-muted-foreground hover:bg-muted/30 hover:text-foreground border-l-2 border-transparent'}"
					>
						<Server class="w-4 h-4" />
						<span>General</span>
					</button>
					<button 
						onclick={() => activeTab = 'models'}
						class="flex items-center gap-3 px-3 py-2.5 text-xs rounded-sm transition-all focus:outline-none {activeTab === 'models' ? 'bg-primary/10 text-primary border-l-2 border-primary' : 'text-muted-foreground hover:bg-muted/30 hover:text-foreground border-l-2 border-transparent'}"
					>
						<Cpu class="w-4 h-4" />
						<span>LLM Models</span>
					</button>
					<button 
						onclick={() => activeTab = 'network'}
						class="flex items-center gap-3 px-3 py-2.5 text-xs rounded-sm transition-all focus:outline-none {activeTab === 'network' ? 'bg-primary/10 text-primary border-l-2 border-primary' : 'text-muted-foreground hover:bg-muted/30 hover:text-foreground border-l-2 border-transparent'}"
					>
						<Globe class="w-4 h-4" />
						<span>Network Proxy</span>
					</button>
					<button 
						onclick={() => activeTab = 'storage'}
						class="flex items-center gap-3 px-3 py-2.5 text-xs rounded-sm transition-all focus:outline-none {activeTab === 'storage' ? 'bg-primary/10 text-primary border-l-2 border-primary' : 'text-muted-foreground hover:bg-muted/30 hover:text-foreground border-l-2 border-transparent'}"
					>
						<Database class="w-4 h-4" />
						<span>Storage</span>
					</button>
					<button 
						onclick={() => activeTab = 'security'}
						class="flex items-center gap-3 px-3 py-2.5 text-xs rounded-sm transition-all focus:outline-none {activeTab === 'security' ? 'bg-primary/10 text-primary border-l-2 border-primary' : 'text-muted-foreground hover:bg-muted/30 hover:text-foreground border-l-2 border-transparent'}"
					>
						<Shield class="w-4 h-4" />
						<span>Security Limits</span>
					</button>
					<button 
						onclick={() => activeTab = 'advanced'}
						class="flex items-center gap-3 px-3 py-2.5 text-xs rounded-sm transition-all focus:outline-none {activeTab === 'advanced' ? 'bg-primary/10 text-primary border-l-2 border-primary' : 'text-muted-foreground hover:bg-muted/30 hover:text-foreground border-l-2 border-transparent'}"
					>
						<KeyRound class="w-4 h-4" />
						<span>Advanced</span>
					</button>
				</div>

				<!-- Settings Edit Area -->
				<div class="flex-1 overflow-y-auto p-6 scrollbar-thin">
					{#if activeTab === 'general'}
						<div class="space-y-8 animate-in fade-in duration-300">
							<div class="space-y-4">
								<h3 class="text-sm font-bold tracking-wider text-foreground pb-2 border-b border-border/50">Core Parameters</h3>
								
								<div class="space-y-4">
									<div class="flex flex-col gap-1.5">
										<label for="workspace-dir" class="text-[11px] uppercase tracking-widest text-muted-foreground font-bold">Workspace Directory</label>
										<div class="flex gap-2">
											<input id="workspace-dir" type="text" value="C:\Users\fzpyi\.openclaw-autoclaw\workspace\z-claw" disabled class="flex-1 bg-muted/20 border border-border rounded-sm px-3 py-2 text-xs text-foreground/80 focus:outline-none focus:border-primary/50 transition-colors" />
											<button class="px-4 py-2 bg-muted/40 hover:bg-muted border border-border rounded-sm text-xs font-medium transition-colors">Browse...</button>
										</div>
									</div>

									<div class="flex flex-col gap-1.5">
										<div class="text-[11px] uppercase tracking-widest text-muted-foreground font-bold" id="auto-start-label">Auto-Start</div>
										<div class="flex items-center gap-3">
											<button aria-labelledby="auto-start-label" class="w-10 h-5 rounded-full bg-primary/20 relative transition-colors cursor-pointer">
												<span class="absolute left-5 top-0.5 w-4 h-4 bg-primary rounded-full shadow-sm"></span>
											</button>
											<span class="text-xs text-muted-foreground">Launch ZClaw daemon on system boot</span>
										</div>
									</div>
									
									<div class="flex flex-col gap-1.5">
										<div class="text-[11px] uppercase tracking-widest text-muted-foreground font-bold" id="telemetry-label">Telemetry</div>
										<div class="flex items-center gap-3">
											<button aria-labelledby="telemetry-label" class="w-10 h-5 rounded-full bg-muted border border-border relative transition-colors cursor-pointer">
												<span class="absolute left-0.5 top-0.5 w-4 h-4 bg-muted-foreground/40 rounded-full shadow-sm"></span>
											</button>
											<span class="text-xs text-muted-foreground">Send anonymous usage stats (Requires restart)</span>
										</div>
									</div>
								</div>
							</div>

							<div class="space-y-4">
								<h3 class="text-sm font-bold tracking-wider text-red-500 pb-2 border-b border-border/50">Danger Zone</h3>
								<button class="px-4 py-2 bg-red-500/10 hover:bg-red-500/20 border border-red-500/30 rounded-sm text-xs text-red-500 font-bold transition-colors">Reset All Configurations</button>
							</div>
						</div>
					{:else}
						<div class="flex h-full items-center justify-center flex-col gap-3 text-muted-foreground/50 opacity-50 animate-in fade-in duration-300">
							<Cpu class="w-12 h-12" />
							<p class="text-xs tracking-widest uppercase text-center max-w-xs">{activeTab} Modulator<br/>Status: Offline / Under Construction</p>
						</div>
					{/if}
				</div>
			</div>

			<!-- Footer Actions -->
			<div class="h-14 border-t border-border/50 flex items-center justify-end px-6 shrink-0 bg-muted/5 gap-3">
				<button onclick={closeSettings} class="px-4 py-2 text-xs font-bold tracking-widest uppercase text-muted-foreground hover:text-foreground transition-colors">Cancel</button>
				<button class="px-5 py-2 bg-primary/20 hover:bg-primary/30 border border-primary/50 text-primary text-xs font-bold tracking-widest uppercase rounded-sm shadow-[0_0_10px_rgba(var(--color-primary),0.2)] transition-all flex items-center gap-2">
					<Settings class="w-3.5 h-3.5" />
					Apply Changes
				</button>
			</div>
		</div>
	</div>
{/if}
