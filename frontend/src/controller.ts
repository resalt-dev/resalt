import { get } from "svelte/store";
import {
    auth as authStore,
    user as userStore,
    minions as minionsStore,
} from "./stores";
import { create_event_connection, list_minions, request_authtoken } from "./api";
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

function require_token(navigate?): boolean {
    let token = get(authStore);
    if (!token) {
        if (!!navigate) {
            navigate(paths.login.path);
        }
        return false;
    }
    return true;
}

export async function connect_events(timeout: number) {
    if (typeof timeout != "number") timeout = 1000;

    if (!require_token()) return;

    let token = get(authStore);
    let source = await create_event_connection(token);

    source.addEventListener('message', function (e) {
        console.log(e.data);
    }, false);

    source.addEventListener('open', function (e) {
        // Connection was opened.
        console.log("SSE Connected");
    }, false);

    source.addEventListener('error', function (e) {
        // Connection was closed.
        console.log("Retrying SSE connection in " + Math.round(timeout/1000) + " seconds...");
        setTimeout(() => {
            connect_events(Math.min(timeout * 2, 5 * 60 * 1000));
        }, timeout);
    }, false);
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
