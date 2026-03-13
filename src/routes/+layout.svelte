<script lang="ts">
    import './layout.css';
    import TitleBar from '$lib/components/TitleBar.svelte';
    import { ModeWatcher } from 'mode-watcher';
    import favicon from '$lib/assets/favicon.png';
    import { page } from '$app/state';
    import { locales, localizeHref } from '$lib/paraglide/runtime';
    import { onMount } from 'svelte';
    import { loadConfig } from '$lib/stores/app';

    const { children } = $props();

    onMount(() => {
        loadConfig();
    });
</script>

<svelte:head>
    <link rel="icon" href={favicon}/>
</svelte:head>
<ModeWatcher defaultMode="dark" themeColors={{ dark: '#09090b', light: '#ffffff' }} />

<div class="zclaw-app flex h-screen w-full flex-col font-sans selection:bg-[var(--app-accent)]/30 overflow-hidden">
    <!-- TitleBar floats at the top of the window -->
    <TitleBar/>
    
    <!-- Unified padding container for the inner content -->
    <main class="flex flex-1 overflow-hidden px-3 pb-3">
        {@render children()}
    </main>
</div>

<div style="display:none">
    {#each locales as locale (locale)}
        <a href={localizeHref(page.url.pathname, { locale })}>{locale}</a>
    {/each}
</div>