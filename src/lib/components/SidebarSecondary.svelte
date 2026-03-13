<script lang="ts">
  import { sessions, currentSessionId, messages } from '$lib/stores/app';
  import {CalendarClock,Plus,Slack,MessageCircle,Hash} from '@lucide/svelte';
  import { generateId, getCurrentTime } from '$lib/stores/app';

  // Mock data
  const mockSessions = [
    { id: '1', title: '系统架构分析', preview: '如何优化 ZeroClaw 吞吐量？', time: '12:45', messages: [] },
    { id: '2', title: 'IM 机器人联调', preview: 'Slack 签名验证失败问题排查', time: '昨天', messages: [] },
  ];

  const mockTasks = [
    { id: '1', name: '每日代码清理', time: '09:00', active: true },
    { id: '2', name: '周五报表导出', time: 'Paused', active: false },
  ];

  const mockChannels = [
    { id: '1', name: '# dev-ops-monitoring', type: 'slack', active: true },
    { id: '2', name: 'Telegram: Alpha-Group', type: 'telegram', active: false },
  ];

  $sessions = mockSessions;

  function selectSession(id: string) {
    currentSessionId.set(id);
  }

  function newSession() {
    const id = generateId();
    $sessions = [
      { id, title: '新对话', preview: '', time: '现在', messages: [] },
      ...$sessions
    ];
    currentSessionId.set(id);
    messages.set([]);
  }
</script>

<section class="sidebar-secondary">
  <div class="header">
    <h1>ZClaw Dashboard</h1>
  </div>

  <div class="content">
    <!-- 定时任务 -->
    <div class="section">
      <div class="section-header">
        <span class="section-title">
          <CalendarClock size={12} />
          定时任务
        </span>
        <button class="add-btn"><Plus size={12} /></button>
      </div>
      <div class="section-items">
        {#each mockTasks as task}
          <div class="task-item">
            <div class="status-dot {task.active ? 'active' : ''}"></div>
            <span class="item-name">{task.name}</span>
            <span class="item-time">{task.time}</span>
          </div>
        {/each}
      </div>
    </div>

    <!-- IM 频道 -->
    <div class="section">
      <div class="section-header">
        <span class="section-title">
          <Hash size={12} />
          IM 频道
        </span>
        <button class="add-btn"><Plus size={12} /></button>
      </div>
      <div class="section-items">
        {#each mockChannels as channel}
          <div class="channel-item {channel.active ? 'active' : ''}">
              {#if channel.type === 'slack'}
                  <Slack size={12}/>
                  {:else }
                  <MessageCircle size={12}/>
                  {/if}

            <span class="item-name">{channel.name}</span>
          </div>
        {/each}
      </div>
    </div>

    <!-- 会话历史 -->
    <div class="section">
      <div class="section-header">
        <span class="section-title">最近会话</span>
        <button class="add-btn" onclick={newSession}><Plus size={12} /></button>
      </div>
      <div class="section-items">
        {#each $sessions as session}
          <div 
            class="session-item {$currentSessionId === session.id ? 'active' : ''}"
            onclick={() => selectSession(session.id)}
          >
            <div class="session-title">{session.title}</div>
            <div class="session-time">{session.time}</div>
          </div>
        {/each}
      </div>
    </div>
  </div>
</section>

<style>
  .sidebar-secondary {
    width: 256px;
    border-right: 1px solid rgba(39, 39, 42, 0.5);
    background: rgba(12, 12, 14, 0.5);
    display: flex;
    flex-direction: column;
  }

  .header {
    padding: 16px;
    border-bottom: 1px solid rgba(39, 39, 42, 0.5);
  }

  .header h1 {
    font-size: 14px;
    font-weight: bold;
    color: white;
    letter-spacing: -0.02em;
  }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 12px;
  }

  .content::-webkit-scrollbar {
    width: 4px;
  }

  .content::-webkit-scrollbar-thumb {
    background: #27272a;
    border-radius: 10px;
  }

  .section {
    margin-bottom: 24px;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 8px;
    margin-bottom: 8px;
  }

  .section-title {
    font-size: 10px;
    font-weight: bold;
    color: #71717a;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .add-btn {
    background: transparent;
    border: none;
    color: #52525b;
    cursor: pointer;
    padding: 2px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .add-btn:hover {
    color: #60a5fa;
  }

  .section-items {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .task-item, .channel-item, .session-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px;
    border-radius: 8px;
    font-size: 12px;
    color: #a1a1aa;
    cursor: pointer;
    transition: all 0.15s;
  }

  .task-item:hover, .channel-item:hover, .session-item:hover {
    background: rgba(24, 24, 27, 0.5);
    color: #e4e4e7;
  }

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #52525b;
    flex-shrink: 0;
  }

  .status-dot.active {
    background: #10b981;
  }

  .item-name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .item-time {
    font-size: 9px;
    color: #52525b;
  }

  .channel-item.active {
    background: rgba(39, 39, 42, 0.4);
    color: white;
  }

  .session-item.active {
    background: rgba(39, 39, 42, 0.4);
    border: 1px solid rgba(63, 63, 70, 0.5);
  }

  .session-item {
    flex-direction: column;
    align-items: flex-start;
    border: 1px solid transparent;
  }

  .session-title {
    font-size: 11px;
    font-weight: 500;
    color: #a1a1aa;
  }

  .session-item:hover .session-title,
  .session-item.active .session-title {
    color: #e4e4e7;
  }

  .session-time {
    font-size: 9px;
    color: #3f3f46;
    margin-top: 4px;
  }
</style>