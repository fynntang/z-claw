<script lang="ts">
    import {agents, currentAgentId, showSettings} from '$lib/stores/app';
    import {Bot, Code, Cpu, Plus, Settings} from '@lucide/svelte';

    // Mock agents
    const mockAgents = [
        {id: '1', name: '通用助手', icon: Bot, status: 'running' as const, model: 'gpt-4o-mini', version: '0.1.0', skills: ['对话', '分析'], channels: []},
        {id: '2', name: '代码助手', icon: Cpu, status: 'stopped' as const, model: 'gpt-4o', version: '0.1.0', skills: ['代码', '调试'], channels: []},
        {id: '3', name: '数据分析', icon: Code, status: 'stopped' as const, model: 'claude-3-sonnet', version: '0.1.0', skills: ['数据分析', '可视化'], channels: []},
    ];

    $agents = mockAgents;

    function selectAgent(id: string) {
        currentAgentId.set(id);
    }

    function openSettings() {
        showSettings.set(true);
    }
</script>

<aside class="sidebar-nav">
    <div class="logo">
        <span class="logo-z">Z</span>
    </div>

    <nav class="agents-list">
        {#each $agents as agent}
            {@const AgentIcon = typeof agent.icon === 'string' ? Bot : agent.icon}
            <button
                    class="agent-btn {$currentAgentId === agent.id ? 'active' : ''}"
                    onclick={() => selectAgent(agent.id)}
                    title={agent.name}
            >
                <AgentIcon size={20}/>
                {#if agent.status === 'running'}
                    <span class="status-dot"></span>
                {/if}
            </button>
        {/each}

        <div class="divider"></div>

        <button class="agent-btn add" title="添加 Agent">
            <Plus size={20}/>
        </button>
    </nav>

    <div class="bottom-actions">
        <button class="settings-btn" onclick={openSettings}>
            <Settings size={24}/>
        </button>
    </div>
</aside>

<style>
    .sidebar-nav {
        width: 64px;
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 24px 0;
        border-right: 1px solid rgba(39, 39, 42, 0.5);
        background: rgba(0, 0, 0, 0.4);
        z-index: 50;
    }

    .logo {
        margin-bottom: 32px;
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
    }

    .agents-list {
        display: flex;
        flex-direction: column;
        gap: 16px;
        flex: 1;
        align-items: center;
    }

    .agent-btn {
        position: relative;
        width: 40px;
        height: 40px;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        background: #27272a;
        color: #71717a;
        border: none;
        cursor: pointer;
        transition: all 0.2s;
    }

    .agent-btn:hover {
        color: #e4e4e7;
        background: #3f3f46;
    }

    .agent-btn.active {
        background: #2563eb;
        color: white;
        box-shadow: 0 0 0 2px rgba(37, 99, 235, 0.2);
    }

    .agent-btn.add {
        border: 1px dashed #3f3f46;
        background: transparent;
    }

    .agent-btn.add:hover {
        border-color: #52525b;
        color: #a1a1aa;
    }

    .status-dot {
        position: absolute;
        bottom: 0;
        right: 0;
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
        margin: 8px 0;
    }

    .bottom-actions {
        margin-top: auto;
        display: flex;
        flex-direction: column;
        gap: 20px;
    }

    .settings-btn {
        background: transparent;
        border: none;
        color: #52525b;
        cursor: pointer;
        padding: 8px;
        border-radius: 8px;
        transition: all 0.2s;
    }

    .settings-btn:hover {
        color: white;
        background: #27272a;
    }
</style>