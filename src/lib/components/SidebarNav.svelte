<script lang="ts">
	import {
		agents,
		currentAgentId,
		showSettings,
		sessions,
		currentSessionId,
		activeSidebarTab
	} from '$lib/stores/app';
	import { Bot, Plus, Settings, Hash, Calendar, MessageSquare } from '@lucide/svelte';
	import { generateId } from '$lib/stores/app';
	import ToolsPanel from './ToolsPanel.svelte';
	import LogsPanel from './LogsPanel.svelte';

	// Placeholder data for channels and tasks (to be replaced with backend data)
	const channels: { id: string; name: string; type: string; active: boolean }[] = [];
	const tasks: { id: string; name: string; time: string; active: boolean }[] = [];

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
	<!-- Main Content Area -->
	<div class="sidebar-content">
		{#if $activeSidebarTab === 'tools'}
			<!-- Tools Panel - Full Width -->
			<div class="panel-column full-width">
				<ToolsPanel />
			</div>
		{:else if $activeSidebarTab === 'logs'}
			<!-- Logs Panel - Full Width -->
			<div class="panel-column full-width">
				<LogsPanel />
			</div>
		{:else}
			<!-- Left Column: List -->
			<div class="list-column">
				<!-- Agents List -->
				{#if $activeSidebarTab === 'agents'}
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
				{:else if $activeSidebarTab === 'channels'}
					<div class="list-header">
						<span class="list-title">IM 频道</span>
						<button class="icon-btn" title="添加频道">
							<Plus size={14} />
						</button>
					</div>
					<div class="list-items">
						{#if channels.length === 0}
							<div class="empty-state">
								<span class="empty-text">暂无频道</span>
								<span class="empty-hint">点击 + 添加 IM 频道</span>
							</div>
						{:else}
							{#each channels as channel}
								<div class="list-item {channel.active ? 'active' : ''}">
									<Hash size={16} class="item-icon-simple" />
									<div class="item-info">
										<span class="item-name">{channel.name}</span>
										<span class="item-meta">{channel.type}</span>
									</div>
								</div>
							{/each}
						{/if}
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
						{#if tasks.length === 0}
							<div class="empty-state">
								<span class="empty-text">暂无任务</span>
								<span class="empty-hint">点击 + 添加定时任务</span>
							</div>
						{:else}
							{#each tasks as task}
								<div class="list-item {task.active ? 'active' : ''}">
									<div class="task-status {task.active ? 'running' : ''}"></div>
									<div class="item-info">
										<span class="item-name">{task.name}</span>
										<span class="item-meta">{task.time}</span>
									</div>
								</div>
							{/each}
						{/if}
					</div>
				{/if}
			</div>

			<!-- Right Column: Sessions (only for agents tab) -->
			{#if $activeSidebarTab === 'agents'}
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
			{/if}
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

	.panel-column.full-width {
		width: 100%;
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

	:global(.item-icon-simple) {
		color: #52525b;
		flex-shrink: 0;
	}

	.list-item.active :global(.item-icon-simple) {
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

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 24px 16px;
		text-align: center;
	}

	.empty-text {
		font-size: 12px;
		color: #52525b;
		margin-bottom: 4px;
	}

	.empty-hint {
		font-size: 10px;
		color: #3f3f46;
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

	:global(.session-icon) {
		color: #52525b;
		flex-shrink: 0;
	}

	.session-item.active :global(.session-icon) {
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
