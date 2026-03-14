<script lang="ts">
	import { X, Server, Shield, Database, Cpu, Globe, KeyRound, Settings, Save, AlertCircle, Box, Plus, Trash2, Link2 } from '@lucide/svelte';
	import { invoke } from '@tauri-apps/api/core';
	
	let { isOpen = $bindable(false) } = $props();

	function closeSettings() {
		isOpen = false;
	}

	let activeTab = $state('providers'); // default to the new providers menu
	
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
	
	const defaultProviderUrls: Record<string, string> = {
		'openai': 'https://api.openai.com/v1',
		'anthropic': 'https://api.anthropic.com/v1',
		'openrouter': 'https://openrouter.ai/api/v1',
		'ollama': 'http://localhost:11434/v1',
		'gemini': 'https://generativelanguage.googleapis.com/v1beta/openai/',
		'doubao': 'https://ark.cn-beijing.volces.com/api/v3',
		'qwen': 'https://dashscope.aliyuncs.com/compatible-mode/v1',
		'deepseek': 'https://api.deepseek.com',
		'glm': 'https://open.bigmodel.cn/api/paas/v4',
		'xai': 'https://api.x.ai/v1',
		'together': 'https://api.together.xyz/v1',
		'groq': 'https://api.groq.com/openai/v1',
		'lmstudio': 'http://localhost:1234/v1',
		'llamacpp': 'http://localhost:8080/v1'
	};

	interface UIProviderInfo {
		name: string;
		display_name: string;
		local: boolean;
	}
	let providersList = $state<UIProviderInfo[]>([]);
	
	function toggleProviderConfig(id: string) {
		expandingProvider = expandingProvider === id ? null : id;
		if (expandingProvider && !localOverrides[id] && !id.startsWith('custom_')) {
			localOverrides[id] = { api_key: '', api_url: defaultProviderUrls[id] || '' };
		}
	}

	function addNewCustomProvider() {
		const newId = 'custom_' + Date.now();
		customProviders = [...customProviders, {
			id: newId,
			display_name: 'New Custom Provider',
			api_url: 'https://',
			api_key: '',
			api_type: 'openai-completions',
			models: [{ id: 'my-model', name: 'My Model' }]
		}];
		expandingProvider = newId;
	}

	function deleteCustomProvider(id: string) {
		customProviders = customProviders.filter(p => p.id !== id);
		if (config.provider === id) config.provider = 'openai';
		if (expandingProvider === id) expandingProvider = null;
	}

	function addModelToCustom(providerId: string) {
		const pi = customProviders.findIndex(p => p.id === providerId);
		if (pi > -1) customProviders[pi].models.push({ id: 'new-model', name: 'New Model' });
	}

	function removeModelFromCustom(providerId: string, idx: number) {
		const pi = customProviders.findIndex(p => p.id === providerId);
		if (pi > -1) customProviders[pi].models.splice(idx, 1);
	}
	
	interface CustomModelDef { id: string; name: string; }
	interface CustomProviderDef {
		id: string; // unique internal id
		display_name: string;
		api_url: string;
		api_key: string;
		api_type: 'openai-completions' | 'anthropic-completions';
		models: CustomModelDef[];
	}

	// Runtime state for editable keys/urls of built-in/local providers by name
	let localOverrides = $state<Record<string, { api_key: string; api_url: string }>>({});
	let customProviders = $state<CustomProviderDef[]>([]);

	let activeCustomProvider = $derived(customProviders.find(p => p.id === config.provider));

	let expandingProvider = $state<string | null>(null);

	async function loadConfig() {
		try {
			isLoading = true;
			const [pData, cData] = await Promise.all([
				invoke<UIProviderInfo[]>('providers_list'),
				invoke<typeof config>('config_get')
			]);
			
			providersList = pData;
			config = cData;

			// Load customized UI state from browsers LocalStorage
			const savedOverrides = localStorage.getItem('zclaw_provider_overrides');
			if (savedOverrides) {
				try { localOverrides = JSON.parse(savedOverrides); } catch(e) { console.warn("Error parsing local overrides", e); }
			}
			const savedCustoms = localStorage.getItem('zclaw_custom_providers');
			if (savedCustoms) {
				try { customProviders = JSON.parse(savedCustoms); } catch (e) { console.warn("Error parsing custom providers", e); }
			}

			// Ensure current provider slot is initialized in UI overrides
			if (!localOverrides[config.provider] && config.provider !== 'openai') { 
				// Fallback generic init, assuming it's unconfigured locally
				localOverrides[config.provider] = {
					api_key: config.api_key || '',
					api_url: config.api_url || defaultProviderUrls[config.provider] || ''
				};
			}
		} catch (e) {
			console.error("Failed to load config or providers:", e);
		} finally {
			isLoading = false;
		}
	}

	function saveLocalData() {
		localStorage.setItem('zclaw_provider_overrides', JSON.stringify(localOverrides));
		localStorage.setItem('zclaw_custom_providers', JSON.stringify(customProviders));
	}

	async function saveConfig() {
		try {
			saveStatus = 'saving';
			errorMessage = '';
			saveLocalData();
			
			// Inject currently active provider details explicitly down into Rust config
			const override = localOverrides[config.provider];
			if (override) {
				config.api_key = override.api_key;
				config.api_url = override.api_url || defaultProviderUrls[config.provider] || config.api_url;
			} else {
				const targetCustom = customProviders.find(p => p.id === config.provider);
				if (targetCustom) {
					config.api_key = targetCustom.api_key;
					config.api_url = targetCustom.api_url;
				}
			}

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
						onclick={() => activeTab = 'providers'}
						class="flex items-center gap-3 px-3 py-2.5 text-xs rounded-sm transition-all focus:outline-none {activeTab === 'providers' ? 'bg-primary/10 text-primary border-l-2 border-primary' : 'text-muted-foreground hover:bg-muted/30 hover:text-foreground border-l-2 border-transparent'}"
					>
						<Box class="w-4 h-4" />
						<span>Providers</span>
					</button>
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
					{:else if activeTab === 'providers'}
						<div class="space-y-6 animate-in fade-in duration-300 max-w-2xl mx-auto pb-8">
							<div class="flex items-center justify-between pb-2 border-b border-border/50">
								<div>
									<h3 class="text-sm font-bold tracking-wider text-foreground">Provider Configuration</h3>
									<p class="text-[10px] text-muted-foreground opacity-70 mt-1 uppercase tracking-widest">Connect and manage AI endpoints</p>
								</div>
								<button class="text-[10px] uppercase font-bold tracking-widest bg-primary/10 hover:bg-primary/20 text-primary px-3 py-1.5 rounded-sm transition-colors flex items-center gap-1.5 border border-primary/20 shadow-xs" onclick={addNewCustomProvider}>
									<Plus class="w-3 h-3" /> ADD CUSTOM
								</button>
							</div>

							<div class="space-y-3">
								<!-- Provider Loops -->
								{#each [...providersList.map(p => ({...p, is_custom: false})), ...customProviders.map(p => ({name: p.id, display_name: p.display_name, local: false, is_custom: true, api_type: p.api_type, models: p.models, api_url: p.api_url, api_key: p.api_key}))] as provider (provider.name)}
									<div class="border border-border/50 rounded-md bg-muted/5 overflow-hidden transition-all duration-200 {expandingProvider === provider.name ? 'ring-1 ring-primary/30 shadow-md' : 'hover:bg-muted/10'}">
										<button class="w-full px-4 py-3 flex items-center justify-between group" onclick={() => toggleProviderConfig(provider.name)}>
											<div class="flex items-center gap-3">
												<div class="w-8 h-8 rounded-sm bg-background flex items-center justify-center border border-border/50 shadow-xs">
													{#if provider.local}<Cpu class="w-4 h-4 text-amber-500" />
													{:else if provider.is_custom}<Box class="w-4 h-4 text-purple-400" />
													{:else}<Server class="w-4 h-4 text-blue-500" />{/if}
												</div>
												<div class="flex flex-col items-start gap-1">
													<span class="text-sm font-bold text-foreground tracking-wide leading-none group-hover:text-primary transition-colors">{provider.display_name}</span>
													<div class="flex items-center gap-1.5">
														{#if provider.local}
															<span class="text-[8px] px-1 py-px rounded-sm bg-amber-500/10 text-amber-500 font-bold tracking-widest border border-amber-500/20 leading-none">LOCAL</span>
														{:else if provider.is_custom}
															<span class="text-[8px] px-1 py-px rounded-sm bg-purple-500/10 text-purple-400 font-bold tracking-widest border border-purple-500/20 leading-none">CUSTOM</span>
														{:else}
															<span class="text-[8px] px-1 py-px rounded-sm bg-blue-500/10 text-blue-400 font-bold tracking-widest border border-blue-500/20 leading-none">BUILT-IN</span>
														{/if}
														
														{#if (provider.is_custom && 'api_key' in provider && provider.api_key) || (!provider.is_custom && localOverrides[provider.name]?.api_key)}
															<span class="text-[8px] px-1 py-px rounded-sm bg-green-500/10 text-green-500 font-bold tracking-widest border border-green-500/20 leading-none flex items-center gap-0.5"><KeyRound class="w-2 h-2" /> KEY</span>
														{/if}
													</div>
												</div>
											</div>
											<div class="flex items-center gap-4">
												{#if config.provider === provider.name}
													<span class="text-[10px] font-bold text-primary animate-pulse tracking-widest px-2 py-0.5 border border-primary/20 rounded-full bg-primary/5 hidden sm:inline-block">ACTIVE</span>
												{/if}
												<span class="text-[10px] uppercase font-bold tracking-widest text-muted-foreground group-hover:text-foreground transition-colors px-2 py-1 rounded bg-background/50 border border-border/30 shadow-xs">
													{expandingProvider === provider.name ? 'CLOSE' : 'CONFIG'}
												</span>
											</div>
										</button>

										{#if expandingProvider === provider.name}
											<div class="border-t border-border/50 p-4 bg-muted/10 space-y-4 animate-in slide-in-from-top-2 duration-200">
												{#if provider.is_custom}
													<!-- Custom Provider Config -->
													{@const cp = customProviders.find(c => c.id === provider.name)!}
													<div class="space-y-4">
														<div class="flex justify-between items-center pb-2 border-b border-border/30">
															<span class="text-[10px] text-muted-foreground uppercase font-bold tracking-widest">Custom Identity</span>
															<button class="text-red-500/70 hover:text-red-500 text-[10px] font-bold tracking-widest uppercase flex items-center gap-1 transition-colors" onclick={() => deleteCustomProvider(provider.name)}>
																<Trash2 class="w-3 h-3" /> Delete
															</button>
														</div>
														<div class="grid grid-cols-2 gap-4">
															<div class="space-y-1.5 flex flex-col">
																<span class="text-[10px] uppercase font-bold text-muted-foreground">Display Name</span>
																<input type="text" bind:value={cp.display_name} class="w-full h-8 bg-background border border-border/80 rounded px-3 text-xs focus:border-primary/50 font-bold tracking-wide" />
															</div>
															<div class="space-y-1.5 flex flex-col">
																<span class="text-[10px] uppercase font-bold text-muted-foreground">Protocol Format</span>
																<select bind:value={cp.api_type} class="w-full h-8 bg-background border border-border/80 rounded px-2 text-xs focus:border-primary/50 font-mono text-muted-foreground">
																	<option value="openai-completions">OpenAI Spec</option>
																	<option value="anthropic-completions">Anthropic Spec</option>
																</select>
															</div>
														</div>
														
														<div class="grid grid-cols-2 gap-4">
															<div class="space-y-1.5 flex flex-col">
																<span class="text-[10px] uppercase font-bold text-muted-foreground flex items-center gap-1"><Link2 class="w-3 h-3" /> Base URL</span>
																<input type="text" bind:value={cp.api_url} placeholder="https://..." class="w-full h-8 bg-background border border-border/80 rounded px-3 text-xs focus:border-primary/50 font-mono" />
															</div>
															<div class="space-y-1.5 flex flex-col">
																<span class="text-[10px] uppercase font-bold text-muted-foreground flex items-center gap-1"><KeyRound class="w-3 h-3" /> Secret Key</span>
																<input type="password" bind:value={cp.api_key} placeholder="sk-..." class="w-full h-8 bg-background border border-border/80 rounded px-3 text-xs focus:border-primary/50 font-mono placeholder:text-muted-foreground/30" />
															</div>
														</div>
														
														<div class="pt-4 border-t border-border/30">
															<div class="flex items-center justify-between mb-3">
																<span class="text-[10px] uppercase font-bold text-muted-foreground tracking-widest flex items-center gap-1.5"><Cpu class="w-3 h-3" /> Supported Models</span>
																<button class="text-[10px] text-primary hover:text-primary/70 font-bold tracking-widest uppercase flex items-center gap-1 transition-colors" onclick={() => addModelToCustom(provider.name)}>
																	<Plus class="w-3 h-3" /> Add Model
																</button>
															</div>
															<div class="space-y-2">
																{#each cp.models as model, idx (idx)}
																	<div class="flex items-center gap-2">
																		<input type="text" bind:value={model.name} placeholder="Display Name (e.g. GPT-4)" class="flex-[0.4] h-8 bg-background border border-border/80 rounded px-3 text-xs focus:border-primary/50 font-bold" />
																		<input type="text" bind:value={model.id} placeholder="Strict ID (e.g. gpt-4-turbo)" class="flex-[0.6] h-8 bg-background border border-border/80 rounded px-3 text-xs focus:border-primary/50 font-mono text-muted-foreground" />
																		<button class="w-8 h-8 flex items-center justify-center text-red-500/50 hover:text-red-500 hover:bg-red-500/10 rounded transition-colors shrink-0 border border-transparent hover:border-red-500/20" onclick={() => removeModelFromCustom(provider.name, idx)}>
																			<X class="w-3 h-3" />
																		</button>
																	</div>
																{/each}
																{#if cp.models.length === 0}
																	<div class="text-xs text-muted-foreground/50 italic text-center py-2">No models mapped. App will fail to load context if empty.</div>
																{/if}
															</div>
														</div>
													</div>
												{:else}
													<!-- Built-in & Local Config -->
													<div class="grid grid-cols-2 gap-4">
														<div class="space-y-1.5 flex flex-col">
															<label class="text-[10px] uppercase font-bold text-muted-foreground tracking-widest flex items-center justify-between">
																<span class="flex items-center gap-1"><Link2 class="w-3 h-3" /> Base URL</span>
																{#if !provider.local}<span class="text-[8px] text-orange-500/70 opacity-60">SYSTEM LOCKED</span>{/if}
															</label>
															{#if provider.local}
																<input type="text" bind:value={localOverrides[provider.name]!.api_url} class="w-full h-8 bg-background border border-border/80 rounded px-3 text-xs focus:border-primary/50 font-mono" />
															{:else}
																<input type="text" readonly value={defaultProviderUrls[provider.name] || 'Internal Zeroclaw Route'} class="w-full h-8 bg-muted border border-border/30 rounded px-3 text-xs text-muted-foreground cursor-not-allowed font-mono shadow-inner" />
															{/if}
														</div>
														<div class="space-y-1.5 flex flex-col">
															<label class="text-[10px] uppercase font-bold text-muted-foreground tracking-widest flex items-center gap-1"><KeyRound class="w-3 h-3" /> API Key</label>
															<input type="password" bind:value={localOverrides[provider.name]!.api_key} placeholder="Enter secret to authenticate..." class="w-full h-8 bg-background border border-border/80 rounded px-3 text-xs focus:border-primary/50 font-mono placeholder:text-muted-foreground/30" />
														</div>
													</div>
												{/if}
												<div class="flex justify-end pt-2">
													<button class="text-xs tracking-widest uppercase font-bold bg-primary text-primary-foreground hover:bg-primary/90 px-4 py-1.5 rounded shadow-xs" onclick={() => {config.provider = provider.name; expandingProvider = null;}}>Activate connection</button>
												</div>
											</div>
										{/if}
									</div>
								{/each}
							</div>
						</div>
					{:else if activeTab === 'general'}
						<div class="space-y-8 animate-in fade-in duration-300 max-w-xl mx-auto pt-4">
							<div class="space-y-4">
								<h3 class="text-sm font-bold tracking-wider text-foreground pb-2 border-b border-border/50">Core Parameters</h3>
								
								<div class="space-y-6">
									<div class="flex flex-col gap-2">
										<label class="text-[11px] uppercase tracking-widest text-muted-foreground font-bold flex items-center justify-between">
											Active Provider
											<button onclick={() => activeTab = 'providers'} class="text-[9px] text-primary hover:underline">Manage Hub</button>
										</label>
										<select bind:value={config.provider} class="w-full h-10 bg-muted/10 border border-border/80 rounded-sm px-3 text-sm text-foreground focus:outline-none focus:border-primary/50 transition-all font-bold tracking-wide shadow-xs appearance-none">
											<optgroup label="System Intelligence">
												{#each providersList as p (p.name)}
													<option value={p.name}>{p.display_name} {p.local ? '(Local)' : ''}</option>
												{/each}
											</optgroup>
											{#if customProviders.length > 0}
												<optgroup label="Custom Intelligence">
													{#each customProviders as p (p.id)}
														<option value={p.id}>{p.display_name} (Custom)</option>
													{/each}
												</optgroup>
											{/if}
										</select>
									</div>

									<div class="flex flex-col gap-2">
										<span class="text-[11px] uppercase tracking-widest text-muted-foreground font-bold">Inference Model</span>
										{#if activeCustomProvider && activeCustomProvider.models && activeCustomProvider.models.length > 0}
											<select bind:value={config.model} class="w-full h-10 bg-muted/10 border border-border/80 rounded-sm px-3 text-sm text-foreground focus:outline-none focus:border-primary/50 transition-all font-mono shadow-xs appearance-none">
												{#each activeCustomProvider.models as m (m.id)}
													<option value={m.id}>{m.name} [{m.id}]</option>
												{/each}
											</select>
										{:else}
											<input type="text" bind:value={config.model} placeholder="e.g. gpt-4o" class="w-full h-10 bg-muted/10 border border-border/80 rounded-sm px-3 text-sm text-foreground focus:outline-none focus:border-primary/50 transition-all font-mono placeholder:text-muted-foreground/30 shadow-xs" />
										{/if}
									</div>

									<div class="flex flex-col gap-2 pt-4 border-t border-border/30">
										<div class="flex items-center justify-between">
											<label for="temperature" class="text-[11px] uppercase tracking-widest text-muted-foreground font-bold flex items-center gap-2"><Settings class="w-3.5 h-3.5 opacity-50" /> Temperature: </label>
											<span class="text-primary font-mono font-bold">{config.temperature.toFixed(1)}</span>
										</div>
										<input id="temperature" type="range" min="0.0" max="2.0" step="0.1" bind:value={config.temperature} class="w-full h-2 mt-2 bg-muted/50 rounded-full appearance-none cursor-pointer accent-primary hover:bg-muted border border-border/50 shadow-inner" />
										<div class="flex justify-between text-[9px] text-muted-foreground font-mono mt-1 opacity-50">
											<span>0.0 (Precise)</span>
											<span>1.0 (Balanced)</span>
											<span>2.0 (Creative)</span>
										</div>
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
