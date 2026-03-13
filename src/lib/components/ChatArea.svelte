<script lang="ts">
  import { messages, showAgentInfo, config } from '$lib/stores/app';
  import { invoke } from '@tauri-apps/api/core';
  import { Icon } from '@lucide/svelte';
  import { generateId, getCurrentTime, type Message } from '$lib/stores/app';

  let input = $state('');
  let loading = $state(false);
  let cpuValue = $state('--');
  let memValue = $state('--');

  // Simulate resource monitoring
  $effect(() => {
    const interval = setInterval(() => {
      cpuValue = (Math.random() * 15 + 3).toFixed(1);
      memValue = Math.floor(Math.random() * 40 + 50).toString();
    }, 3000);
    return () => clearInterval(interval);
  });

  // Initial message
  $effect(() => {
    if ($messages.length === 0) {
      $messages = [
        {
          id: generateId(),
          role: 'assistant',
          content: '您好，我是 ZClaw。当前已为您加载 [通用助手] 核心。请配置您的 API Key 开始对话。',
          time: getCurrentTime()
        }
      ];
    }
  });

  async function sendMessage() {
    const text = input.trim();
    if (!text || loading) return;

    input = '';
    
    // Add user message
    const userMsg: Message = {
      id: generateId(),
      role: 'user',
      content: text,
      time: getCurrentTime()
    };
    $messages = [...$messages, userMsg];
    loading = true;

    try {
      const response = await invoke<{ content: string; success: boolean; error?: string }>('chat', {
        message: text,
        apiKey: $config.apiKey || null,
        apiUrl: $config.apiUrl || null,
        model: $config.model || null,
      });

      const assistantMsg: Message = {
        id: generateId(),
        role: 'assistant',
        content: response.success ? response.content : `错误: ${response.error}`,
        time: getCurrentTime()
      };
      $messages = [...$messages, assistantMsg];
    } catch (e) {
      const errorMsg: Message = {
        id: generateId(),
        role: 'assistant',
        content: `请求失败: ${e}`,
        time: getCurrentTime()
      };
      $messages = [...$messages, errorMsg];
    } finally {
      loading = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  }

  function scrollToBottom(el: HTMLElement) {
    el.scrollTop = el.scrollHeight;
  }
</script>

<main class="chat-main">
  <!-- Header -->
  <header class="chat-header">
    <div class="agent-info">
      <div class="agent-avatar">
        <Icon name="bot" size={20} />
      </div>
      <div class="agent-details">
        <div class="agent-name">通用助手</div>
        <div class="agent-status">v0.1.0 · {loading ? '思考中...' : '就绪'}</div>
      </div>
    </div>

    <div class="header-actions">
      <div class="resource-stats">
        <span><Icon name="cpu" size={12} /> {cpuValue}%</span>
        <span><Icon name="database" size={12} /> {memValue}MB</span>
      </div>
      <div class="divider"></div>
      <button class="detail-btn" onclick={() => showAgentInfo.set(!$showAgentInfo)}>
        <Icon name="info" size={14} />
        Agent 详情
      </button>
    </div>
  </header>

  <!-- Messages -->
  <div class="messages-container" use:scrollToBottom>
    {#each $messages as msg}
      <div class="message {msg.role}">
        <div class="message-avatar {msg.role}">
          <Icon name={msg.role === 'user' ? 'user' : 'bot'} size={16} />
        </div>
        <div class="message-body">
          <div class="message-content {msg.role}">
            {msg.content}
          </div>
          <div class="message-time">{msg.time}</div>
        </div>
      </div>
    {/each}
    
    {#if loading}
      <div class="message assistant">
        <div class="message-avatar assistant">
          <Icon name="bot" size={16} />
        </div>
        <div class="message-body">
          <div class="message-content assistant loading">
            <span class="dot"></span>
            <span class="dot"></span>
            <span class="dot"></span>
          </div>
        </div>
      </div>
    {/if}
  </div>

  <!-- Agent Info Card -->
  {#if $showAgentInfo}
    <div class="agent-card">
      <div class="agent-card-header">
        <div class="agent-card-avatar">
          <Icon name="bot" size={28} />
        </div>
        <div>
          <h3>通用助手</h3>
          <p>轻量级 AI 助手</p>
        </div>
      </div>
      <div class="agent-card-skills">
        <div class="skills-label">核心技能</div>
        <div class="skills-list">
          <span class="skill blue">对话</span>
          <span class="skill purple">分析</span>
          <span class="skill green">代码</span>
        </div>
      </div>
      <div class="agent-card-info">
        <div class="info-row">
          <span>运行环境</span>
          <span>Local (ZClaw)</span>
        </div>
        <div class="info-row">
          <span>当前模型</span>
          <span>{$config.model || '未配置'}</span>
        </div>
      </div>
    </div>
  {/if}

  <!-- Input -->
  <div class="input-container">
    <div class="input-box">
      <textarea
        bind:value={input}
        onkeydown={handleKeydown}
        placeholder="发送指令或询问 ZClaw..."
        rows="1"
        disabled={loading}
      ></textarea>

      <div class="input-actions">
        <div class="input-tools">
          <button><Icon name="zap" size={14} /></button>
          <button><Icon name="paperclip" size={14} /></button>
          <div class="tool-divider"></div>
          <span class="status-text">MCP: Ready</span>
        </div>
        <button class="send-btn" onclick={sendMessage} disabled={loading || !input.trim()}>
          <span>发送</span>
          <Icon name="send" size={14} />
        </button>
      </div>
    </div>
  </div>
</main>

<style>
  .chat-main {
    flex: 1;
    display: flex;
    flex-direction: column;
    position: relative;
    background: #09090b;
  }

  .chat-header {
    height: 56px;
    border-bottom: 1px solid rgba(39, 39, 42, 0.5);
    padding: 0 24px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: rgba(0, 0, 0, 0.2);
    backdrop-filter: blur(12px);
  }

  .agent-info {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .agent-avatar {
    width: 32px;
    height: 32px;
    border-radius: 8px;
    background: linear-gradient(135deg, #2563eb 0%, #7c3aed 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
  }

  .agent-name {
    font-size: 12px;
    font-weight: bold;
    color: white;
  }

  .agent-status {
    font-size: 10px;
    color: #71717a;
    margin-top: 2px;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .resource-stats {
    display: flex;
    gap: 16px;
    font-size: 10px;
    font-family: 'JetBrains Mono', monospace;
    color: #52525b;
  }

  .resource-stats span {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .divider {
    width: 1px;
    height: 16px;
    background: #27272a;
  }

  .detail-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-radius: 8px;
    background: rgba(24, 24, 27, 0.5);
    border: 1px solid #27272a;
    color: #71717a;
    font-size: 12px;
    cursor: pointer;
    transition: all 0.15s;
  }

  .detail-btn:hover {
    color: white;
  }

  .messages-container {
    flex: 1;
    overflow-y: auto;
    padding: 32px;
    display: flex;
    flex-direction: column;
    gap: 32px;
  }

  .messages-container::-webkit-scrollbar {
    width: 4px;
  }

  .messages-container::-webkit-scrollbar-thumb {
    background: #27272a;
    border-radius: 10px;
  }

  .message {
    display: flex;
    gap: 16px;
    align-items: flex-start;
  }

  .message.user {
    flex-direction: row-reverse;
  }

  .message-avatar {
    width: 32px;
    height: 32px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .message-avatar.user {
    background: #27272a;
    border: 1px solid #3f3f46;
    color: #71717a;
  }

  .message-avatar.assistant {
    background: rgba(37, 99, 235, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.2);
    color: #60a5fa;
  }

  .message-body {
    max-width: 75%;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .message.user .message-body {
    align-items: flex-end;
  }

  .message-content {
    padding: 12px 16px;
    border-radius: 16px;
    font-size: 14px;
    line-height: 1.6;
    white-space: pre-wrap;
  }

  .message-content.user {
    background: #2563eb;
    color: white;
    box-shadow: 0 4px 12px rgba(37, 99, 235, 0.2);
  }

  .message-content.assistant {
    background: rgba(24, 24, 27, 0.5);
    border: 1px solid #27272a;
    color: #e4e4e7;
  }

  .message-content.loading {
    display: flex;
    gap: 4px;
    padding: 16px;
  }

  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #52525b;
    animation: bounce 1.4s infinite ease-in-out both;
  }

  .dot:nth-child(1) { animation-delay: -0.32s; }
  .dot:nth-child(2) { animation-delay: -0.16s; }

  @keyframes bounce {
    0%, 80%, 100% { transform: scale(0); }
    40% { transform: scale(1); }
  }

  .message-time {
    font-size: 10px;
    color: #3f3f46;
    padding: 0 4px;
  }

  .agent-card {
    position: absolute;
    right: 24px;
    top: 64px;
    width: 320px;
    background: rgba(12, 12, 14, 0.9);
    backdrop-filter: blur(12px);
    border: 1px solid #27272a;
    border-radius: 16px;
    padding: 20px;
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
    z-index: 40;
  }

  .agent-card-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 16px;
  }

  .agent-card-avatar {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    background: linear-gradient(135deg, #2563eb 0%, #7c3aed 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
  }

  .agent-card-header h3 {
    font-size: 14px;
    font-weight: bold;
    color: white;
  }

  .agent-card-header p {
    font-size: 10px;
    color: #71717a;
    margin-top: 2px;
  }

  .skills-label {
    font-size: 10px;
    font-weight: bold;
    color: #52525b;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 8px;
  }

  .skills-list {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-bottom: 16px;
  }

  .skill {
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 9px;
    font-weight: 500;
  }

  .skill.blue {
    background: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.2);
    color: #60a5fa;
  }

  .skill.purple {
    background: rgba(168, 85, 247, 0.1);
    border: 1px solid rgba(168, 85, 247, 0.2);
    color: #c084fc;
  }

  .skill.green {
    background: rgba(16, 185, 129, 0.1);
    border: 1px solid rgba(16, 185, 129, 0.2);
    color: #34d399;
  }

  .agent-card-info {
    border-top: 1px solid #27272a;
    padding-top: 12px;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    font-size: 10px;
    margin-bottom: 8px;
  }

  .info-row span:first-child {
    color: #71717a;
  }

  .info-row span:last-child {
    color: #d4d4d8;
  }

  .input-container {
    padding: 24px;
  }

  .input-box {
    max-width: 900px;
    margin: 0 auto;
    background: rgba(24, 24, 27, 0.8);
    border: 1px solid #27272a;
    border-radius: 16px;
    padding: 8px;
    transition: border-color 0.15s;
  }

  .input-box:focus-within {
    border-color: #3f3f46;
  }

  .input-box textarea {
    width: 100%;
    background: transparent;
    border: none;
    outline: none;
    resize: none;
    padding: 12px;
    font-size: 14px;
    color: #e4e4e7;
    font-family: inherit;
    max-height: 160px;
    min-height: 44px;
  }

  .input-box textarea::placeholder {
    color: #3f3f46;
  }

  .input-actions {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 4px 8px;
    border-top: 1px solid rgba(39, 39, 42, 0.5);
    margin-top: 4px;
  }

  .input-tools {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .input-tools button {
    background: transparent;
    border: none;
    color: #52525b;
    padding: 6px;
    border-radius: 6px;
    cursor: pointer;
    transition: color 0.15s;
  }

  .input-tools button:hover {
    color: #a1a1aa;
  }

  .tool-divider {
    width: 1px;
    height: 12px;
    background: #27272a;
    margin: 0 4px;
  }

  .status-text {
    font-size: 10px;
    color: #52525b;
    font-family: 'JetBrains Mono', monospace;
  }

  .send-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    background: #2563eb;
    border: none;
    color: white;
    padding: 6px 16px;
    border-radius: 12px;
    font-size: 12px;
    font-weight: bold;
    cursor: pointer;
    transition: all 0.15s;
    box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
  }

  .send-btn:hover:not(:disabled) {
    background: #3b82f6;
  }

  .send-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .send-btn:active:not(:disabled) {
    transform: scale(0.98);
  }
</style>