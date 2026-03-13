<script lang="ts">
	import { FileText, User, Paperclip, Sparkles, Send } from '@lucide/svelte';
	import { messages, currentSessionId, sendChat, generateId, getCurrentTime } from '$lib/stores/app';

	let {
		activeAgentId = 1,
		rightPanelTab = $bindable('agent'),
		showRightPanel = $bindable(true)
	} = $props();

	const agents = [
		{ id: 1, color: 'bg-blue-600', name: '苏东坡', icon: '🤖', description: 'AI coworker agent' },
		{ id: 2, color: 'bg-rose-600', name: '李白', icon: '🍷', description: '创意写作专家' },
		{ id: 3, color: 'bg-amber-600', name: '杜甫', icon: '✍️', description: '逻辑分析助手' },
		{ id: 4, color: 'bg-emerald-600', name: '王维', icon: '🌿', description: '视觉艺术顾问' },
	];

	let activeAgent = $derived(agents.find((a) => a.id === activeAgentId) || agents[0]);
	let inputText = $state('');
	let sending = $state(false);

	async function handleSend() {
		const text = inputText.trim();
		if (!text || sending) return;

		const userMsg = { id: generateId(), role: 'user' as const, content: text, time: getCurrentTime() };
		messages.update((m) => [...m, userMsg]);
		inputText = '';
		sending = true;

		try {
			const res = await sendChat(text);
			if (res.session_id) currentSessionId.set(res.session_id);
			const content = res.success ? res.content : `错误: ${res.error ?? '请求失败'}`;
			messages.update((m) => [...m, { id: generateId(), role: 'assistant' as const, content, time: getCurrentTime() }]);
		} catch (e) {
			messages.update((m) => [...m, { id: generateId(), role: 'assistant' as const, content: `请求异常: ${e}`, time: getCurrentTime() }]);
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

<main class="flex-1 flex flex-col min-h-0 bg-[var(--app-panel)]" role="main">
	<header class="h-12 shrink-0 flex items-center justify-between px-5 border-b border-[var(--app-divider)]">
		<div class="flex items-center gap-3">
			<div class="w-9 h-9 rounded-xl {activeAgent.color} flex items-center justify-center text-lg">{activeAgent.icon}</div>
			<span class="text-[15px] font-semibold text-[var(--app-text)] tracking-tight">{activeAgent.name}</span>
		</div>
		<div class="flex items-center gap-0.5">
			<button
				type="button"
				class="flex items-center gap-2 px-3 py-2 rounded-lg text-[13px] font-medium transition-colors duration-200 {showRightPanel && rightPanelTab === 'file' ? 'bg-[var(--app-accent-dim)] text-[var(--app-accent)]' : 'text-[var(--app-text-muted)] hover:text-[var(--app-text)] hover:bg-[var(--app-surface-hover)]'}"
				onclick={() => { rightPanelTab = 'file'; showRightPanel = true; }}
			>
				<FileText size={14} /> 文件
			</button>
			<button
				type="button"
				class="flex items-center gap-2 px-3 py-2 rounded-lg text-[13px] font-medium transition-colors duration-200 {showRightPanel && rightPanelTab === 'agent' ? 'bg-[var(--app-accent-dim)] text-[var(--app-accent)]' : 'text-[var(--app-text-muted)] hover:text-[var(--app-text)] hover:bg-[var(--app-surface-hover)]'}"
				onclick={() => { rightPanelTab = 'agent'; showRightPanel = !showRightPanel || rightPanelTab !== 'agent'; }}
			>
				<User size={14} /> Agent
			</button>
		</div>
	</header>

	<div class="flex-1 overflow-y-auto px-5 py-6 min-h-0 flex flex-col justify-end">
		{#if $messages.length === 0}
			<div class="flex-1 flex flex-col items-center justify-center gap-4 text-center py-16">
				<div class="w-14 h-14 rounded-2xl bg-[var(--app-surface)] flex items-center justify-center text-2xl border border-[var(--app-divider)]">{activeAgent.icon}</div>
				<p class="text-[var(--app-text)] text-[17px] font-medium tracking-tight">向 {activeAgent.name} 说点什么</p>
				<p class="text-[var(--app-text-muted)] text-[13px]">Enter 发送 · Shift+Enter 换行</p>
			</div>
		{/if}

		<div class="max-w-2xl mx-auto w-full space-y-5">
			{#each $messages as msg (msg.id)}
				<div class="flex gap-3 {msg.role === 'user' ? 'flex-row-reverse' : ''}">
					<div class="w-9 h-9 rounded-xl flex items-center justify-center shrink-0 text-base {msg.role === 'assistant' ? activeAgent.color : 'bg-[var(--app-accent)] text-white'}">
						{msg.role === 'assistant' ? activeAgent.icon : 'A'}
					</div>
					<div class="max-w-[82%] {msg.role === 'user' ? 'text-right' : ''}">
						<div
							class="inline-block rounded-2xl px-4 py-3 text-[14px] leading-relaxed {msg.role === 'assistant'
								? 'bg-[var(--app-surface)] text-[var(--app-text)] border-l-2 border-[var(--app-accent)]'
								: 'bg-[var(--app-accent)] text-white'}"
						>
							<span class="whitespace-pre-wrap">{msg.content}</span>
						</div>
					</div>
				</div>
			{/each}

			{#if sending}
				<div class="flex gap-3">
					<div class="w-9 h-9 rounded-xl shrink-0 flex items-center justify-center text-base {activeAgent.color}">{activeAgent.icon}</div>
					<div class="rounded-2xl px-4 py-3 text-[14px] text-[var(--app-text-muted)] italic bg-[var(--app-surface)] border-l-2 border-[var(--app-accent)]">正在回复...</div>
				</div>
			{/if}
		</div>
	</div>

	<div class="shrink-0 p-4 pt-3 border-t border-[var(--app-divider)]">
		<div class="max-w-2xl mx-auto flex gap-2 items-end rounded-2xl bg-[var(--app-surface)] border border-[var(--app-divider)] focus-within:border-[var(--app-accent)]/50 focus-within:ring-2 focus-within:ring-[var(--app-accent)]/20 transition-all duration-200 px-4 py-2.5">
			<textarea
				class="flex-1 min-h-[44px] max-h-32 py-2.5 bg-transparent border-none outline-none text-[15px] resize-none placeholder:text-[var(--app-text-muted)] text-[var(--app-text)] leading-relaxed"
				placeholder="发送消息..."
				rows="1"
				bind:value={inputText}
				onkeydown={onTextareaKeydown}
				disabled={sending}
			></textarea>
			<div class="flex items-center gap-0.5 shrink-0 pb-1">
				<button type="button" class="p-2.5 rounded-xl text-[var(--app-text-muted)] hover:text-[var(--app-text)] hover:bg-[var(--app-surface-hover)] transition-colors duration-200" title="附件"><Paperclip size={16} /></button>
				<button type="button" class="p-2.5 rounded-xl text-[var(--app-text-muted)] hover:text-[var(--app-text)] hover:bg-[var(--app-surface-hover)] transition-colors duration-200" title="扩展"><Sparkles size={16} /></button>
				<button
					type="button"
					class="p-2.5 rounded-xl bg-[var(--app-accent)] text-white hover:opacity-90 transition-opacity duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
					onclick={handleSend}
					disabled={sending}
					title="发送"
				>
					<Send size={16} />
				</button>
			</div>
		</div>
	</div>
</main>
