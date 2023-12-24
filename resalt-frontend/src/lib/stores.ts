import constants from '$lib/constants';
import type Config from '$model/Config';
import Filter from '$model/Filter';
import Message from '$model/Message';
import type { MessageType } from '$model/MessageType';
import type RunResult from '$model/RunResult';
import type User from '$model/User';
import { persisted } from 'svelte-local-storage-store';
import { writable, type Readable, type Writable } from 'svelte/store';

const prefix = `${constants.appName.toLowerCase()}_`;

// First param is the local storage key.
// Second param is the initial value.

export const sidebarCollapsed = persisted(`${prefix}sidebarCollapsed`, false);

export const config: Writable<Config | null> = persisted(`${prefix}config`, null);
/**
 * @deprecated
 */
export const socketDeprecated = writable<{ connected: boolean; lastPing: Date | null }>({
	connected: false,
	lastPing: null,
});
export const theme = persisted(`${prefix}theme`, {
	color: 'primary',
});
export const currentUser: Writable<User | null> = persisted(`${prefix}currentUser`, null);

// Used for turning variables into pretty names in Header
export const replacementParams: Writable<Record<string, string>> = writable({});

// Used for Run tab
export const returns: Writable<RunResult[]> = writable([]);

// Used for Minion lists
export const filters: Writable<Filter[]> = writable([Filter.newEmpty()]);
filters.subscribe((filters) => {
	if (filters.length === 0) {
		filters.push(Filter.newEmpty());
	}
});

// Custom Store for toasts (of type Message) where we can add a toast,
// and they get automatically removed after a timeout.
interface ToastStore extends Readable<Message[]> {
	/**
	 * Add a toast to the store.
	 * @param {MessageType} type - The type of the toast (success, error, etc.)
	 * @param {string} title - The title of the toast, preferably short.
	 * @param {any} message - The message of the toast.
	 */
	// eslint-disable-next-line no-unused-vars
	add(this: void, type: MessageType, title: string, message: unknown): void;
	/**
	 * Clear all toasts from the store.
	 */
	// eslint-disable-next-line no-unused-vars
	clear(this: void): void;
}
function createToastStore(): ToastStore {
	const { subscribe, set, update } = writable<Message[]>([]);

	return {
		subscribe,
		add: (type: MessageType, title: string, message: unknown) => {
			const newToast = new Message(type, title, message);
			update((messages) => {
				messages.push(newToast);
				return messages;
			});
			setTimeout(() => {
				update((messages) => {
					// console.log('Checking toast age');
					// Check if all Messages are older than 5000ms + some change
					if (messages.every((m) => m.timestamp < Date.now() - 5000)) {
						// console.log('Cleaning up toasts');
						return [];
					} else {
						return messages;
					}
				});
			}, 6500);
		},
		clear: () => set([]),
	};
}

export const toasts: ToastStore = createToastStore();
