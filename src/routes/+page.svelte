<script lang="ts">
	import AgentSidebar from '$lib/components/AgentSidebar.svelte';
	import ChannelSidebar from '$lib/components/ChannelSidebar.svelte';
	import ChatArea from '$lib/components/ChatArea.svelte';
	import RightPanel from '$lib/components/RightPanel.svelte';
	import SettingsPanel from '$lib/components/SettingsPanel.svelte';

	let activeAgentId = $state(1);
	let activeListItem = $state('chat-1');
	let rightPanelTab = $state('agent');
	let showRightPanel = $state(true);
</script>

<div class="flex h-full w-full gap-3">
	<AgentSidebar bind:activeAgentId />
	<ChannelSidebar bind:activeListItem />
	<div class="flex-1 flex zclaw-surface overflow-hidden relative shadow-xl min-w-0">
		<ChatArea 
			bind:rightPanelTab 
			bind:showRightPanel 
			{activeAgentId} 
		/>
		<RightPanel 
			{showRightPanel} 
			{rightPanelTab}
			{activeAgentId}
			onToggle={() => { showRightPanel = !showRightPanel; }}
		/>
	</div>
	
	<SettingsPanel />
</div>
