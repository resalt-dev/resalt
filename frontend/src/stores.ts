import { writable } from "svelte-local-storage-store";
import constants from "./constants";

const prefix = constants.appName.toLowerCase() + "_";

// First param is the local storage key.
// Second param is the initial value.

export const sidebarCollapsed = writable(prefix + "sidebarCollapsed", false);
export const auth = writable(prefix + "auth", undefined);
export const user = writable(prefix + "user", undefined);
export const minions = writable(prefix + "minions", undefined);