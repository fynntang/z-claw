import { writable } from 'svelte/store';
import type { Component } from 'svelte';
import { persisted } from 'svelte-local-storage-store';

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

// Config
export interface AppConfig {
  apiKey: string;
  apiUrl: string;
  model: string;
  localModelPath: string;
}

// Stores
export const messages = writable<Message[]>([]);
export const sessions = writable<Session[]>([]);
export const currentSessionId = writable<string>('');
export const agents = writable<Agent[]>([]);
export const currentAgentId = writable<string>('');
export const config = persisted<AppConfig>('zclaw-config', {
  apiKey: '',
  apiUrl: 'https://api.openai.com/v1',
  model: 'gpt-4o-mini',
  localModelPath: ''
});
export const showSettings = writable(false);
export const showAgentInfo = writable(false);

// Helper
export function generateId(): string {
  return Math.random().toString(36).substring(2, 9);
}

export function getCurrentTime(): string {
  return new Date().toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
}