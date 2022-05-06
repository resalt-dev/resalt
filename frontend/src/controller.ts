import { get } from "svelte/store";
import {
    auth as authStore,
    user as userStore,
    minions as minionsStore,
} from "./stores";
import { list_minions, request_authtoken } from "./api";
import paths from "./paths";
import { get_user } from "./api";

import type { ApiResponse } from "./models";

export async function login(navigate, username: string, password: string) {
    let result: ApiResponse = await request_authtoken(username, password);

    if (result.status == 200) {
        authStore.set(result.data.token);
        await load_user(navigate);
        navigate(paths.home.path);
    } else {
        // todo: error message is in result.data
        logout();
    }
}

export async function logout() {
    authStore.set(undefined);
    userStore.set(undefined);
}

function require_token(navigate): boolean {
    let token = get(authStore);
    if (!token) {
        navigate(paths.login.path);
        return false;
    }
    return true;
}

export async function load_user(navigate) {
    if (!require_token(navigate)) return;

    let token = get(authStore);

    let result = await get_user(token);
    if (result.status == 200) {
        userStore.set(result.data);
    } else if (result.status == 401) {
        logout();
        navigate(paths.logout.path);
    } else {
        // todo: error message is in result.data
    }
}

export async function load_minions(navigate, force_refresh = false) {
    if (!require_token(navigate)) return;

    let token = get(authStore);

    let result = await list_minions(token, force_refresh);
    if (result.status == 200) {
        minionsStore.set(result.data.minions);
    } else {
        // todo: error handle
    }
}
