/**
 * Configuration types for Z-Claw desktop.
 * These types match the Rust AppConfig structure.
 */

/**
 * Application configuration.
 * This is the contract between frontend and backend.
 */
export interface AppConfig {
	/** API key (decrypted when returned from backend) */
	apiKey: string;
	/** Custom API endpoint URL */
	apiUrl: string;
	/** Provider name (e.g., "openai", "anthropic", "openrouter") */
	provider: string;
	/** Default model name */
	model: string;
	/** Temperature parameter (0.0 - 2.0) */
	temperature: number;
	/** Local model path (optional, for local inference) */
	localModelPath?: string;
}

/**
 * Configuration validation result.
 */
export interface ValidationResult {
	valid: boolean;
	errors: string[];
}

/**
 * Default configuration values.
 */
export const DEFAULT_CONFIG: AppConfig = {
	apiKey: '',
	apiUrl: 'https://api.openai.com/v1',
	provider: 'openai',
	model: 'gpt-4o-mini',
	temperature: 0.7,
	localModelPath: undefined
};

/**
 * Provider definition.
 */
export interface Provider {
	id: string;
	name: string;
	models: string[];
}

/**
 * Available providers for the model selector.
 */
export const AVAILABLE_PROVIDERS: Provider[] = [
	{
		id: 'openai',
		name: 'OpenAI',
		models: ['gpt-4o-mini', 'gpt-4o', 'gpt-4-turbo', 'gpt-3.5-turbo']
	},
	{
		id: 'anthropic',
		name: 'Anthropic',
		models: ['claude-3-5-sonnet-20241022', 'claude-3-opus-20240229', 'claude-3-haiku-20240307']
	},
	{
		id: 'openrouter',
		name: 'OpenRouter',
		models: ['openrouter/auto', 'anthropic/claude-sonnet-4', 'openai/gpt-4o']
	},
	{ id: 'deepseek', name: 'DeepSeek', models: ['deepseek-chat', 'deepseek-coder'] },
	{ id: 'ollama', name: 'Ollama (Local)', models: ['llama3.2', 'llama3.1', 'mistral', 'codellama'] }
];

/**
 * Provider type.
 */
export type ProviderId = (typeof AVAILABLE_PROVIDERS)[number]['id'];
