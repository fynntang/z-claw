<script lang="ts">
    import './layout.css';
    import { ModeWatcher } from 'mode-watcher';
    import favicon from '$lib/assets/favicon.png';
    import { page } from '$app/state';
    import { locales, localizeHref } from '$lib/paraglide/runtime';
    import { onMount } from 'svelte';
    import Titlebar from '$lib/components/Titlebar.svelte';

    const { children } = $props();

    onMount(() => {
    });
</script>

<svelte:head>
    <link rel="icon" href={favicon}/>
</svelte:head>
<ModeWatcher />

<div class="flex flex-col h-screen w-full overflow-hidden bg-background">
    <Titlebar />
    <div class="flex-1 w-full relative overflow-hidden">
        {@render children()}
    </div>
</div>

<div style="display:none">
    {#each locales as locale (locale)}
        <a href={localizeHref(page.url.pathname, { locale })}>{locale}</a>
    {/each}
</div>