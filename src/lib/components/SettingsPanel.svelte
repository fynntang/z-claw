<script lang="ts">
  import { showSettings, config } from '$lib/stores/app';
  import { Icon } from '@lucide/svelte';

  let activeTab = $state('model');
  let localModelPath = $state('');

  const tabs = [
    { id: 'model', name: '模型与 API', icon: 'layers' },
    { id: 'mcp', name: 'MCP 服务', icon: 'box' },
    { id: 'skills', name: '技能', icon: 'wand-2' },
    { id: 'channels', name: 'IM 频道', icon: 'hash' },
    { id: 'workspace', name: '工作区', icon: 'monitor' },
    { id: 'about', name: '关于', icon: 'info' },
  ];

  function close() {
    showSettings.set(false);
  }

  function saveConfig() {
    // TODO: Save to local storage / file
    console.log('Saving config:', $config);
    close();
  }
</script>

{#if $showSettings}
  <div class="settings-overlay" onclick={close}>
    <div class="settings-panel" onclick={(e) => e.stopPropagation()}>
      <!-- Sidebar -->
      <div class="settings-sidebar">
        <h2>设置中心</h2>
        <nav class="settings-nav">
          {#each tabs as tab}
            <button 
              class="nav-item {activeTab === tab.id ? 'active' : ''}"
              onclick={() => activeTab = tab.id}
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

        {#if activeTab === 'model'}
          <div class="settings-section">
            <h3>模型与 API 配置</h3>

            <div class="form-group">
              <label>API Key</label>
              <input 
                type="password" 
                bind:value={$config.apiKey}
                placeholder="sk-..."
              />
            </div>

            <div class="form-group">
              <label>API Endpoint</label>
              <input 
                type="text" 
                bind:value={$config.apiUrl}
                placeholder="https://api.openai.com/v1"
              />
            </div>

            <div class="form-group">
              <label>默认模型</label>
              <select bind:value={$config.model}>
                <option value="gpt-4o-mini">OpenAI: GPT-4o-mini</option>
                <option value="gpt-4o">OpenAI: GPT-4o</option>
                <option value="claude-3-5-sonnet-20241022">Anthropic: Claude 3.5 Sonnet</option>
                <option value="deepseek-chat">DeepSeek: Chat</option>
              </select>
            </div>

            <div class="form-group">
              <label>本地推理权重路径 (可选)</label>
              <div class="input-with-btn">
                <input 
                  type="text" 
                  bind:value={localModelPath}
                  placeholder="/path/to/model.gguf"
                  class="mono"
                />
                <button>浏览</button>
              </div>
            </div>

            <button class="save-btn" onclick={saveConfig}>
              保存配置
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
                  <span class="value">OpenAI Compatible API</span>
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
            <h3>{tabs.find(t => t.id === activeTab)?.name}</h3>
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

  .input-with-btn button:hover {
    background: #3f3f46;
    color: white;
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

  .save-btn:hover {
    background: #3b82f6;
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