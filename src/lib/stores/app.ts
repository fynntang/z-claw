import { get, writable } from 'svelte/store';
import type { Component } from 'svelte';
import { invoke } from '@tauri-apps/api/core';
import type { AppConfig, ValidationResult } from '$lib/types/config';
import { DEFAULT_CONFIG } from '$lib/types/config';

// Messages
export interface Message {
	id: string;
	role: 'user' | 'assistant' | 'system';
	content: string;
	time: string;
}

// Session
export interface Session {
	id: string;
	title: string;
	preview: string;
	time: string;
	messages: Message[];
}

// Agent
export interface Agent {
	id: string;
	name: string;
	icon: Component | string;
	status: 'running' | 'stopped' | 'error';
	model: string;
	version: string;
	skills: string[];
	channels: string[];
}

// Stores
export const messages = writable<Message[]>([]);
export const sessions = writable<Session[]>([]);
export const currentSessionId = writable<string>('');
export const agents = writable<Agent[]>([]);
export const currentAgentId = writable<string>('');

// Config store - NOT persisted to localStorage for security
// API key is stored encrypted in ~/.zeroclaw/config.toml
export const config = writable<AppConfig>(DEFAULT_CONFIG);
export const configLoaded = writable<boolean>(false);
export const configLoading = writable<boolean>(false);

export const showSettings = writable(false);
export const showAgentInfo = writable(false);

// Helper
export function generateId(): string {
	return Math.random().toString(36).substring(2, 9);
}

export function getCurrentTime(): string {
	return new Date().toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
}

/** 后端返回的 config 为 snake_case，转为前端 camelCase */
function normalizeBackendConfig(r: Record<string, unknown>): AppConfig {
	return {
		apiKey: (r.api_key as string) ?? (r.apiKey as string) ?? '',
		apiUrl: (r.api_url as string) ?? (r.apiUrl as string) ?? DEFAULT_CONFIG.apiUrl,
		provider: (r.provider as string) ?? DEFAULT_CONFIG.provider,
		model: (r.model as string) ?? DEFAULT_CONFIG.model,
		temperature: (r.temperature as number) ?? DEFAULT_CONFIG.temperature,
		localModelPath: (r.local_model_path as string) ?? (r.localModelPath as string) ?? undefined
	};
}

/**
 * Load configuration from backend (ZeroClaw config file).
 * Should be called on app startup.
 */
export async function loadConfig(): Promise<void> {
	configLoading.set(true);
	try {
		const result = await invoke<Record<string, unknown>>('config_get');
		config.set(normalizeBackendConfig(result));
		configLoaded.set(true);
	} catch (e) {
		console.error('Failed to load config:', e);
		config.set(DEFAULT_CONFIG);
		configLoaded.set(true);
	} finally {
		configLoading.set(false);
	}
}

/** 前端 camelCase 转为后端 snake_case */
function toBackendConfig(c: AppConfig): Record<string, unknown> {
	return {
		api_key: c.apiKey,
		api_url: c.apiUrl,
		provider: c.provider,
		model: c.model,
		temperature: c.temperature,
		local_model_path: c.localModelPath ?? null
	};
}

/**
 * Save configuration to backend (ZeroClaw config file).
 * @returns ValidationResult indicating success or validation errors
 */
export async function saveConfig(newConfig: AppConfig): Promise<ValidationResult> {
	try {
		await invoke('config_set', { config: toBackendConfig(newConfig) });
		config.set(newConfig);
		return { valid: true, errors: [] };
	} catch (e) {
		const errorMsg = String(e);
		return { valid: false, errors: [errorMsg] };
	}
}

/**
 * Validate configuration without saving.
 */
export async function validateConfig(newConfig: AppConfig): Promise<ValidationResult> {
	return await invoke<ValidationResult>('config_validate', {
		config: toBackendConfig(newConfig)
	});
}

/** Tauri chat 命令返回结构，与 Rust AgentResponse 一致（snake_case） */
export interface AgentResponse {
	content: string;
	success: boolean;
	error?: string;
	session_id?: string;
}

/**
 * 发送一条消息到当前会话，调用 Tauri chat 命令。
 * 使用 store 中的 config 作为 api_key/api_url/provider/model。
 * 若当前无 session_id 则传空，由后端生成并返回。
 */
export async function sendChat(message: string): Promise<AgentResponse> {
	const cfg = get(config);
	const sessionId = get(currentSessionId) || undefined;
	const payload = {
		message,
		session_id: sessionId || null,
		api_key: cfg.apiKey?.trim() || null,
		api_url: cfg.apiUrl?.trim() || null,
		provider: cfg.provider || null,
		model: cfg.model || null
	};
	const result = await invoke<AgentResponse>('chat', payload);
	return result;
}
