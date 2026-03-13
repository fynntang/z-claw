import { writable } from 'svelte/store';
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

// Sidebar tab state
export type SidebarTab = 'agents' | 'channels' | 'tasks' | 'tools' | 'logs';
export const activeSidebarTab = writable<SidebarTab>('agents');

// Helper
export function generateId(): string {
	return Math.random().toString(36).substring(2, 9);
}

export function getCurrentTime(): string {
	return new Date().toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
}

/**
 * Load configuration from backend (ZeroClaw config file).
 * Should be called on app startup.
 */
export async function loadConfig(): Promise<void> {
	configLoading.set(true);
	try {
		const result = await invoke<AppConfig>('config_get');
		config.set(result);
		configLoaded.set(true);
	} catch (e) {
		console.error('Failed to load config:', e);
		// Use default config on error
		config.set(DEFAULT_CONFIG);
		configLoaded.set(true);
	} finally {
		configLoading.set(false);
	}
}

/**
 * Save configuration to backend (ZeroClaw config file).
 * @returns ValidationResult indicating success or validation errors
 */
export async function saveConfig(newConfig: AppConfig): Promise<ValidationResult> {
	try {
		await invoke('config_set', { config: newConfig });
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
	return await invoke<ValidationResult>('config_validate', { config: newConfig });
}
