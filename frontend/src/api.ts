import constants from './constants';
import type Minion from './models/Minion';
import type User from './models/User';
import type SaltEvent from './models/SaltEvent';
import type Job from './models/Job';

// API class is independent, and is not allowed to import svelte/store's.

export async function apiRequestAuthToken(
    username: string,
    password: string,
): Promise<String> {
    const res = await fetch(`${constants.apiUrl}/auth/login`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            username,
            password,
        }),
    });

    if (res.status !== 200) {
        throw new Error(await res.text());
    }

    return (await res.json()).token;
}

export async function apiCreateEventConnection(
    token: string,
): Promise<EventSource> {
    const stream = new EventSource(`${constants.apiUrl}/pipeline?token=${token}`);
    return stream;
}

export async function sendAuthenticatedRequest(
    method: string,
    path: string,
    token: string,
    body?: any,
): Promise<any> {
    if (!token) {
        throw new Error('No API token provided');
    }

    const res = await fetch(constants.apiUrl + path, {
        method,
        headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${token}`,
        },
        body: body ? JSON.stringify(body) : undefined,
    });

    if (res.status !== 200) {
        throw new Error(await res.text());
    }

    return res.json();
}

export async function apiFetchUser(token: string): Promise<User> {
    return sendAuthenticatedRequest('GET', '/auth/user', token);
}

export async function apiListMinions(
    token: string,
    limit?: number,
    offset?: number,
): Promise<Array<Minion>> {
    const args = new URLSearchParams();

    if (limit) args.append('limit', limit.toString());
    if (offset) args.append('offset', offset.toString());

    return sendAuthenticatedRequest('GET', `/minions?${args.toString()}`, token);
}

export async function apiRefreshMinions(token: string): Promise<void> {
    await sendAuthenticatedRequest('POST', '/minions/refresh', token);
}

export async function apiListEvents(
    token: string,
    limit?: number,
    offset?: number,
): Promise<Array<SaltEvent>> {
    const args = new URLSearchParams();

    if (limit) args.append('limit', limit.toString());
    if (offset) args.append('offset', offset.toString());

    return sendAuthenticatedRequest('GET', `/events?${args.toString()}`, token);
}

export async function apiListJobs(
    token: string,
    user?: string,
    startDate?: Date,
    endDate?: Date,
    limit?: number,
    offset?: number,
): Promise<Array<Job>> {
    const args = new URLSearchParams();

    if (user) args.append('user', user);
    if (startDate) args.append('start_date', startDate.toISOString());
    if (endDate) args.append('end_date', endDate.toISOString());
    if (limit) args.append('limit', limit.toString());
    if (offset) args.append('offset', offset.toString());

    return sendAuthenticatedRequest('GET', `/jobs?${args.toString()}`, token);
}
