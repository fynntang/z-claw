<script lang="ts">
	import { X, Server, Shield, Database, Cpu, Globe, KeyRound, Settings, Save, AlertCircle } from '@lucide/svelte';
	import { invoke } from '@tauri-apps/api/core';
	
	let { isOpen = $bindable(false) } = $props();

	function closeSettings() {
		isOpen = false;
	}

	let activeTab = $state('general');
	
	// Configuration State
	let config = $state({
		api_key: '',
		api_url: 'https://api.openai.com/v1',
		provider: 'openai',
		model: 'gpt-4o-mini',
		temperature: 0.7,
		local_model_path: null as string | null
	});
	
	let isLoading = $state(true);
	let saveStatus = $state<'idle' | 'saving' | 'success' | 'error'>('idle');
	let errorMessage = $state('');

	async function loadConfig() {
		try {
			isLoading = true;
			const data = await invoke<typeof config>('config_get');
			config = data;
		} catch (e) {
			console.error("Failed to load config:", e);
		} finally {
			isLoading = false;
		}
	}

	async function saveConfig() {
		try {
			saveStatus = 'saving';
			errorMessage = '';
			await invoke('config_set', { config });
			saveStatus = 'success';
			setTimeout(() => { if (saveStatus === 'success') saveStatus = 'idle'; }, 2000);
		} catch (e) {
			console.error("Failed to save config:", e);
			saveStatus = 'error';
			errorMessage = String(e);
		}
	}
	
	// Run loadConfig when modal opens
	$effect(() => {
		if (isOpen) {
			loadConfig();
			saveStatus = 'idle';
		}
	});

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
					{#if isLoading}
						<div class="flex h-full items-center justify-center flex-col gap-3 text-muted-foreground/50 opacity-50 animate-pulse">
							<Settings class="w-12 h-12 animate-spin-slow" />
							<p class="text-xs tracking-widest uppercase">Loading Configuration...</p>
						</div>
					{:else if activeTab === 'general'}
						<div class="space-y-8 animate-in fade-in duration-300">
							<div class="space-y-4">
								<h3 class="text-sm font-bold tracking-wider text-foreground pb-2 border-b border-border/50">Core Parameters</h3>
								
								<div class="space-y-4">
									<div class="grid grid-cols-2 gap-4">
										<div class="flex flex-col gap-1.5">
											<label for="provider" class="text-[11px] uppercase tracking-widest text-muted-foreground font-bold">Provider</label>
											<select id="provider" bind:value={config.provider} class="bg-muted/20 border border-border rounded-sm px-3 py-2 text-xs text-foreground/80 focus:outline-none focus:border-primary/50 transition-colors">
												<option value="openai">OpenAI</option>
												<option value="anthropic">Anthropic</option>
												<option value="openrouter">OpenRouter</option>
												<option value="ollama">Ollama (Local)</option>
											</select>
										</div>
										<div class="flex flex-col gap-1.5">
											<label for="model-name" class="text-[11px] uppercase tracking-widest text-muted-foreground font-bold">Model</label>
											<input id="model-name" type="text" bind:value={config.model} placeholder="e.g. gpt-4o-mini" class="bg-muted/20 border border-border rounded-sm px-3 py-2 text-xs text-foreground/80 focus:outline-none focus:border-primary/50 transition-colors" />
										</div>
									</div>

									<div class="flex flex-col gap-1.5">
										<label for="api-url" class="text-[11px] uppercase tracking-widest text-muted-foreground font-bold">API Base URL</label>
										<input id="api-url" type="text" bind:value={config.api_url} placeholder="https://api.openai.com/v1" class="w-full bg-muted/20 border border-border rounded-sm px-3 py-2 text-xs text-foreground/80 focus:outline-none focus:border-primary/50 transition-colors" />
									</div>

									<div class="flex flex-col gap-1.5">
										<label for="api-key" class="text-[11px] uppercase tracking-widest text-muted-foreground font-bold">API Key</label>
										<input id="api-key" type="password" bind:value={config.api_key} placeholder="sk-..." class="w-full bg-muted/20 border border-border rounded-sm px-3 py-2 text-xs text-foreground/80 focus:outline-none focus:border-primary/50 transition-colors" />
									</div>

									<div class="flex flex-col gap-1.5 pt-2">
										<div class="flex items-center justify-between">
											<label for="temperature" class="text-[11px] uppercase tracking-widest text-muted-foreground font-bold">Temperature: {config.temperature.toFixed(1)}</label>
										</div>
										<input id="temperature" type="range" min="0.0" max="2.0" step="0.1" bind:value={config.temperature} class="w-full h-1 bg-muted rounded-full appearance-none cursor-pointer accent-primary" />
									</div>
								</div>
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
			<div class="h-14 border-t border-border/50 flex items-center justify-between px-6 shrink-0 bg-muted/5 gap-3">
				<!-- Status Message -->
				<div class="flex items-center text-xs">
					{#if saveStatus === 'success'}
						<span class="text-green-500 font-bold tracking-wider flex items-center gap-2 animate-in fade-in"><Shield class="w-3.5 h-3.5" /> SECURE SAVED</span>
					{:else if saveStatus === 'error'}
						<span class="text-red-500 font-bold tracking-wider flex items-center gap-2 animate-in fade-in" title={errorMessage}><AlertCircle class="w-3.5 h-3.5" /> FAILED (HOVER)</span>
					{/if}
				</div>

				<div class="flex items-center gap-3">
					<button onclick={closeSettings} class="px-4 py-2 text-xs font-bold tracking-widest uppercase text-muted-foreground hover:text-foreground transition-colors">Cancel</button>
					<button onclick={saveConfig} disabled={saveStatus === 'saving'} class="px-5 py-2 bg-primary/20 hover:bg-primary/30 border border-primary/50 text-primary text-xs font-bold tracking-widest uppercase rounded-sm shadow-[0_0_10px_rgba(var(--color-primary),0.2)] transition-all flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed">
						{#if saveStatus === 'saving'}
							<Settings class="w-3.5 h-3.5 animate-spin" />
							Saving...
						{:else}
							<Save class="w-3.5 h-3.5" />
							Apply Changes
						{/if}
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}
