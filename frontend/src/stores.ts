import { writable as writableTemp, type Writable } from 'svelte/store';
import { writable } from 'svelte-local-storage-store';
import constants from './constants';
import type User from './models/User';
import type Config from './models/Config';
import type Alert from './models/Alert';

const prefix = `${constants.appName.toLowerCase()}_`;

// First param is the local storage key.
// Second param is the initial value.

export const sidebarCollapsed = writable(`${prefix}sidebarCollapsed`, false);

export const alerts: Writable<Alert[]> = writable(`${prefix}alerts`, []);
export const auth: Writable<string | null> = writable(`${prefix}auth`, null);
export const config: Writable<Config | null> = writable(`${prefix}config`, null);
export const socket = writableTemp({
    connected: false,
    last_ping: null,
});
export const theme = writable(`${prefix}theme`, {
    color: null,
    dark: false,
});
export const currentUser: Writable<User | null> = writable(`${prefix}currentUser`, null);
