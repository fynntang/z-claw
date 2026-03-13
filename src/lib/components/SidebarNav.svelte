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
		ChevronLeft,
		Calendar,
		Hash,
		MessageSquare
	} from '@lucide/svelte';
	import { generateId } from '$lib/stores/app';

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

	// Panel state
	let expanded = $state(false);
	let activePanel = $state<'sessions' | 'tasks' | 'channels'>('sessions');

	// Mock sessions
	const mockSessions = [
		{ id: '1', title: '系统架构分析', preview: '', time: '12:45', messages: [] },
		{ id: '2', title: 'IM 机器人联调', preview: '', time: '昨天', messages: [] },
		{ id: '3', title: '数据迁移方案', preview: '', time: '3天前', messages: [] }
	];

	$sessions = mockSessions;

	// Mock tasks
	const mockTasks = [
		{ id: '1', name: '每日代码清理', time: '09:00', active: true },
		{ id: '2', name: '周五报表导出', time: 'Paused', active: false }
	];

	// Mock channels
	const mockChannels = [
		{ id: '1', name: '# dev-ops', type: 'slack', active: true },
		{ id: '2', name: 'Alpha-Group', type: 'telegram', active: false }
	];

	function selectAgent(id: string) {
		currentAgentId.set(id);
	}

	function openSettings() {
		showSettings.set(true);
	}

	function togglePanel() {
		expanded = !expanded;
	}

	function selectSession(id: string) {
		currentSessionId.set(id);
	}

	function newSession() {
		const id = generateId();
		$sessions = [{ id, title: '新对话', preview: '', time: '现在', messages: [] }, ...$sessions];
		currentSessionId.set(id);
	}
</script>

<aside class="sidebar-nav" class:expanded>
	<!-- Main Icon Column -->
	<div class="nav-column">
		<!-- Logo -->
		<button class="logo-btn" onclick={togglePanel} title={expanded ? '收起面板' : '展开面板'}>
			<span class="logo-z">Z</span>
		</button>

		<!-- Agents -->
		<nav class="agents-list">
			{#each $agents as agent}
				{@const AgentIcon = typeof agent.icon === 'string' ? Bot : agent.icon}
				<button
					class="agent-btn {$currentAgentId === agent.id ? 'active' : ''}"
					onclick={() => selectAgent(agent.id)}
					title={agent.name}
				>
					<AgentIcon size={20} />
					{#if agent.status === 'running'}
						<span class="status-dot"></span>
					{/if}
				</button>
			{/each}

			<div class="divider"></div>

			<button class="agent-btn add" title="添加 Agent">
				<Plus size={20} />
			</button>
		</nav>

		<!-- Quick Actions -->
		<div class="quick-actions">
			<button
				class="action-btn {activePanel === 'sessions' && expanded ? 'active' : ''}"
				onclick={() => {
					activePanel = 'sessions';
					if (!expanded) expanded = true;
				}}
				title="会话"
			>
				<MessageSquare size={20} />
			</button>
			<button
				class="action-btn {activePanel === 'tasks' && expanded ? 'active' : ''}"
				onclick={() => {
					activePanel = 'tasks';
					if (!expanded) expanded = true;
				}}
				title="定时任务"
			>
				<Calendar size={20} />
			</button>
			<button
				class="action-btn {activePanel === 'channels' && expanded ? 'active' : ''}"
				onclick={() => {
					activePanel = 'channels';
					if (!expanded) expanded = true;
				}}
				title="IM 频道"
			>
				<Hash size={20} />
			</button>
		</div>

		<!-- Bottom Actions -->
		<div class="bottom-actions">
			<button class="settings-btn" onclick={openSettings} title="设置">
				<Settings size={20} />
			</button>
		</div>
	</div>

	<!-- Expandable Panel -->
	{#if expanded}
		<div class="expand-panel">
			<div class="panel-header">
				<h3>
					{#if activePanel === 'sessions'}
						会话
					{:else if activePanel === 'tasks'}
						定时任务
					{:else}
						IM 频道
					{/if}
				</h3>
				<button class="collapse-btn" onclick={togglePanel}>
					<ChevronLeft size={16} />
				</button>
			</div>

			<div class="panel-content">
				<!-- Sessions Panel -->
				{#if activePanel === 'sessions'}
					<button class="new-session-btn" onclick={newSession}>
						<Plus size={14} />
						新建会话
					</button>
					<div class="items-list">
						{#each $sessions as session}
							<button
								class="item-btn {$currentSessionId === session.id ? 'active' : ''}"
								onclick={() => selectSession(session.id)}
							>
								<span class="item-title">{session.title}</span>
								<span class="item-time">{session.time}</span>
							</button>
						{/each}
					</div>

					<!-- Tasks Panel -->
				{:else if activePanel === 'tasks'}
					<div class="items-list">
						{#each mockTasks as task}
							<div class="task-item">
								<div class="task-status {task.active ? 'active' : ''}"></div>
								<span class="item-title">{task.name}</span>
								<span class="item-time">{task.time}</span>
							</div>
						{/each}
					</div>
					<div class="empty-hint">
						<button class="add-btn">
							<Plus size={12} /> 添加任务
						</button>
					</div>

					<!-- Channels Panel -->
				{:else}
					<div class="items-list">
						{#each mockChannels as channel}
							<div class="channel-item {channel.active ? 'active' : ''}">
								<span class="channel-type">{channel.type}</span>
								<span class="item-title">{channel.name}</span>
							</div>
						{/each}
					</div>
					<div class="empty-hint">
						<button class="add-btn">
							<Plus size={12} /> 添加频道
						</button>
					</div>
				{/if}
			</div>
		</div>
	{/if}
</aside>

<style>
	.sidebar-nav {
		display: flex;
		flex-direction: row;
		border-right: 1px solid rgba(39, 39, 42, 0.5);
		background: rgba(0, 0, 0, 0.4);
		z-index: 50;
		transition: width 0.2s ease;
	}

	.nav-column {
		width: 64px;
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 16px 0;
		flex-shrink: 0;
	}

	.logo-btn {
		background: transparent;
		border: none;
		cursor: pointer;
		padding: 0;
		margin-bottom: 24px;
	}

	.logo-z {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		border-radius: 12px;
		background: linear-gradient(135deg, #2563eb 0%, #7c3aed 100%);
		color: white;
		font-size: 20px;
		font-weight: bold;
		box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
		transition:
			transform 0.15s,
			box-shadow 0.15s;
	}

	.logo-btn:hover .logo-z {
		transform: scale(1.05);
		box-shadow: 0 6px 16px rgba(37, 99, 235, 0.4);
	}

	.agents-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
		align-items: center;
	}

	.agent-btn {
		position: relative;
		width: 40px;
		height: 40px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #18181b;
		color: #71717a;
		border: 1px solid transparent;
		cursor: pointer;
		transition: all 0.15s;
	}

	.agent-btn:hover {
		color: #e4e4e7;
		background: #27272a;
		border-color: #3f3f46;
	}

	.agent-btn.active {
		background: linear-gradient(135deg, #2563eb 0%, #7c3aed 100%);
		color: white;
		border-color: transparent;
		box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
	}

	.agent-btn.add {
		border: 1px dashed #3f3f46;
		background: transparent;
	}

	.agent-btn.add:hover {
		border-color: #52525b;
		color: #a1a1aa;
		background: rgba(24, 24, 27, 0.5);
	}

	.status-dot {
		position: absolute;
		bottom: -2px;
		right: -2px;
		width: 10px;
		height: 10px;
		border-radius: 50%;
		background: #10b981;
		border: 2px solid #0c0c0e;
	}

	.divider {
		width: 32px;
		height: 1px;
		background: #27272a;
		margin: 4px 0;
	}

	.quick-actions {
		display: flex;
		flex-direction: column;
		gap: 8px;
		margin-top: 16px;
		padding-top: 16px;
		border-top: 1px solid rgba(39, 39, 42, 0.5);
	}

	.action-btn {
		width: 40px;
		height: 40px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		color: #52525b;
		border: none;
		cursor: pointer;
		transition: all 0.15s;
	}

	.action-btn:hover {
		color: #a1a1aa;
		background: rgba(24, 24, 27, 0.5);
	}

	.action-btn.active {
		color: #60a5fa;
		background: rgba(37, 99, 235, 0.1);
	}

	.bottom-actions {
		margin-top: auto;
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.settings-btn {
		background: transparent;
		border: none;
		color: #52525b;
		cursor: pointer;
		padding: 8px;
		border-radius: 12px;
		transition: all 0.15s;
	}

	.settings-btn:hover {
		color: white;
		background: rgba(24, 24, 27, 0.5);
	}

	/* Expandable Panel */
	.expand-panel {
		width: 240px;
		display: flex;
		flex-direction: column;
		border-left: 1px solid rgba(39, 39, 42, 0.3);
		background: rgba(12, 12, 14, 0.5);
		animation: slideIn 0.2s ease;
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateX(-8px);
		}
		to {
			opacity: 1;
			transform: translateX(0);
		}
	}

	.panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px;
		border-bottom: 1px solid rgba(39, 39, 42, 0.5);
	}

	.panel-header h3 {
		font-size: 12px;
		font-weight: 600;
		color: #d4d4d8;
		letter-spacing: 0.02em;
	}

	.collapse-btn {
		background: transparent;
		border: none;
		color: #52525b;
		cursor: pointer;
		padding: 4px;
		border-radius: 6px;
		transition: all 0.15s;
	}

	.collapse-btn:hover {
		color: #a1a1aa;
		background: rgba(24, 24, 27, 0.5);
	}

	.panel-content {
		flex: 1;
		overflow-y: auto;
		padding: 12px;
	}

	.panel-content::-webkit-scrollbar {
		width: 4px;
	}

	.panel-content::-webkit-scrollbar-thumb {
		background: #27272a;
		border-radius: 10px;
	}

	.new-session-btn {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		padding: 10px;
		background: linear-gradient(135deg, #2563eb 0%, #7c3aed 100%);
		border: none;
		border-radius: 8px;
		color: white;
		font-size: 12px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
		margin-bottom: 12px;
	}

	.new-session-btn:hover {
		box-shadow: 0 4px 12px rgba(37, 99, 235, 0.4);
		transform: translateY(-1px);
	}

	.items-list {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.item-btn {
		width: 100%;
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		gap: 4px;
		padding: 10px 12px;
		background: transparent;
		border: 1px solid transparent;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.15s;
		text-align: left;
	}

	.item-btn:hover {
		background: rgba(24, 24, 27, 0.5);
		border-color: rgba(63, 63, 70, 0.3);
	}

	.item-btn.active {
		background: rgba(37, 99, 235, 0.1);
		border-color: rgba(59, 130, 246, 0.3);
	}

	.item-title {
		font-size: 12px;
		color: #a1a1aa;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		width: 100%;
	}

	.item-btn.active .item-title,
	.item-btn:hover .item-title {
		color: #e4e4e7;
	}

	.item-time {
		font-size: 10px;
		color: #52525b;
	}

	/* Task items */
	.task-item {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 10px 12px;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.task-item:hover {
		background: rgba(24, 24, 27, 0.5);
	}

	.task-status {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #3f3f46;
		flex-shrink: 0;
	}

	.task-status.active {
		background: #10b981;
		box-shadow: 0 0 8px rgba(16, 185, 129, 0.5);
	}

	/* Channel items */
	.channel-item {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 10px 12px;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.channel-item:hover {
		background: rgba(24, 24, 27, 0.5);
	}

	.channel-item.active {
		background: rgba(37, 99, 235, 0.1);
	}

	.channel-type {
		font-size: 9px;
		font-weight: 600;
		text-transform: uppercase;
		color: #52525b;
		background: #18181b;
		padding: 2px 6px;
		border-radius: 4px;
	}

	.empty-hint {
		padding: 16px;
		text-align: center;
	}

	.add-btn {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		background: transparent;
		border: 1px dashed #3f3f46;
		color: #52525b;
		font-size: 11px;
		padding: 8px 16px;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.add-btn:hover {
		border-color: #52525b;
		color: #a1a1aa;
	}
</style>
