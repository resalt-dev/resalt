import constants from "./constants";
import { ApiResponse, User } from "./models";

// API class is independent, and is not allowed to import svelte/store's.

export async function request_authtoken(username: string, password: string): Promise<ApiResponse> {
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

    return new ApiResponse(res.status, res.status == 200 ? await res.json() : await res.text());
}

export async function create_event_connection(token: string): Promise<EventSource> {
    var stream = new EventSource(constants.apiUrl + "/events?token=" + token);
    return stream;
}

export async function _authd_req(method: string, path: string, token: string, body?: any): Promise<ApiResponse> {
    if (!token) {
        return new ApiResponse(204, "");
    }

    let res = await fetch(constants.apiUrl + path, {
        method: method,
        headers: {
            "Content-Type": "application/json",
            "Authorization": "Bearer " + token,
        },
        body: body ? JSON.stringify(body) : undefined,
    });

    return new ApiResponse(res.status, res.status == 200 ? await res.json() : await res.text());
}

export async function get_user(token: string): Promise<ApiResponse> {
    return _authd_req("GET", "/auth/user", token);
}

export async function list_minions(token: string, force_refresh: boolean): Promise<ApiResponse> {
    return _authd_req("GET", `/minions?refresh=${force_refresh}`, token);
}