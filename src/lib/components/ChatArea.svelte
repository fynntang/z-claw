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

<main class="flex-1 flex flex-col relative z-10 bg-[#141417] overflow-hidden rounded-[20px]">
    
    <!-- 顶部信息栏 -->
    <header class="h-14 border-b border-white/5 flex items-center justify-between px-6 z-30 shrink-0">
        <div class="flex items-center gap-3">
            <div class="w-7 h-7 rounded-md bg-white/10 flex items-center justify-center text-sm">{activeAgent.icon}</div>
            <span class="font-bold text-[15px] text-zinc-100 tracking-wide">{activeAgent.name}</span>
        </div>

        <div class="flex items-center gap-2">
            <button
                onclick={() => { rightPanelTab = 'file'; showRightPanel = true; }}
                class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-medium transition-all focus:outline-none {showRightPanel && rightPanelTab === 'file' ? 'bg-zinc-800 text-white shadow-sm' : 'text-zinc-400 hover:bg-white/5 hover:text-zinc-200'}"
            >
                <FileText size={14} /> 文件
            </button>
            <button
                onclick={() => { rightPanelTab = 'agent'; showRightPanel = (!showRightPanel || rightPanelTab !== 'agent'); }}
                class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-medium transition-all focus:outline-none {showRightPanel && rightPanelTab === 'agent' ? 'bg-zinc-800 text-white shadow-sm' : 'text-zinc-400 hover:bg-white/5 hover:text-zinc-200'}"
            >
                <User size={14} /> Agent
            </button>
        </div>
    </header>

    <!-- 消息区域 -->
    <div class="flex-1 overflow-y-auto p-6 space-y-6 custom-scrollbar flex flex-col justify-end">
        {#if $messages.length === 0}
            <div class="w-full h-full flex items-center justify-center text-zinc-600 text-sm italic opacity-50">
                等待输入中...
            </div>
        {/if}
        {#each $messages as msg (msg.id)}
            <div class="flex gap-4 {msg.role === 'user' ? 'flex-row-reverse' : ''}">
                <div class="w-8 h-8 rounded-full flex items-center justify-center shrink-0 text-sm shadow-md {msg.role === 'assistant' ? activeAgent.color : 'bg-orange-600 text-white'}">
                    {msg.role === 'assistant' ? activeAgent.icon : 'A'}
                </div>
                <div class="max-w-[80%] flex flex-col {msg.role === 'user' ? 'items-end' : ''}">
                    <div class="rounded-[20px] p-4 text-[14px] leading-relaxed {
                        msg.role === 'assistant'
                        ? 'bg-[#1c1c1f] border border-white/5 text-zinc-200 shadow-xl'
                        : 'bg-orange-600 text-white shadow-lg shadow-orange-900/20'
                    }">
                        <pre class="whitespace-pre-wrap font-sans font-medium">{msg.content}</pre>
                    </div>
                </div>
            </div>
        {/each}
        {#if sending}
            <div class="flex gap-4">
                <div class="w-8 h-8 rounded-full flex items-center justify-center shrink-0 text-sm shadow-md {activeAgent.color}">
                    {activeAgent.icon}
                </div>
                <div class="rounded-[20px] px-4 py-2 text-[14px] text-zinc-500 italic">正在回复...</div>
            </div>
        {/if}
    </div>

    <!-- 输入框区域 -->
    <div class="px-6 pb-6 pt-2 shrink-0 border-t border-transparent">
        <div class="max-w-4xl mx-auto">
            <div class="bg-[#1c1c1f] border border-white/10 rounded-[24px] p-4 py-3 shadow-2xl focus-within:border-zinc-600 transition-all flex flex-col gap-2">
                <textarea
                    class="w-full bg-transparent border-none outline-none text-[15px] resize-none placeholder:text-zinc-600 text-zinc-200"
                    placeholder="给 {activeAgent.name} 发送消息..."
                    rows="2"
                    bind:value={inputText}
                    onkeydown={onTextareaKeydown}
                    disabled={sending}
                ></textarea>
                <div class="flex justify-between items-center mt-2">
                    <div class="flex gap-4 px-1">
                        <button type="button" class="text-zinc-500 hover:text-zinc-300 transition-colors focus:outline-none"><Paperclip size={18}/></button>
                        <button type="button" class="text-zinc-500 hover:text-zinc-300 transition-colors focus:outline-none"><Sparkles size={18}/></button>
                    </div>
                    <button
                        type="button"
                        class="flex items-center justify-center w-8 h-8 bg-zinc-700 hover:bg-zinc-600 text-white rounded-full shadow-lg transition-all active:scale-95 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
                        onclick={handleSend}
                        disabled={sending}
                    >
                        <Send size={14} class="-ml-0.5" />
                    </button>
                </div>
            </div>
        </div>
    </div>
</main>

<style>
    .custom-scrollbar::-webkit-scrollbar {
        width: 6px;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb {
        background: #3f3f46;
        border-radius: 6px;
    }
    textarea {
        background: transparent;
    }
</style>
