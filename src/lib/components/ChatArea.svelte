<script lang="ts">
    import { FileText, User, Paperclip, Sparkles, Send } from '@lucide/svelte';
    import {
        messages,
        currentSessionId,
        sendChat,
        generateId,
        getCurrentTime
    } from '$lib/stores/app';

    let {
        activeAgentId = 1,
        rightPanelTab = $bindable('agent'),
        showRightPanel = $bindable(true)
    } = $props();

    const agents = [
        { id: 1, color: 'bg-blue-600', name: '苏东坡', icon: '🤖', description: 'AI coworker agent' },
        { id: 2, color: 'bg-red-800', name: '李白', icon: '🍷', description: '创意写作专家' },
        { id: 3, color: 'bg-orange-600', name: '杜甫', icon: '✍️', description: '逻辑分析助手' },
        { id: 4, color: 'bg-emerald-700', name: '王维', icon: '🌿', description: '视觉艺术顾问' },
    ];

    let activeAgent = $derived(agents.find((a) => a.id === activeAgentId) || agents[0]);
    let inputText = $state('');
    let sending = $state(false);

    /** 发送消息：追加用户消息、调用 Tauri chat、追加助手回复或错误 */
    async function handleSend() {
        const text = inputText.trim();
        if (!text || sending) return;

        const userMsg = {
            id: generateId(),
            role: 'user' as const,
            content: text,
            time: getCurrentTime()
        };
        messages.update((m) => [...m, userMsg]);
        inputText = '';
        sending = true;

        try {
            const res = await sendChat(text);
            if (res.session_id) currentSessionId.set(res.session_id);
            const content = res.success
                ? res.content
                : `错误: ${res.error ?? '请求失败'}`;
            const assistantMsg = {
                id: generateId(),
                role: 'assistant' as const,
                content,
                time: getCurrentTime()
            };
            messages.update((m) => [...m, assistantMsg]);
        } catch (e) {
            const errMsg = {
                id: generateId(),
                role: 'assistant' as const,
                content: `请求异常: ${e}`,
                time: getCurrentTime()
            };
            messages.update((m) => [...m, errMsg]);
        } finally {
            sending = false;
        }
    }

    function onTextareaKeydown(e: KeyboardEvent) {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            handleSend();
        }
    }
</script>

<main class="chat-area flex-1 flex flex-col relative z-10 bg-[var(--app-surface-elevated)] overflow-hidden">
    <header class="h-13 shrink-0 flex items-center justify-between px-5 border-b border-[var(--app-border)]">
        <div class="flex items-center gap-3">
            <div class="w-8 h-8 rounded-xl {activeAgent.color} flex items-center justify-center text-base shadow-sm">{activeAgent.icon}</div>
            <span class="font-semibold text-[15px] text-[var(--app-text)] tracking-tight">{activeAgent.name}</span>
        </div>
        <div class="flex items-center gap-1">
            <button
                onclick={() => { rightPanelTab = 'file'; showRightPanel = true; }}
                class="flex items-center gap-2 px-3 py-2 rounded-lg text-[13px] font-medium transition-colors focus:outline-none {showRightPanel && rightPanelTab === 'file' ? 'bg-[var(--app-accent-soft)] text-[var(--app-accent)]' : 'text-[var(--app-text-muted)] hover:bg-[var(--app-surface-hover)] hover:text-[var(--app-text-secondary)]'}"
            >
                <FileText size={14} /> 文件
            </button>
            <button
                onclick={() => { rightPanelTab = 'agent'; showRightPanel = (!showRightPanel || rightPanelTab !== 'agent'); }}
                class="flex items-center gap-2 px-3 py-2 rounded-lg text-[13px] font-medium transition-colors focus:outline-none {showRightPanel && rightPanelTab === 'agent' ? 'bg-[var(--app-accent-soft)] text-[var(--app-accent)]' : 'text-[var(--app-text-muted)] hover:bg-[var(--app-surface-hover)] hover:text-[var(--app-text-secondary)]'}"
            >
                <User size={14} /> Agent
            </button>
        </div>
    </header>

    <div class="flex-1 overflow-y-auto px-5 py-6 space-y-5 custom-scrollbar flex flex-col justify-end">
        {#if $messages.length === 0}
            <div class="chat-empty w-full flex-1 flex flex-col items-center justify-center gap-4 py-12">
                <div class="w-14 h-14 rounded-2xl bg-[var(--app-accent-soft)] flex items-center justify-center text-2xl">{activeAgent.icon}</div>
                <p class="text-[var(--app-text-secondary)] text-[15px]">向 {activeAgent.name} 发送第一条消息</p>
                <p class="text-[var(--app-text-muted)] text-[13px]">支持 Enter 发送，Shift+Enter 换行</p>
            </div>
        {/if}
        {#each $messages as msg (msg.id)}
            <div class="flex gap-3 {msg.role === 'user' ? 'flex-row-reverse' : ''}">
                <div class="w-8 h-8 rounded-xl flex items-center justify-center shrink-0 text-sm {msg.role === 'assistant' ? activeAgent.color : 'bg-[var(--app-accent)] text-[var(--app-bg)]'} shadow-sm">
                    {msg.role === 'assistant' ? activeAgent.icon : 'A'}
                </div>
                <div class="max-w-[78%] flex flex-col {msg.role === 'user' ? 'items-end' : ''}">
                    <div class="rounded-2xl px-4 py-3 text-[14px] leading-relaxed {
                        msg.role === 'assistant'
                        ? 'bg-[var(--app-surface)] border border-[var(--app-border)] text-[var(--app-text)]'
                        : 'bg-[var(--app-accent)] text-[var(--app-bg)]'
                    }">
                        <pre class="whitespace-pre-wrap font-sans">{msg.content}</pre>
                    </div>
                </div>
            </div>
        {/each}
        {#if sending}
            <div class="flex gap-3">
                <div class="w-8 h-8 rounded-xl flex items-center justify-center shrink-0 text-sm {activeAgent.color} shadow-sm">{activeAgent.icon}</div>
                <div class="rounded-2xl px-4 py-2.5 text-[14px] text-[var(--app-text-muted)] italic bg-[var(--app-surface)] border border-[var(--app-border)]">正在回复...</div>
            </div>
        {/if}
    </div>

    <div class="px-5 pb-5 pt-3 shrink-0 border-t border-[var(--app-border)]">
        <div class="max-w-3xl mx-auto">
            <div class="chat-input-wrap rounded-2xl px-4 py-3 flex flex-col gap-2 bg-[var(--app-surface)] border border-[var(--app-border)] focus-within:border-[var(--app-border-focus)] focus-within:ring-1 focus-within:ring-[var(--app-border-focus)] transition-all">
                <textarea
                    class="w-full bg-transparent border-none outline-none text-[15px] resize-none placeholder:text-[var(--app-text-muted)] text-[var(--app-text)]"
                    placeholder="给 {activeAgent.name} 发送消息..."
                    rows="2"
                    bind:value={inputText}
                    onkeydown={onTextareaKeydown}
                    disabled={sending}
                ></textarea>
                <div class="flex justify-between items-center">
                    <div class="flex gap-1">
                        <button type="button" class="w-8 h-8 flex items-center justify-center rounded-lg text-[var(--app-text-muted)] hover:text-[var(--app-accent)] hover:bg-[var(--app-surface-hover)] transition-colors focus:outline-none"><Paperclip size={16}/></button>
                        <button type="button" class="w-8 h-8 flex items-center justify-center rounded-lg text-[var(--app-text-muted)] hover:text-[var(--app-accent)] hover:bg-[var(--app-surface-hover)] transition-colors focus:outline-none"><Sparkles size={16}/></button>
                    </div>
                    <button
                        type="button"
                        class="w-9 h-9 flex items-center justify-center rounded-xl bg-[var(--app-accent)] text-[var(--app-bg)] hover:opacity-90 transition-opacity active:scale-95 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
                        onclick={handleSend}
                        disabled={sending}
                    >
                        <Send size={15} class="-ml-0.5" />
                    </button>
                </div>
            </div>
        </div>
    </div>
</main>

<style>
    .chat-area { border-radius: 0; }
    .custom-scrollbar::-webkit-scrollbar { width: 5px; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--app-border); border-radius: 10px; }
    textarea { background: transparent; }
</style>
