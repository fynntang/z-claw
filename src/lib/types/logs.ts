/**
 * Logs types for Z-Claw desktop.
 * These types match the Rust LogEntry structure.
 */

/**
 * Log level.
 */
export type LogLevel = 'debug' | 'info' | 'warn' | 'error';

/**
 * Log entry for UI display.
 */
export interface LogEntry {
	/** Unique log ID */
	id: string;
	/** Timestamp string */
	timestamp: string;
	/** Log level */
	level: LogLevel;
	/** Source component */
	source: string;
	/** Log message */
	message: string;
}

/**
 * Log level configuration for filtering.
 */
export const LOG_LEVELS: { id: LogLevel; label: string; color: string }[] = [
	{ id: 'debug', label: 'Debug', color: 'text-gray-500' },
	{ id: 'info', label: 'Info', color: 'text-blue-500' },
	{ id: 'warn', label: 'Warn', color: 'text-yellow-500' },
	{ id: 'error', label: 'Error', color: 'text-red-500' }
];
