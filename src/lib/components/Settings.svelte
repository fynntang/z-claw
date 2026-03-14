<script lang="ts">
	/**
	 * Settings modal — ZeroClaw config.
	 * Maps to backend AppConfig: provider, agent, memory, autonomy, timeout, etc.
	 */
	import { X, KeyRound, Link2, Cpu, Save, Sliders, Timer, Bot, Database, Shield } from '@lucide/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { Dialog } from 'bits-ui';

	let { isOpen = $bindable(false) } = $props();

	function closeSettings() {
		isOpen = false;
	}

	/** Default API base URLs per provider (ZeroClaw-compatible). */
	const DEFAULT_API_URLS: Record<string, string> = {
		openai: 'https://api.openai.com/v1',
		anthropic: 'https://api.anthropic.com/v1',
		openrouter: 'https://openrouter.ai/api/v1',
		ollama: 'http://localhost:11434/v1',
		gemini: 'https://generativelanguage.googleapis.com/v1beta/openai/',
		doubao: 'https://ark.cn-beijing.volces.com/api/v3',
		qwen: 'https://dashscope.aliyuncs.com/compatible-mode/v1',
		deepseek: 'https://api.deepseek.com',
		glm: 'https://open.bigmodel.cn/api/paas/v4',
		xai: 'https://api.x.ai/v1',
		together: 'https://api.together.xyz/v1',
		groq: 'https://api.groq.com/openai/v1',
		lmstudio: 'http://localhost:1234/v1',
		llamacpp: 'http://localhost:8080/v1'
	};

	interface UIProviderInfo {
		name: string;
		display_name: string;
		local: boolean;
	}

	type ConfigState = {
		api_key: string;
		api_url: string;
		provider: string;
		model: string;
		temperature: number;
		local_model_path: string | null;
		provider_timeout_secs: number;
		api_path: string | null;
		max_tool_iterations: number;
		max_history_messages: number;
		compact_context: boolean;
		parallel_tools: boolean;
		memory_backend: string;
		memory_auto_save: boolean;
		autonomy_level: string;
	};

	let config = $state<ConfigState>({
		api_key: '',
		api_url: 'https://openrouter.ai/api/v1',
		provider: 'openrouter',
		model: 'anthropic/claude-sonnet-4.6',
		temperature: 0.7,
		local_model_path: null,
		provider_timeout_secs: 120,
		api_path: null,
		max_tool_iterations: 10,
		max_history_messages: 50,
		compact_context: false,
		parallel_tools: false,
		memory_backend: 'sqlite',
		memory_auto_save: true,
		autonomy_level: 'supervised'
	});

	let providersList = $state<UIProviderInfo[]>([]);
	let isLoading = $state(true);
	let saveStatus = $state<'idle' | 'saving' | 'success' | 'error'>('idle');
	let errorMessage = $state('');

	/** When provider changes, set API URL to default for that provider if available. */
	function onProviderChange(newProvider: string) {
		const suggested = DEFAULT_API_URLS[newProvider];
		if (suggested) {
			config.api_url = suggested;
		}
	}

	async function loadConfig() {
		try {
			isLoading = true;
			const [pData, cData] = await Promise.all([
				invoke<UIProviderInfo[]>('providers_list'),
				invoke<ConfigState>('config_get')
			]);
			providersList = pData;
			config = cData;
		} catch (e) {
			console.error('Failed to load config or providers:', e);
		} finally {
			isLoading = false;
		}
	}

	async function saveConfig() {
		try {
			saveStatus = 'saving';
			errorMessage = '';
			const validation = await invoke<{ valid: boolean; errors: string[] }>('config_validate', {
				config
			});
			if (!validation.valid) {
				saveStatus = 'error';
				errorMessage = validation.errors.join(', ');
				return;
			}
			await invoke('config_set', { config });
			saveStatus = 'success';
			setTimeout(() => {
				if (saveStatus === 'success') saveStatus = 'idle';
			}, 2000);
		} catch (e) {
			saveStatus = 'error';
			errorMessage = String(e);
		}
	}

	$effect(() => {
		if (isOpen) {
			loadConfig();
			saveStatus = 'idle';
		}
	});
</script>

{#if isOpen}
	<div
		class="fixed inset-0 z-200 flex items-center justify-center bg-background/90 backdrop-blur-md animate-in fade-in duration-200"
		role="dialog"
		aria-modal="true"
		aria-labelledby="settings-title"
	>
		<div
			class="w-full max-w-md mx-4 bg-card border border-border rounded-xl shadow-2xl flex flex-col overflow-hidden animate-in zoom-in-95 duration-300 slide-in-from-bottom-4"
			style="box-shadow: 0 25px 50px -12px oklch(0 0 0 / 0.15);"
		>
			<!-- Header -->
			<div class="shrink-0 flex items-center justify-between px-5 py-4 border-b border-border bg-muted/30">
				<h2 id="settings-title" class="text-base font-semibold text-foreground tracking-tight">
					ZeroClaw 配置
				</h2>
				<button
					type="button"
					onclick={closeSettings}
					class="p-2 -mr-2 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted transition-colors"
					aria-label="关闭"
				>
					<X class="w-4 h-4" />
				</button>
			</div>

			<!-- Body: single form aligned with ZeroClaw config -->
			<div class="flex-1 overflow-y-auto p-5 space-y-6">
				{#if isLoading}
					<div class="flex flex-col items-center justify-center py-12 gap-3 text-muted-foreground">
						<div class="w-8 h-8 border-2 border-primary/30 border-t-primary rounded-full animate-spin"></div>
						<p class="text-sm">加载配置…</p>
					</div>
				{:else}
					<!-- Provider -->
					<div class="space-y-2">
						<label for="settings-provider" class="text-sm font-medium text-foreground block">
							提供商
						</label>
						<select
							id="settings-provider"
							bind:value={config.provider}
							onchange={() => onProviderChange(config.provider)}
							class="w-full h-10 px-3 rounded-lg border border-input bg-background text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring focus:border-transparent"
						>
							{#each providersList as p (p.name)}
								<option value={p.name}>
									{p.display_name}{p.local ? ' (本地)' : ''}
								</option>
							{/each}
						</select>
					</div>

					<!-- API URL (for local/custom endpoints) -->
					<div class="space-y-2">
						<label for="settings-api-url" class="text-sm font-medium text-foreground flex items-center gap-1.5">
							<Link2 class="w-3.5 h-3.5 text-muted-foreground" />
							API 地址
						</label>
						<input
							id="settings-api-url"
							type="url"
							bind:value={config.api_url}
							placeholder="https://api.openai.com/v1"
							class="w-full h-10 px-3 rounded-lg border border-input bg-background text-foreground text-sm font-mono placeholder:text-muted-foreground/50 focus:outline-none focus:ring-2 focus:ring-ring"
						/>
					</div>

					<!-- API Key -->
					<div class="space-y-2">
						<label for="settings-api-key" class="text-sm font-medium text-foreground flex items-center gap-1.5">
							<KeyRound class="w-3.5 h-3.5 text-muted-foreground" />
							API Key
						</label>
						<input
							id="settings-api-key"
							type="password"
							bind:value={config.api_key}
							placeholder="sk-…"
							autocomplete="off"
							class="w-full h-10 px-3 rounded-lg border border-input bg-background text-foreground text-sm font-mono placeholder:text-muted-foreground/50 focus:outline-none focus:ring-2 focus:ring-ring"
						/>
						{#if ['ollama', 'lmstudio', 'llamacpp'].includes(config.provider)}
							<p class="text-xs text-muted-foreground">本地服务可留空。</p>
						{/if}
					</div>

					<!-- Model -->
					<div class="space-y-2">
						<label for="settings-model" class="text-sm font-medium text-foreground flex items-center gap-1.5">
							<Cpu class="w-3.5 h-3.5 text-muted-foreground" />
							默认模型
						</label>
						<input
							id="settings-model"
							type="text"
							bind:value={config.model}
							placeholder="gpt-4o-mini"
							class="w-full h-10 px-3 rounded-lg border border-input bg-background text-foreground text-sm font-mono placeholder:text-muted-foreground/50 focus:outline-none focus:ring-2 focus:ring-ring"
						/>
					</div>

					<!-- Temperature -->
					<div class="space-y-3">
						<div class="flex items-center justify-between">
							<label for="settings-temperature" class="text-sm font-medium text-foreground flex items-center gap-1.5">
								<Sliders class="w-3.5 h-3.5 text-muted-foreground" />
								温度
							</label>
							<span class="text-sm font-mono text-primary tabular-nums">{config.temperature.toFixed(1)}</span>
						</div>
						<input
							id="settings-temperature"
							type="range"
							min="0"
							max="2"
							step="0.1"
							bind:value={config.temperature}
							class="w-full h-2 rounded-full appearance-none cursor-pointer bg-muted accent-primary"
						/>
						<div class="flex justify-between text-xs text-muted-foreground">
							<span>精确</span>
							<span>均衡</span>
							<span>创意</span>
						</div>
					</div>

					<!-- 请求与超时 -->
					<div class="pt-2 border-t border-border space-y-3">
						<h3 class="text-sm font-medium text-foreground flex items-center gap-1.5">
							<Timer class="w-3.5 h-3.5 text-muted-foreground" />
							请求与超时
						</h3>
						<div class="space-y-2">
							<label for="settings-timeout" class="text-sm text-muted-foreground block">API 超时（秒）</label>
							<input
								id="settings-timeout"
								type="number"
								min="1"
								max="600"
								bind:value={config.provider_timeout_secs}
								class="w-full h-10 px-3 rounded-lg border border-input bg-background text-foreground text-sm font-mono focus:outline-none focus:ring-2 focus:ring-ring"
							/>
						</div>
						<div class="space-y-2">
							<label for="settings-api-path" class="text-sm text-muted-foreground block">API 路径后缀（可选）</label>
							<input
								id="settings-api-path"
								type="text"
								bind:value={config.api_path}
								placeholder="/v1/chat/completions"
								class="w-full h-10 px-3 rounded-lg border border-input bg-background text-foreground text-sm font-mono placeholder:text-muted-foreground/50 focus:outline-none focus:ring-2 focus:ring-ring"
							/>
						</div>
					</div>

					<!-- Agent -->
					<div class="pt-2 border-t border-border space-y-3">
						<h3 class="text-sm font-medium text-foreground flex items-center gap-1.5">
							<Bot class="w-3.5 h-3.5 text-muted-foreground" />
							Agent
						</h3>
						<div class="grid grid-cols-2 gap-3">
							<div class="space-y-2">
								<label for="settings-max-tool-iterations" class="text-sm text-muted-foreground block">最大工具调用次数</label>
								<input
									id="settings-max-tool-iterations"
									type="number"
									min="1"
									max="100"
									bind:value={config.max_tool_iterations}
									class="w-full h-10 px-3 rounded-lg border border-input bg-background text-foreground text-sm font-mono focus:outline-none focus:ring-2 focus:ring-ring"
								/>
							</div>
							<div class="space-y-2">
								<label for="settings-max-history" class="text-sm text-muted-foreground block">最大历史消息数</label>
								<input
									id="settings-max-history"
									type="number"
									min="1"
									max="500"
									bind:value={config.max_history_messages}
									class="w-full h-10 px-3 rounded-lg border border-input bg-background text-foreground text-sm font-mono focus:outline-none focus:ring-2 focus:ring-ring"
								/>
							</div>
						</div>
						<div class="flex flex-col gap-2">
							<label class="flex items-center gap-2 cursor-pointer">
								<input type="checkbox" bind:checked={config.compact_context} class="rounded border-input accent-primary" />
								<span class="text-sm text-muted-foreground">压缩上下文 (compact_context)</span>
							</label>
							<label class="flex items-center gap-2 cursor-pointer">
								<input type="checkbox" bind:checked={config.parallel_tools} class="rounded border-input accent-primary" />
								<span class="text-sm text-muted-foreground">并行工具调用 (parallel_tools)</span>
							</label>
						</div>
					</div>

					<!-- 记忆 -->
					<div class="pt-2 border-t border-border space-y-3">
						<h3 class="text-sm font-medium text-foreground flex items-center gap-1.5">
							<Database class="w-3.5 h-3.5 text-muted-foreground" />
							记忆
						</h3>
						<div class="space-y-2">
							<label for="settings-memory-backend" class="text-sm text-muted-foreground block">后端</label>
							<select
								id="settings-memory-backend"
								bind:value={config.memory_backend}
								class="w-full h-10 px-3 rounded-lg border border-input bg-background text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
							>
								<option value="sqlite">sqlite</option>
								<option value="markdown">markdown</option>
								<option value="none">none</option>
								<option value="lucid">lucid</option>
								<option value="postgres">postgres</option>
								<option value="qdrant">qdrant</option>
							</select>
						</div>
						<label class="flex items-center gap-2 cursor-pointer">
							<input type="checkbox" bind:checked={config.memory_auto_save} class="rounded border-input accent-primary" />
							<span class="text-sm text-muted-foreground">自动保存 (auto_save)</span>
						</label>
					</div>

					<!-- 自主性 -->
					<div class="pt-2 border-t border-border space-y-3">
						<h3 class="text-sm font-medium text-foreground flex items-center gap-1.5">
							<Shield class="w-3.5 h-3.5 text-muted-foreground" />
							自主性
						</h3>
						<div class="space-y-2">
							<label for="settings-autonomy" class="text-sm text-muted-foreground block">级别</label>
							<select
								id="settings-autonomy"
								bind:value={config.autonomy_level}
								class="w-full h-10 px-3 rounded-lg border border-input bg-background text-foreground text-sm focus:outline-none focus:ring-2 focus:ring-ring"
							>
								<option value="readonly">只读 (readonly)</option>
								<option value="supervised">受监督 (supervised)</option>
								<option value="full">完全 (full)</option>
							</select>
							<p class="text-xs text-muted-foreground">控制 Agent 可执行操作的权限范围。</p>
						</div>
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div class="shrink-0 flex items-center justify-between gap-3 px-5 py-4 border-t border-border bg-muted/20">
				<div class="min-w-0 text-sm">
					{#if saveStatus === 'success'}
						<span class="text-green-600 dark:text-green-400 font-medium">已保存</span>
					{:else if saveStatus === 'error'}
						<span class="text-destructive font-medium truncate block" title={errorMessage}>
							{errorMessage || '保存失败'}
						</span>
					{/if}
				</div>
				<div class="flex items-center gap-2 shrink-0">
					<button
						type="button"
						onclick={closeSettings}
						class="px-4 py-2 text-sm font-medium text-muted-foreground hover:text-foreground transition-colors"
					>
						取消
					</button>
					<button
						type="button"
						onclick={saveConfig}
						disabled={isLoading || saveStatus === 'saving'}
						class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-primary text-primary-foreground hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none transition-colors"
					>
						{#if saveStatus === 'saving'}
							<span class="w-3.5 h-3.5 border-2 border-primary-foreground/30 border-t-primary-foreground rounded-full animate-spin"></span>
							保存中…
						{:else}
							<Save class="w-3.5 h-3.5" />
							保存
						{/if}
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}
