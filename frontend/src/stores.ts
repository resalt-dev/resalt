import { writable } from "svelte-local-storage-store";
import constants from "./constants";

const prefix = constants.appName.toLowerCase() + "_";

// First param is the local storage key.
// Second param is the initial value.

export const sidebarCollapsed = writable(prefix + "sidebarCollapsed", false);
export const auth = writable(prefix + "auth", null);
export const user = writable(prefix + "user", null);
export const minions = writable(prefix + "minions", null);
export const socket = writable(prefix + "socket", { connected: false, last_ping: null });

export const alerts = writable(prefix + "alerts", []);