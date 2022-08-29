import { writable as writableTemp } from 'svelte/store';
import { writable } from 'svelte-local-storage-store';
import constants from './constants';

const prefix = `${constants.appName.toLowerCase()}_`;

// First param is the local storage key.
// Second param is the initial value.

export const sidebarCollapsed = writable(`${prefix}sidebarCollapsed`, false);

export const alerts = writable(`${prefix}alerts`, []);
export const auth = writable(`${prefix}auth`, null);
export const config = writable(`${prefix}config`, null);
export const socket = writableTemp({
    connected: false,
    last_ping: null,
});
export const theme = writable(`${prefix}theme`, {
    color: null,
    dark: false,
});
export const currentUser = writable(`${prefix}currentUser`, null);
