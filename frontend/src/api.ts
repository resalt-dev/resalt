import constants from "./constants";
import { Job, Minion, SaltEvent, User } from "./models";

// API class is independent, and is not allowed to import svelte/store's.

export async function request_authtoken(
    username: string,
    password: string
): Promise<String> {
    let res = await fetch(constants.apiUrl + "/auth/login", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            username: username,
            password: password,
        }),
    });

    if (res.status != 200) {
        throw new Error(await res.text());
    }

    return (await res.json()).token;
}

export async function create_event_connection(
    token: string
): Promise<EventSource> {
    var stream = new EventSource(constants.apiUrl + "/pipeline?token=" + token);
    return stream;
}

export async function _authd_req(
    method: string,
    path: string,
    token: string,
    body?: any
): Promise<any> {
    if (!token) {
        throw new Error("No API token provided");
    }

    let res = await fetch(constants.apiUrl + path, {
        method: method,
        headers: {
            "Content-Type": "application/json",
            Authorization: "Bearer " + token,
        },
        body: body ? JSON.stringify(body) : undefined,
    });

    if (res.status != 200) {
        throw new Error(await res.text());
    }

    return res.json();
}

export async function get_user(token: string): Promise<User> {
    return await _authd_req("GET", "/auth/user", token);
}

export async function list_minions(
    token: string,
    force_refresh: boolean
): Promise<Minion[]> {
    return (await _authd_req("GET", `/minions?refresh=${force_refresh}`, token))
        .minions;
}

export async function list_events(token: string): Promise<SaltEvent[]> {
    return (await _authd_req("GET", `/events`, token)).events;
}

export async function list_jobs(
    token: string,
    user?: string,
    start_date?: Date,
    end_date?: Date,
    limit?: number,
    offset?: number
): Promise<Job[]> {
    let args = new URLSearchParams();

    if (user) args.append("user", user);
    if (start_date) args.append("start_date", start_date.toISOString());
    if (end_date) args.append("end_date", end_date.toISOString());
    if (limit) args.append("limit", limit.toString());
    if (offset) args.append("offset", offset.toString());

    return (await _authd_req("GET", `/jobs?${args.toString()}`, token)).jobs;
}
