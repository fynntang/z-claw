<script lang="ts">
    import {getCurrentWindow} from '@tauri-apps/api/window';
    import {Bell, Minus, Moon, Settings, Square, Sun, X} from '@lucide/svelte';
    import {onMount} from 'svelte';
    import {showSettings} from '$lib/stores/app';
    import {mode as darkMode, toggleMode} from 'mode-watcher';
    import {Button} from "$lib/components/ui/button";


    const appWindow = getCurrentWindow();

    let isMaximized = $state(false);
    let hasNotification = $state(false);

    onMount(async () => {
        isMaximized = await appWindow.isMaximized();
        // 监听窗口状态变化
        const unlisten = await appWindow.onResized(async () => {
            isMaximized = await appWindow.isMaximized();
        });

        return () => {

            unlisten();
        };
    });

    async function minimize() {
        await appWindow.minimize();
    }

    async function toggleMaximize() {
        await appWindow.toggleMaximize();
    }

    async function close() {
        await appWindow.close();
    }

    function toggleNotification() {
        hasNotification = !hasNotification;
        // TODO: 打开通知面板
    }

    function openSettings() {
        showSettings.set(true);
    }
</script>

<svelte:head>
    <link rel="icon" href="/icon.ico" type="image/x-icon"/>
</svelte:head>

<div class="titlebar" data-tauri-drag-region>
    <div class="titlebar-drag" data-tauri-drag-region>
        <img src="/icon.ico" alt="ZClaw" class="app-icon"/>
        <span class="app-title">ZClaw</span>
    </div>

    <div class="titlebar-actions">
        <!-- 通知按钮 -->
        <button class="action-btn" onclick={toggleNotification} title="通知">
            <Bell size={16}/>
            {#if hasNotification}
                <span class="notification-dot"></span>
            {/if}
        </button>

        <!-- Dark Mode 切换 -->
        <Button onclick={toggleMode} class="action-btn" variant="outline" size="icon"
                title={darkMode.current === "dark" ? '切换到亮色模式' : '切换到暗色模式'}>
            <Sun class="h-[1.2rem] w-[1.2rem] scale-100 rotate-0 !transition-all dark:scale-0 dark:-rotate-90"
                 size={16}/>
            <Moon class="absolute h-[1.2rem] w-[1.2rem] scale-0 rotate-90 !transition-all dark:scale-100 dark:rotate-0"
                  size={16}/>
            <span class="sr-only">Toggle theme</span>
        </Button>

        <!-- 设置按钮 -->
        <button class="action-btn" onclick={openSettings} title="设置">
            <Settings size={16}/>
        </button>

        <div class="separator"></div>

        <!-- 窗口控制 -->
        <div class="titlebar-controls">
            <button class="control-btn minimize" onclick={minimize} title="最小化">
                <Minus size={16}/>
            </button>
            <button class="control-btn maximize" onclick={toggleMaximize} title={isMaximized ? '还原' : '最大化'}>
                <Square size={14}/>
            </button>
            <button class="control-btn close" onclick={close} title="关闭">
                <X size={16}/>
            </button>
        </div>
    </div>
</div>

<style>
    .titlebar {
        height: 32px;
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0 4px;
        user-select: none;
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        z-index: 9999;
        backdrop-filter: blur(12px);
        transition: background 0.2s ease, border-color 0.2s ease;
    }

    /* Dark mode - when html has 'dark' class */
    :global(html.dark) .titlebar {
        background: linear-gradient(180deg, rgba(24, 24, 27, 0.95) 0%, rgba(18, 18, 20, 0.98) 100%);
        border-bottom: 1px solid rgba(39, 39, 42, 0.5);
    }

    /* Light mode - when html does NOT have 'dark' class */
    :global(html:not(.dark)) .titlebar {
        background: linear-gradient(180deg, rgba(255, 255, 255, 0.98) 0%, rgba(250, 250, 250, 1) 100%);
        border-bottom: 1px solid rgba(228, 228, 231, 1);
    }

    .titlebar-drag {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 0 8px;
        flex: 1;
        height: 100%;
    }

    .app-icon {
        width: 18px;
        height: 18px;
        object-fit: contain;
    }

    .app-title {
        font-size: 12px;
        font-weight: 600;
        color: #a1a1aa;
        letter-spacing: 0.02em;
        transition: color 0.2s ease;
    }

    :global(html:not(.dark)) .app-title {
        color: #52525b;
    }

    .titlebar-actions {
        display: flex;
        align-items: center;
        height: 100%;
        gap: 2px;
    }

    .action-btn {
        appearance: none;
        padding: 0;
        margin: 0;
        border: none;
        display: inline-flex;
        justify-content: center;
        align-items: center;
        width: 36px;
        height: 28px;
        background-color: transparent;
        color: #71717a;
        cursor: pointer;
        transition: all 0.15s ease;
        border-radius: 6px;
        position: relative;
    }

    :global(html.dark) .action-btn:hover {
        background: rgba(63, 63, 70, 0.5);
        color: #e4e4e7;
    }

    :global(html:not(.dark)) .action-btn:hover {
        background: rgba(228, 228, 231, 1);
        color: #18181b;
    }

    .action-btn:active {
        transform: scale(0.95);
    }

    .notification-dot {
        position: absolute;
        top: 6px;
        right: 8px;
        width: 6px;
        height: 6px;
        border-radius: 50%;
        background: #ef4444;
        box-shadow: 0 0 4px rgba(239, 68, 68, 0.5);
    }

    .separator {
        width: 1px;
        height: 16px;
        background: rgba(63, 63, 70, 0.5);
        margin: 0 6px;
        transition: background 0.2s ease;
    }

    :global(html:not(.dark)) .separator {
        background: rgba(228, 228, 231, 1);
    }

    .titlebar-controls {
        display: flex;
        align-items: center;
        height: 100%;
    }

    .control-btn {
        appearance: none;
        padding: 0;
        margin: 0;
        border: none;
        display: inline-flex;
        justify-content: center;
        align-items: center;
        width: 46px;
        height: 32px;
        background-color: transparent;
        color: #71717a;
        cursor: pointer;
        transition: all 0.15s ease;
    }

    :global(html.dark) .control-btn:hover {
        background: rgba(63, 63, 70, 0.5);
        color: #e4e4e7;
    }

    :global(html:not(.dark)) .control-btn:hover {
        background: rgba(228, 228, 231, 1);
        color: #18181b;
    }

    .control-btn.close:hover {
        background: #dc2626;
        color: white;
    }

    .control-btn:active {
        transform: scale(0.95);
    }
</style>