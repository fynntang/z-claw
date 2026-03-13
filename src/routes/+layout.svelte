<script lang="ts">
    import './layout.css';
    import TitleBar from '$lib/components/TitleBar.svelte';
    import {ModeWatcher} from 'mode-watcher';
    import favicon from "$lib/assets/favicon.png";
    import {page} from "$app/state";
    import {locales, localizeHref} from '$lib/paraglide/runtime';


    const {children} = $props();
</script>

<svelte:head>
    <link rel="icon" href={favicon}/>
</svelte:head>
<ModeWatcher/>

<div class="app-wrapper">
    <TitleBar/>
    <main class="app-content">
        {@render children()}
    </main>
</div>
<div style="display:none">
    {#each locales as locale}
        <a href={localizeHref(page.url.pathname, { locale })}>{locale}</a>
    {/each}
</div>

<style>
    .app-wrapper {
        display: flex;
        flex-direction: column;
        height: 100vh;
        width: 100%;
        overflow: hidden;
    }

    .app-content {
        flex: 1;
        margin-top: 32px; /* TitleBar height */
        overflow: hidden;
    }
</style>