import { get } from "svelte/store";
import {
    auth as authStore,
    user as userStore,
    minions as minionsStore,
    socket as socketStore,
    alerts,
} from "./stores";
import {
    create_event_connection,
    list_minions,
    request_authtoken,
} from "./api";
import paths from "./paths";
import { get_user } from "./api";

import { ApiResponse, Alert } from "./models";

function alert(type: string, message: string): void {
    alerts.update((alerts) => [...alerts, new Alert(type, message)]);
}

export async function login(navigate, username: string, password: string) {
    let result: ApiResponse = await request_authtoken(username, password);

    if (result.status == 200) {
        authStore.set(result.data.token);
        await load_user(navigate);
        navigate(paths.home.path);
    } else {
        // todo: error message is in result.data
        console.log("login error", result);
        alert("danger", "Login failed: " + result.data);
        logout();
    }
}

export async function logout() {
    authStore.set(null);
    userStore.set(null);
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

let source: EventSource;

export function close_events() {
    if (source) {
        source.close();
    }
}

export async function connect_events(timeout: number) {
    if (typeof timeout != "number") timeout = 1000;

    if (!require_token()) return;

    if (source && source.readyState == EventSource.OPEN) {
        console.log(
            "Tried connecting to SSE when already connected, returning same."
        );
        return source;
    } else {
        if (get(socketStore).connected) {
            socketStore.set({ connected: false, last_ping: null });
        }
    }

    let token = get(authStore);
    source = await create_event_connection(token);

    source.addEventListener(
        "message",
        function (e) {
            let data = JSON.parse(e.data);
            console.log("data", data);

            const content = data.content;

            switch (data.type) {
                case "update_minion":
                    minionsStore.update((minions) => {
                        // minions is a Vector of Minions.
                        // If minion exists, replace it. If not, then add it.
                        let index = minions.findIndex(
                            (minion) => minion.id == content.minion.id
                        );
                        if (index >= 0) {
                            minions[index] = content.minion;
                        } else {
                            minions.push(content.minion);
                        }
                        return minions;
                    });
                    break;
            };
        },
        false
    );

    source.addEventListener(
        "ping",
        function (e) {
            let time = new Date(JSON.parse(e.data).time + "Z");
            socketStore.update((s) => {
                s.last_ping = time;
                return s;
            });
            // console.log("ping", time);
        },
        false
    );

    source.addEventListener(
        "open",
        function (e) {
            // Connection was opened.
            socketStore.set({ connected: true, last_ping: null });
            console.log("SSE Connected");
        },
        false
    );

    source.addEventListener(
        "error",
        function (e) {
            // Connection was closed.
            socketStore.set({ connected: false, last_ping: null });
            console.log(
                "Retrying SSE connection in " +
                    Math.round(timeout / 1000) +
                    " seconds..."
            );
            setTimeout(() => {
                connect_events(Math.min(timeout * 2, 5 * 60 * 1000));
            }, timeout);
        },
        false
    );
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
        return true;
    } else {
        // todo: error message is in result.data
        return false;
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
