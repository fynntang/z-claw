<script lang="ts">
    import './layout.css';
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
<ModeWatcher />

{@render children()}

<div style="display:none">
    {#each locales as locale (locale)}
        <a href={localizeHref(page.url.pathname, { locale })}>{locale}</a>
    {/each}
</div>