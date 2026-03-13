<script lang="ts">
	import {
		agents,
		currentAgentId,
		showSettings,
		sessions,
		currentSessionId
	} from '$lib/stores/app';
	import {
		Bot,
		Code,
		Cpu,
		Plus,
		Settings,
		Hash,
		Calendar,
		MessageSquare,
		Wrench,
		ScrollText
	} from '@lucide/svelte';
	import { generateId } from '$lib/stores/app';
	import ToolsPanel from './ToolsPanel.svelte';
	import LogsPanel from './LogsPanel.svelte';

	// Tab state
	let activeTab = $state<'agents' | 'channels' | 'tasks' | 'tools' | 'logs'>('agents');

	// Mock agents
	const mockAgents = [
		{
			id: '1',
			name: '通用助手',
			icon: Bot,
			status: 'running' as const,
			model: 'gpt-4o-mini',
			version: '0.1.0',
			skills: ['对话', '分析'],
			channels: []
		},
		{
			id: '2',
			name: '代码助手',
			icon: Cpu,
			status: 'stopped' as const,
			model: 'gpt-4o',
			version: '0.1.0',
			skills: ['代码', '调试'],
			channels: []
		},
		{
			id: '3',
			name: '数据分析',
			icon: Code,
			status: 'stopped' as const,
			model: 'claude-3-sonnet',
			version: '0.1.0',
			skills: ['数据分析', '可视化'],
			channels: []
		}
	];

	$agents = mockAgents;

	// Mock sessions
	const mockSessions = [
		{ id: '1', title: '系统架构分析', preview: '', time: '12:45', messages: [] },
		{ id: '2', title: 'IM 机器人联调', preview: '', time: '昨天', messages: [] },
		{ id: '3', title: '数据迁移方案', preview: '', time: '3天前', messages: [] }
	];

	$sessions = mockSessions;

	// Mock channels
	const mockChannels = [
		{ id: '1', name: '# dev-ops', type: 'slack', active: true },
		{ id: '2', name: '# alerts', type: 'slack', active: false },
		{ id: '3', name: 'Alpha-Group', type: 'telegram', active: false }
	];

	// Mock tasks
	const mockTasks = [
		{ id: '1', name: '每日代码清理', time: '09:00', active: true },
		{ id: '2', name: '周五报表导出', time: 'Paused', active: false },
		{ id: '3', name: '周报生成', time: '18:00', active: true }
	];

	function selectAgent(id: string) {
		currentAgentId.set(id);
	}

	function selectSession(id: string) {
		currentSessionId.set(id);
	}

	function newSession() {
		const id = generateId();
		$sessions = [{ id, title: '新对话', preview: '', time: '现在', messages: [] }, ...$sessions];
		currentSessionId.set(id);
	}

	function openSettings() {
		showSettings.set(true);
	}
</script>

<aside class="sidebar">
	<!-- Tab Bar -->
	<div class="tab-bar">
		<button
			class="tab-btn {activeTab === 'agents' ? 'active' : ''}"
			onclick={() => (activeTab = 'agents')}
		>
			<Bot size={14} />
			<span>智能体</span>
		</button>
		<button
			class="tab-btn {activeTab === 'channels' ? 'active' : ''}"
			onclick={() => (activeTab = 'channels')}
		>
			<Hash size={14} />
			<span>频道</span>
		</button>
		<button
			class="tab-btn {activeTab === 'tasks' ? 'active' : ''}"
			onclick={() => (activeTab = 'tasks')}
		>
			<Calendar size={14} />
			<span>任务</span>
		</button>
		<button
			class="tab-btn {activeTab === 'tools' ? 'active' : ''}"
			onclick={() => (activeTab = 'tools')}
		>
			<Wrench size={14} />
			<span>工具</span>
		</button>
		<button
			class="tab-btn {activeTab === 'logs' ? 'active' : ''}"
			onclick={() => (activeTab = 'logs')}
		>
			<ScrollText size={14} />
			<span>日志</span>
		</button>
	</div>

	<!-- Main Content Area -->
	<div class="sidebar-content">
		<!-- Left Column: List -->
		<div class="list-column">
			<!-- Agents List -->
			{#if activeTab === 'agents'}
				<div class="list-header">
					<span class="list-title">我的智能体</span>
					<button class="icon-btn" title="添加智能体">
						<Plus size={14} />
					</button>
				</div>
				<div class="list-items">
					{#each $agents as agent}
						{@const AgentIcon = typeof agent.icon === 'string' ? Bot : agent.icon}
						<button
							class="list-item {$currentAgentId === agent.id ? 'active' : ''}"
							onclick={() => selectAgent(agent.id)}
						>
							<div class="item-icon">
								<AgentIcon size={18} />
								{#if agent.status === 'running'}
									<span class="status-indicator running"></span>
								{/if}
							</div>
							<div class="item-info">
								<span class="item-name">{agent.name}</span>
								<span class="item-meta">{agent.model}</span>
							</div>
						</button>
					{/each}
				</div>

				<!-- Channels List -->
			{:else if activeTab === 'channels'}
				<div class="list-header">
					<span class="list-title">IM 频道</span>
					<button class="icon-btn" title="添加频道">
						<Plus size={14} />
					</button>
				</div>
				<div class="list-items">
					{#each mockChannels as channel}
						<div class="list-item {channel.active ? 'active' : ''}">
							<Hash size={16} class="item-icon-simple" />
							<div class="item-info">
								<span class="item-name">{channel.name}</span>
								<span class="item-meta">{channel.type}</span>
							</div>
						</div>
					{/each}
				</div>

				<!-- Tasks List -->
			{:else}
				<div class="list-header">
					<span class="list-title">定时任务</span>
					<button class="icon-btn" title="添加任务">
						<Plus size={14} />
					</button>
				</div>
				<div class="list-items">
					{#each mockTasks as task}
						<div class="list-item {task.active ? 'active' : ''}">
							<div class="task-status {task.active ? 'running' : ''}"></div>
							<div class="item-info">
								<span class="item-name">{task.name}</span>
								<span class="item-meta">{task.time}</span>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>

		<!-- Right Column: Sessions (only for agents tab) -->
		{#if activeTab === 'agents'}
			<div class="sessions-column">
				<div class="list-header">
					<span class="list-title">消息会话</span>
					<button class="icon-btn" onclick={newSession} title="新建会话">
						<Plus size={14} />
					</button>
				</div>
				<div class="list-items">
					{#each $sessions as session}
						<button
							class="session-item {$currentSessionId === session.id ? 'active' : ''}"
							onclick={() => selectSession(session.id)}
						>
							<MessageSquare size={14} class="session-icon" />
							<div class="session-info">
								<span class="session-title">{session.title}</span>
								<span class="session-time">{session.time}</span>
							</div>
						</button>
					{/each}
				</div>
			</div>
		{:else if activeTab === 'tools'}
			<div class="panel-column">
				<ToolsPanel />
			</div>
		{:else if activeTab === 'logs'}
			<div class="panel-column">
				<LogsPanel />
			</div>
		{/if}
	</div>

	<!-- Bottom Actions -->
	<div class="sidebar-footer">
		<button class="footer-btn" onclick={openSettings}>
			<Settings size={16} />
			<span>设置</span>
		</button>
	</div>
</aside>

<style>
	.sidebar {
		width: 320px;
		display: flex;
		flex-direction: column;
		border-right: 1px solid rgba(39, 39, 42, 0.5);
		background: rgba(0, 0, 0, 0.3);
	}

	/* Tab Bar */
	.tab-bar {
		display: flex;
		padding: 8px;
		gap: 4px;
		border-bottom: 1px solid rgba(39, 39, 42, 0.5);
		background: rgba(0, 0, 0, 0.2);
	}

	.tab-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		padding: 8px 12px;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #71717a;
		font-size: 12px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.tab-btn:hover {
		color: #a1a1aa;
		background: rgba(24, 24, 27, 0.5);
	}

	.tab-btn.active {
		color: white;
		background: rgba(37, 99, 235, 0.2);
	}

	/* Content Area */
	.sidebar-content {
		flex: 1;
		display: flex;
		overflow: hidden;
	}

	.list-column {
		flex: 1;
		display: flex;
		flex-direction: column;
		border-right: 1px solid rgba(39, 39, 42, 0.3);
		min-width: 140px;
	}

	.sessions-column {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-width: 140px;
	}

	.panel-column {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-width: 200px;
		background: rgba(0, 0, 0, 0.2);
	}

	/* List Header */
	.list-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px;
		border-bottom: 1px solid rgba(39, 39, 42, 0.3);
	}

	.list-title {
		font-size: 10px;
		font-weight: 600;
		color: #52525b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.icon-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: #52525b;
		cursor: pointer;
		transition: all 0.15s;
	}

	.icon-btn:hover {
		color: #a1a1aa;
		background: rgba(24, 24, 27, 0.5);
	}

	/* List Items */
	.list-items {
		flex: 1;
		overflow-y: auto;
		padding: 8px;
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.list-items::-webkit-scrollbar {
		width: 4px;
	}

	.list-items::-webkit-scrollbar-thumb {
		background: #27272a;
		border-radius: 10px;
	}

	.list-item {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 10px;
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.15s;
		text-align: left;
		width: 100%;
	}

	.list-item:hover {
		background: rgba(24, 24, 27, 0.5);
	}

	.list-item.active {
		background: rgba(37, 99, 235, 0.15);
	}

	.item-icon {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border-radius: 8px;
		background: #18181b;
		color: #71717a;
		flex-shrink: 0;
	}

	.list-item.active .item-icon {
		background: linear-gradient(135deg, #2563eb 0%, #7c3aed 100%);
		color: white;
	}

	.status-indicator {
		position: absolute;
		bottom: -2px;
		right: -2px;
		width: 8px;
		height: 8px;
		border-radius: 50%;
		border: 2px solid #0c0c0e;
	}

	.status-indicator.running {
		background: #10b981;
		box-shadow: 0 0 6px rgba(16, 185, 129, 0.5);
	}

	.item-icon-simple {
		color: #52525b;
		flex-shrink: 0;
	}

	.list-item.active .item-icon-simple {
		color: #60a5fa;
	}

	.item-info {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.item-name {
		font-size: 12px;
		font-weight: 500;
		color: #a1a1aa;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.list-item.active .item-name,
	.list-item:hover .item-name {
		color: #e4e4e7;
	}

	.item-meta {
		font-size: 10px;
		color: #52525b;
	}

	/* Task Status */
	.task-status {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #3f3f46;
		flex-shrink: 0;
	}

	.task-status.running {
		background: #10b981;
		box-shadow: 0 0 6px rgba(16, 185, 129, 0.5);
	}

	/* Session Items */
	.session-item {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px;
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.15s;
		text-align: left;
		width: 100%;
	}

	.session-item:hover {
		background: rgba(24, 24, 27, 0.5);
	}

	.session-item.active {
		background: rgba(37, 99, 235, 0.15);
	}

	.session-icon {
		color: #52525b;
		flex-shrink: 0;
	}

	.session-item.active .session-icon {
		color: #60a5fa;
	}

	.session-info {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.session-title {
		font-size: 12px;
		color: #a1a1aa;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.session-item.active .session-title,
	.session-item:hover .session-title {
		color: #e4e4e7;
	}

	.session-time {
		font-size: 10px;
		color: #3f3f46;
	}

	/* Footer */
	.sidebar-footer {
		padding: 8px;
		border-top: 1px solid rgba(39, 39, 42, 0.5);
	}

	.footer-btn {
		display: flex;
		align-items: center;
		gap: 8px;
		width: 100%;
		padding: 10px 12px;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #52525b;
		font-size: 12px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.footer-btn:hover {
		color: #a1a1aa;
		background: rgba(24, 24, 27, 0.5);
	}
</style>
