import { writable, type Readable, type Writable } from 'svelte/store';
import { writable as writableLocalStorage } from 'svelte-local-storage-store';
import constants from './constants';
import type User from './models/User';
import type Config from './models/Config';
import type Message from './models/Message';

const prefix = `${constants.appName.toLowerCase()}_`;

// First param is the local storage key.
// Second param is the initial value.

export const sidebarCollapsed = writableLocalStorage(
    `${prefix}sidebarCollapsed`,
    false,
);

export const auth: Writable<string | null> = writableLocalStorage(
    `${prefix}auth`,
    null,
);
export const config: Writable<Config | null> = writableLocalStorage(
    `${prefix}config`,
    null,
);
export const socket = writable({
    connected: false,
    last_ping: null,
});
export const theme = writableLocalStorage(`${prefix}theme`, {
    color: null,
    dark: false,
});
export const currentUser: Writable<User | null> = writableLocalStorage(
    `${prefix}currentUser`,
    null,
);

// Custom Store for toasts (of type Message) where we can add a toast,
// and they get automatically removed after a timeout.
interface ToastStore extends Readable<Message[]> {
    /**
     * Add a toast to the store.
     * @param message The message to add.
     */
    // eslint-disable-next-line no-unused-vars
    add(this: void, message: Message): void;
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
        add: (message: Message) => {
            update((messages) => {
                messages.push(message);
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
