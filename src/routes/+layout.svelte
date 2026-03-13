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
<ModeWatcher defaultMode="dark" themeColors={{ dark: '#070809', light: '#ffffff' }} />

<div class="zclaw-app flex h-screen w-full flex-col overflow-hidden">
    <TitleBar/>
    <main class="flex flex-1 overflow-hidden">
        {@render children()}
    </main>
</div>

<div style="display:none">
    {#each locales as locale (locale)}
        <a href={localizeHref(page.url.pathname, { locale })}>{locale}</a>
    {/each}
</div>