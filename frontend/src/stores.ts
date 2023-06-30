import { writable, type Readable, type Writable } from 'svelte/store';
import { persisted } from 'svelte-local-storage-store';
import constants from './constants';
import type User from './models/User';
import type Config from './models/Config';
import Message from './models/Message';
import type { MessageType } from './models/MessageType';
import type AuthToken from './models/AuthToken';

const prefix = `${constants.appName.toLowerCase()}_`;

// First param is the local storage key.
// Second param is the initial value.

export const sidebarCollapsed = persisted(`${prefix}sidebarCollapsed`, false);

export const auth: Writable<AuthToken | null> = persisted(`${prefix}auth`, null);
export const config: Writable<Config | null> = persisted(`${prefix}config`, null);
export const socket = writable({
	connected: false,
	lastPing: null,
});
export const theme = persisted(`${prefix}theme`, {
	color: null,
	dark: false,
});
export const currentUser: Writable<User | null> = persisted(
	`${prefix}currentUser`,
	null,
);

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
	add(this: void, type: MessageType, title: string, message: any): void;
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
		add: (type: MessageType, title: string, message: any) => {
			const newToast = new Message(type, title, message);
			update((messages) => {
				messages.push(newToast);
				return messages;
			});
			setTimeout(() => {
				update((messages) => {
					messages.shift();
					return messages;
				});
			}, 5000);
		},
		clear: () => set([]),
	};
}

export const toasts: ToastStore = createToastStore();
