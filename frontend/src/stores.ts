import { writable, type Writable } from 'svelte/store';
import { writable as writableLocalStorage } from 'svelte-local-storage-store';
import constants from './constants';
import type User from './models/User';
import type Config from './models/Config';
import type Alert from './models/Alert';

const prefix = `${constants.appName.toLowerCase()}_`;

// First param is the local storage key.
// Second param is the initial value.

export const sidebarCollapsed = writableLocalStorage(`${prefix}sidebarCollapsed`, false);

export const toasts: Writable<Alert[]> = writable([]);
export const auth: Writable<string | null> = writableLocalStorage(`${prefix}auth`, null);
export const config: Writable<Config | null> = writableLocalStorage(`${prefix}config`, null);
export const socket = writable({
    connected: false,
    last_ping: null,
});
export const theme = writableLocalStorage(`${prefix}theme`, {
    color: null,
    dark: false,
});
export const currentUser: Writable<User | null> = writableLocalStorage(`${prefix}currentUser`, null);
