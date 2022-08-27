import constants from './constants';
import type Minion from './models/Minion';
import type SaltEvent from './models/SaltEvent';
import type Job from './models/Job';
import type PublicUser from './models/PublicUser';
import type Key from './models/Key';
import type Config from './models/Config';

// API class is independent, and is not allowed to import svelte/store's.

export async function apiCreateEventConnection(
    token: string,
): Promise<EventSource> {
    const stream = new EventSource(`${constants.apiUrl}/pipeline?token=${token}`);
    return stream;
}

async function sendAuthenticatedRequest(
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

async function sendUnauthenticatedRequest(
    method: string,
    path: string,
    body?: any,
): Promise<any> {
    const res = await fetch(constants.apiUrl + path, {
        method,
        headers: {
            'Content-Type': 'application/json',
        },
        body: body ? JSON.stringify(body) : undefined,
    });

    if (res.status !== 200) {
        throw new Error(await res.text());
    }

    return res.json();
}

export async function apiRequestAuthToken(
    username: string,
    password: string,
): Promise<String> {
    return (await sendUnauthenticatedRequest('POST', '/auth/login', {
        username,
        password,
    })).token;
}

export async function apiGetConfig(): Promise<Config> {
    return sendUnauthenticatedRequest('GET', '/config');
}

export async function apiGetCurrentUser(token: string): Promise<PublicUser> {
    return sendAuthenticatedRequest('GET', '/auth/user', token);
}

export async function apiListMinions(
    token: string,
    sort?: string,
    limit?: number,
    offset?: number,
): Promise<Array<Minion>> {
    const args = new URLSearchParams();

    if (sort) args.append('sort', sort);
    if (limit) args.append('limit', limit.toString());
    if (offset) args.append('offset', offset.toString());

    return sendAuthenticatedRequest('GET', `/minions?${args.toString()}`, token);
}

export async function apiRefreshMinions(token: string): Promise<void> {
    await sendAuthenticatedRequest('POST', '/minions/refresh', token);
}

export async function apiGetMinionById(
    token: string,
    minionId: string,
): Promise<Minion> {
    return sendAuthenticatedRequest('GET', `/minions/${minionId}`, token);
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

export async function apiRunJob(
    token: string,
    client: string,
    tgtType: string,
    tgt: string,
    fun: string,
    arg: Array<string>,
    kwarg: Map<string, string>,
    batchSize: string,
    timeout: number,
): Promise<any> {
    return sendAuthenticatedRequest('POST', '/jobs', token, {
        client,
        tgtType,
        tgt,
        fun,
        arg,
        kwarg,
        batchSize,
        timeout,
    });
}

export async function apiGetJobById(
    token: string,
    jobId: string,
): Promise<Job> {
    return sendAuthenticatedRequest('GET', `/jobs/${jobId}`, token);
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

export async function apiListUsers(
    token: string,
    limit?: number,
    offset?: number,
): Promise<Array<PublicUser>> {
    const args = new URLSearchParams();

    if (limit) args.append('limit', limit.toString());
    if (offset) args.append('offset', offset.toString());

    return sendAuthenticatedRequest('GET', `/users?${args.toString()}`, token);
}

export async function apiGetUser(
    token: string,
    username: string,
): Promise<PublicUser> {
    return sendAuthenticatedRequest('GET', `/users/${username}`, token);
}

export async function apiListKeys(
    token: string,
): Promise<Array<Key>> {
    return sendAuthenticatedRequest('GET', '/keys', token);
}

export async function apiAcceptKey(
    token: string,
    key: Key,
): Promise<void> {
    await sendAuthenticatedRequest('PUT', `/keys/${key.state}/${key.id}/accept`, token);
}

export async function apiRejectKey(
    token: string,
    key: Key,
): Promise<void> {
    await sendAuthenticatedRequest('PUT', `/keys/${key.state}/${key.id}/reject`, token);
}

export async function apiDeleteKey(
    token: string,
    key: Key,
): Promise<void> {
    await sendAuthenticatedRequest('DELETE', `/keys/${key.state}/${key.id}/delete`, token);
}
