import { get } from "svelte/store";
import {
    auth as authStore,
    user as userStore,
    minions as minionsStore,
    socket as socketStore,
    alerts,
} from "./stores";
import {
    api_create_event_connection,
    api_list_minions,
    api_list_events,
    api_request_authtoken,
    api_list_jobs,
    api_refresh_minions,
    api_fetch_user,
} from "./api";
import { Alert, SaltEvent, User, Job, Minion } from "./models";

/*
 * INTERNAL UTILS
 */

function _require_token(): string {
    let token = get(authStore);
    if (!token) {
        throw new Error("No API token provided");
    }
    return token;
}

/*
 * UTIL
 */

export enum AlertType {
    INFO = "info",
    SUCCESS = "success",
    WARNING = "warning",
    ERROR = "danger",
}

export function showAlert(type: string, title: string, message: string): void {
    alerts.update((alerts) => [...alerts, new Alert(type, title, message)]);
}

/*
 * NETWORK API
 */

export async function login(username: string, password: string) {
    let token: String = await api_request_authtoken(username, password);
    authStore.set(token);
}

export async function logout() {
    authStore.set(null);
    userStore.set(null);
}

let source: EventSource;
export function close_events() {
    if (source) {
        source.close();
    }
}

export async function connect_events(timeout: number) {
    if (typeof timeout != "number") timeout = 1000;

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

    let token = _require_token();
    source = await api_create_event_connection(token);

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
            }
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

export async function load_user(): Promise<void> {
    let token = _require_token();

    try {
        let user = await api_fetch_user(token);
        userStore.set(user);
    } catch (e) {
        console.log(e);
        throw e;
    }
}

export async function load_minions(force_refresh = false) {
    let token = _require_token();

    let minions = await api_list_minions(token);
    minionsStore.set(minions);
}


export async function get_user(): Promise<User> {
    let token = _require_token();
    return await api_fetch_user(token);
}

export async function get_minions(): Promise<Array<Minion>> {
    let token = _require_token();
    return await api_list_minions(token);
}

export async function refresh_minions(): Promise<void> {
    let token = _require_token();
    await api_refresh_minions(token);
}

export async function get_events(): Promise<Array<SaltEvent>> {
    let token = _require_token();
    return await api_list_events(token);
}

export async function get_jobs(): Promise<Array<Job>> {
    let token = _require_token();
    return await api_list_jobs(token);
}