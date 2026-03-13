<script lang="ts">
  import SidebarNav from '$lib/components/SidebarNav.svelte';
  import SidebarSecondary from '$lib/components/SidebarSecondary.svelte';
  import ChatArea from '$lib/components/ChatArea.svelte';
  import SettingsPanel from '$lib/components/SettingsPanel.svelte';
</script>

<div class="app-container">
  <SidebarNav />
  <SidebarSecondary />
  <ChatArea />
  <SettingsPanel />
</div>

<style>
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(body) {
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background-color: #0c0c0e;
    color: #d4d4d8;
    overflow: hidden;
  }

  :global(.mono) {
    font-family: 'JetBrains Mono', monospace;
  }

  .app-container {
    display: flex;
    height: 100vh;
    width: 100%;
    overflow: hidden;
  }
</style>