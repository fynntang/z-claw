<script lang="ts">
	import { showSettings, config, saveConfig, loadConfig } from '$lib/stores/app';
	import { Icon } from '@lucide/svelte';
	import { AVAILABLE_PROVIDERS, DEFAULT_CONFIG } from '$lib/types/config';
	import { onMount } from 'svelte';
	import { toast } from 'svelte-sonner';

	let activeTab = $state('model');
	let saving = $state(false);
	let loading = $state(true);

	// Local form state (not directly bound to store)
	let formData = $state({
		apiKey: '',
		apiUrl: 'https://api.openai.com/v1',
		provider: 'openai',
		model: 'gpt-4o-mini',
		temperature: 0.7,
		localModelPath: ''
	});

	const tabs = [
		{ id: 'model', name: '模型与 API', icon: 'layers' },
		{ id: 'mcp', name: 'MCP 服务', icon: 'box' },
		{ id: 'skills', name: '技能', icon: 'wand-2' },
		{ id: 'channels', name: 'IM 频道', icon: 'hash' },
		{ id: 'workspace', name: '工作区', icon: 'monitor' },
		{ id: 'about', name: '关于', icon: 'info' }
	];

	// Load config on mount
	onMount(async () => {
		loading = true;
		await loadConfig();
		// Copy store values to form
		formData = {
			apiKey: $config.apiKey,
			apiUrl: $config.apiUrl,
			provider: $config.provider,
			model: $config.model,
			temperature: $config.temperature,
			localModelPath: $config.localModelPath || ''
		};
		loading = false;
	});

	function close() {
		showSettings.set(false);
	}

	async function handleSave() {
		saving = true;
		try {
			const result = await saveConfig({
				apiKey: formData.apiKey,
				apiUrl: formData.apiUrl,
				provider: formData.provider,
				model: formData.model,
				temperature: formData.temperature,
				localModelPath: formData.localModelPath || undefined
			});

			if (result.valid) {
				toast.success('配置已保存');
				close();
			} else {
				toast.error(`保存失败: ${result.errors.join(', ')}`);
			}
		} catch (e) {
			toast.error(`保存失败: ${e}`);
		} finally {
			saving = false;
		}
	}

	// Get models for current provider
	$effect(() => {
		const provider = AVAILABLE_PROVIDERS.find((p) => p.id === formData.provider);
		if (provider && !provider.models.includes(formData.model)) {
			formData.model = provider.models[0];
		}
	});
</script>

{#if $showSettings}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="settings-overlay" onclick={close} onkeydown={(e) => e.key === 'Escape' && close()}>
		<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
		<div class="settings-panel" onclick={(e) => e.stopPropagation()}>
			<!-- Sidebar -->
			<div class="settings-sidebar">
				<h2>设置中心</h2>
				<nav class="settings-nav">
					{#each tabs as tab}
						<button
							class="nav-item {activeTab === tab.id ? 'active' : ''}"
							onclick={() => (activeTab = tab.id)}
						>
							<Icon name={tab.icon} size={14} />
							{tab.name}
						</button>
					{/each}
				</nav>
			</div>

			<!-- Content -->
			<div class="settings-content">
				<button class="close-btn" onclick={close}>
					<Icon name="x" size={24} />
				</button>

				{#if loading}
					<div class="settings-section">
						<p class="loading-text">加载配置中...</p>
					</div>
				{:else if activeTab === 'model'}
					<div class="settings-section">
						<h3>模型与 API 配置</h3>

						<div class="form-group">
							<label for="provider">Provider</label>
							<select id="provider" bind:value={formData.provider}>
								{#each AVAILABLE_PROVIDERS as p}
									<option value={p.id}>{p.name}</option>
								{/each}
							</select>
						</div>

						<div class="form-group">
							<label for="api-key">API Key</label>
							<input
								id="api-key"
								type="password"
								bind:value={formData.apiKey}
								placeholder="sk-..."
							/>
							<p class="hint">API key 将安全存储在 ~/.zeroclaw/config.toml</p>
						</div>

						<div class="form-group">
							<label for="api-url">API Endpoint</label>
							<input
								id="api-url"
								type="text"
								bind:value={formData.apiUrl}
								placeholder="https://api.openai.com/v1"
							/>
						</div>

						<div class="form-group">
							<label for="model">默认模型</label>
							<select id="model" bind:value={formData.model}>
								{#each AVAILABLE_PROVIDERS.find((p) => p.id === formData.provider)?.models || [] as m}
									<option value={m}>{m}</option>
								{/each}
							</select>
						</div>

						<div class="form-group">
							<label for="temperature">Temperature ({formData.temperature.toFixed(1)})</label>
							<input
								id="temperature"
								type="range"
								min="0"
								max="2"
								step="0.1"
								bind:value={formData.temperature}
							/>
							<p class="hint">控制响应的随机性。0 = 确定性，2 = 最大随机性</p>
						</div>

						<div class="form-group">
							<label for="local-model-path">本地推理权重路径 (可选)</label>
							<div class="input-with-btn">
								<input
									id="local-model-path"
									type="text"
									bind:value={formData.localModelPath}
									placeholder="/path/to/model.gguf"
									class="mono"
								/>
								<button disabled>浏览</button>
							</div>
						</div>

						<button class="save-btn" onclick={handleSave} disabled={saving}>
							{saving ? '保存中...' : '保存配置'}
						</button>
					</div>
				{:else if activeTab === 'about'}
					<div class="settings-section">
						<h3>关于 ZClaw</h3>

						<div class="about-content">
							<div class="about-logo">
								<span class="logo-text">ZClaw</span>
							</div>

							<div class="about-info">
								<div class="info-item">
									<span class="label">版本</span>
									<span class="value">0.1.0</span>
								</div>
								<div class="info-item">
									<span class="label">构建</span>
									<span class="value">Tauri + SvelteKit</span>
								</div>
								<div class="info-item">
									<span class="label">引擎</span>
									<span class="value">ZeroClaw</span>
								</div>
								<div class="info-item">
									<span class="label">配置文件</span>
									<span class="value mono">~/.zeroclaw/config.toml</span>
								</div>
							</div>

							<div class="about-links">
								<a href="https://z-claw.ai" target="_blank">官网</a>
								<a href="https://github.com/z-claw/z-claw" target="_blank">GitHub</a>
							</div>
						</div>
					</div>
				{:else}
					<div class="settings-section">
						<h3>{tabs.find((t) => t.id === activeTab)?.name}</h3>
						<p class="coming-soon">功能开发中...</p>
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
	.settings-overlay {
		position: fixed;
		inset: 0;
		z-index: 100;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
	}

	.settings-panel {
		width: 800px;
		height: 600px;
		background: #0c0c0e;
		border: 1px solid #27272a;
		border-radius: 24px;
		overflow: hidden;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		display: flex;
	}

	.settings-sidebar {
		width: 192px;
		border-right: 1px solid rgba(39, 39, 42, 0.5);
		background: rgba(0, 0, 0, 0.2);
		padding: 24px;
	}

	.settings-sidebar h2 {
		font-size: 14px;
		font-weight: bold;
		color: white;
		margin-bottom: 24px;
		padding: 0 12px;
	}

	.settings-nav {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.nav-item {
		display: flex;
		align-items: center;
		gap: 8px;
		width: 100%;
		text-align: left;
		padding: 8px 12px;
		border-radius: 8px;
		font-size: 12px;
		font-weight: 500;
		color: #71717a;
		background: transparent;
		border: none;
		cursor: pointer;
		transition: all 0.15s;
	}

	.nav-item:hover {
		color: #d4d4d8;
	}

	.nav-item.active {
		background: rgba(37, 99, 235, 0.1);
		border-left: 2px solid #3b82f6;
		color: #60a5fa;
	}

	.settings-content {
		flex: 1;
		padding: 40px;
		position: relative;
		overflow-y: auto;
	}

	.close-btn {
		position: absolute;
		right: 24px;
		top: 24px;
		background: transparent;
		border: none;
		color: #52525b;
		cursor: pointer;
		padding: 4px;
	}

	.close-btn:hover {
		color: white;
	}

	.settings-section {
		max-width: 400px;
	}

	.settings-section h3 {
		font-size: 20px;
		font-weight: bold;
		color: white;
		margin-bottom: 32px;
	}

	.form-group {
		margin-bottom: 24px;
	}

	.form-group label {
		display: block;
		font-size: 10px;
		font-weight: bold;
		color: #71717a;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		margin-bottom: 8px;
	}

	.form-group input,
	.form-group select {
		width: 100%;
		background: #18181b;
		border: 1px solid #27272a;
		border-radius: 12px;
		padding: 12px;
		font-size: 14px;
		color: #d4d4d8;
		outline: none;
		transition: border-color 0.15s;
	}

	.form-group input:focus,
	.form-group select:focus {
		border-color: #3b82f6;
	}

	.form-group input.mono {
		font-family: 'JetBrains Mono', monospace;
		font-size: 12px;
	}

	.form-group input[type='range'] {
		padding: 0;
		height: 6px;
		border-radius: 3px;
		background: #27272a;
	}

	.form-group input[type='range']::-webkit-slider-thumb {
		appearance: none;
		width: 16px;
		height: 16px;
		border-radius: 50%;
		background: #3b82f6;
		cursor: pointer;
	}

	.hint {
		font-size: 10px;
		color: #52525b;
		margin-top: 6px;
	}

	.loading-text {
		color: #71717a;
		font-size: 14px;
	}

	.input-with-btn {
		display: flex;
		gap: 8px;
	}

	.input-with-btn input {
		flex: 1;
	}

	.input-with-btn button {
		background: #27272a;
		border: none;
		padding: 0 16px;
		border-radius: 12px;
		font-size: 12px;
		color: #a1a1aa;
		cursor: pointer;
	}

	.input-with-btn button:hover:not(:disabled) {
		background: #3f3f46;
		color: white;
	}

	.input-with-btn button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.save-btn {
		width: 100%;
		background: #2563eb;
		border: none;
		color: white;
		font-weight: bold;
		padding: 12px;
		border-radius: 12px;
		font-size: 14px;
		cursor: pointer;
		transition: background 0.15s;
		box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
	}

	.save-btn:hover:not(:disabled) {
		background: #3b82f6;
	}

	.save-btn:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.coming-soon {
		color: #52525b;
		font-size: 14px;
	}

	.about-content {
		text-align: center;
	}

	.about-logo {
		margin-bottom: 32px;
	}

	.logo-text {
		font-size: 48px;
		font-weight: bold;
		background: linear-gradient(135deg, #2563eb 0%, #7c3aed 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
	}

	.about-info {
		display: flex;
		flex-direction: column;
		gap: 12px;
		margin-bottom: 32px;
	}

	.info-item {
		display: flex;
		justify-content: space-between;
		font-size: 12px;
	}

	.info-item .label {
		color: #71717a;
	}

	.info-item .value {
		color: #d4d4d8;
	}

	.info-item .value.mono {
		font-family: 'JetBrains Mono', monospace;
		font-size: 10px;
	}

	.about-links {
		display: flex;
		justify-content: center;
		gap: 24px;
	}

	.about-links a {
		color: #60a5fa;
		text-decoration: none;
		font-size: 12px;
	}

	.about-links a:hover {
		text-decoration: underline;
	}
</style>
