import { get } from 'svelte/store';
import { auth as authStore, currentUser as currentUserStore } from './stores';
import constants from './constants';
import Minion from './models/Minion';
import SaltEvent from './models/SaltEvent';
import Job from './models/Job';
import User from './models/User';
import Key from './models/Key';
import Config from './models/Config';
import MetricResult from './models/MetricResult';
import PermissionGroup from './models/PermissionGroup';
import type Filter from './models/Filter';

export async function createSSESocket(): Promise<EventSource> {
    const token = get(authStore);
    if (!token) {
        throw new Error('Missing API token');
    }

    const stream = new EventSource(
        `${constants.apiUrl}/pipeline?token=${token}`,
    );
    return stream;
}

async function sendAuthenticatedRequest(
    method: string,
    path: string,
    body?: any,
): Promise<any> {
    const token = get(authStore);
    if (!token) {
        throw new Error('Missing API token');
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

///
/// Auth
///

export async function login(
    username: string,
    password: string,
): Promise<string> {
    const { token } = await sendUnauthenticatedRequest('POST', '/auth/login', {
        username,
        password,
    });
    authStore.set(token);
    return token;
}

export async function getConfig(): Promise<Config> {
    return sendUnauthenticatedRequest('GET', '/config').then((data: any) =>
        Config.fromObject(data),
    );
}

export async function getCurrentUser(): Promise<User> {
    return sendAuthenticatedRequest('GET', '/auth/user').then((data) =>
        User.fromObject(data),
    );
}

export async function logout(): Promise<void> {
    // No remote function call for this yet.
    currentUserStore.set(null);
    authStore.set(null);
}

///
/// Users
///

export async function getUsers(
    limit?: number,
    offset?: number,
): Promise<Array<User>> {
    const args = new URLSearchParams();

    if (limit) args.append('limit', limit.toString());
    if (offset) args.append('offset', offset.toString());

    return sendAuthenticatedRequest('GET', `/users?${args.toString()}`).then(
        (data: any[]) => data.map((item) => User.fromObject(item)),
    );
}

export async function getUserById(userId: string): Promise<User> {
    return sendAuthenticatedRequest('GET', `/users/${userId}`).then(
        (data: any) => User.fromObject(data),
    );
}

export async function updateUserPassword(
    userId: string,
    password: string,
): Promise<void> {
    return sendAuthenticatedRequest(
        'POST',
        `/users/${userId}/password`,

        { password },
    );
}

export async function addUserToPermissionGroup(
    userId: string,
    groupId: string,
): Promise<void> {
    return sendAuthenticatedRequest(
        'POST',
        `/users/${userId}/permissions/${groupId}`,
    );
}

export async function removeUserFromPermissionGroup(
    userId: string,
    groupId: string,
): Promise<void> {
    return sendAuthenticatedRequest(
        'DELETE',
        `/users/${userId}/permissions/${groupId}`,
    );
}

///
/// Minions
///

export async function getMinions(
    filters?: Filter[],
    sort?: string,
    limit?: number,
    offset?: number,
): Promise<Array<Minion>> {
    const args = new URLSearchParams();

    if (filters && filters.length > 0) args.append('filter', encodeURIComponent(JSON.stringify(filters)));
    if (sort) args.append('sort', sort);
    if (limit) args.append('limit', limit.toString());
    if (offset) args.append('offset', offset.toString());

    return sendAuthenticatedRequest('GET', `/minions?${args.toString()}`).then(
        (data: any[]) => data.map((item) => Minion.fromObject(item)),
    );
}

export async function refreshMinions(): Promise<void> {
    await sendAuthenticatedRequest('POST', '/minions/refresh');
}

export async function getMinionById(minionId: string): Promise<Minion> {
    return sendAuthenticatedRequest('GET', `/minions/${minionId}`).then(
        (data: any) => Minion.fromObject(data),
    );
}

///
/// Events
///

export async function getEvents(
    limit?: number,
    offset?: number,
): Promise<Array<SaltEvent>> {
    const args = new URLSearchParams();

    if (limit) args.append('limit', limit.toString());
    if (offset) args.append('offset', offset.toString());

    return sendAuthenticatedRequest('GET', `/events?${args.toString()}`).then(
        (data: any[]) => data.map((item) => SaltEvent.fromObject(item)),
    );
}

///
/// Jobs
///

export async function getJobs(
    sort?: string,
    limit?: number,
    offset?: number,
): Promise<Array<Job>> {
    const args = new URLSearchParams();

    if (sort) args.append('sort', sort);
    if (limit) args.append('limit', limit.toString());
    if (offset) args.append('offset', offset.toString());

    return sendAuthenticatedRequest('GET', `/jobs?${args.toString()}`).then(
        (data: any[]) => data.map((item) => Job.fromObject(item)),
    );
}

export async function runJob(
    client: string,
    tgtType: string,
    tgt: string,
    fun: string,
    arg: Array<string>,
    kwarg: Map<string, string>,
    batchSize: string,
    timeout: number,
): Promise<any> {
    return sendAuthenticatedRequest('POST', '/jobs', {
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

export async function getJobById(jobId: string): Promise<Job> {
    return sendAuthenticatedRequest('GET', `/jobs/${jobId}`).then((data: any) =>
        Job.fromObject(data),
    );
}

///
/// Keys
///

export async function getKeys(): Promise<Array<Key>> {
    return sendAuthenticatedRequest('GET', '/keys').then((data: any[]) =>
        data.map((item) => Key.fromObject(item)),
    );
}

export async function acceptKey(key: Key): Promise<void> {
    await sendAuthenticatedRequest(
        'PUT',
        `/keys/${key.state}/${key.id}/accept`,
    );
}

export async function rejectKey(key: Key): Promise<void> {
    await sendAuthenticatedRequest(
        'PUT',
        `/keys/${key.state}/${key.id}/reject`,
    );
}

export async function deleteKey(key: Key): Promise<void> {
    await sendAuthenticatedRequest(
        'DELETE',
        `/keys/${key.state}/${key.id}/delete`,
    );
}

///
/// Metrics
///

export async function getMetricResults(): Promise<Array<MetricResult>> {
    return sendAuthenticatedRequest('GET', '/metrics').then((data: any[]) =>
        data.map((item) => MetricResult.fromObject(item)),
    );
}

///
/// Permission Groups
///

export async function getPermissionGroups(
    limit?: number,
    offset?: number,
): Promise<Array<PermissionGroup>> {
    const args = new URLSearchParams();

    if (limit) args.append('limit', limit.toString());
    if (offset) args.append('offset', offset.toString());

    return sendAuthenticatedRequest(
        'GET',
        `/permissions?${args.toString()}`,
    ).then((data: any[]) =>
        data.map((item) => PermissionGroup.fromObject(item)),
    );
}

export async function getPermissionGroup(id: string): Promise<PermissionGroup> {
    return sendAuthenticatedRequest('GET', `/permissions/${id}`).then(
        (data: any) => PermissionGroup.fromObject(data),
    );
}

export async function createPermissionGroup(
    name: string,
): Promise<PermissionGroup> {
    return sendAuthenticatedRequest('POST', '/permissions', {
        name,
    }).then((data: any) => PermissionGroup.fromObject(data));
}

export async function deletePermissionGroup(id: string): Promise<void> {
    await sendAuthenticatedRequest('DELETE', `/permissions/${id}`);
}

export async function updatePermissionGroup(
    id: string,
    name: string,
    perms: any[],
    ldapSync: string | null,
): Promise<void> {
    return sendAuthenticatedRequest('PUT', `/permissions/${id}`, {
        name,
        perms: JSON.stringify(perms),
        ldapSync,
    });
}
