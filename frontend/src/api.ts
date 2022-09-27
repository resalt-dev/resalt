import constants from './constants';
import Minion from './models/Minion';
import SaltEvent from './models/SaltEvent';
import Job from './models/Job';
import User from './models/User';
import Key from './models/Key';
import Config from './models/Config';
import MetricResult from './models/MetricResult';
import PermissionGroup from './models/PermissionGroup';

// API class is independent, and is not allowed to import svelte/store's.

export async function apiCreateEventConnection(
    token: string,
): Promise<EventSource> {
    const stream = new EventSource(
        `${constants.apiUrl}/pipeline?token=${token}`,
    );
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
): Promise<string> {
    return (
        await sendUnauthenticatedRequest('POST', '/auth/login', {
            username,
            password,
        })
    ).token;
}

export async function apiGetConfig(): Promise<Config> {
    return sendUnauthenticatedRequest('GET', '/config').then((data: any) =>
        Config.fromObject(data),
    );
}

export async function apiGetCurrentUser(token: string): Promise<User> {
    return sendAuthenticatedRequest('GET', '/auth/user', token).then((data) =>
        User.fromObject(data),
    );
}

///
/// Users
///

export async function apiListUsers(
    token: string,
    limit?: number,
    offset?: number,
): Promise<Array<User>> {
    const args = new URLSearchParams();

    if (limit) args.append('limit', limit.toString());
    if (offset) args.append('offset', offset.toString());

    return sendAuthenticatedRequest(
        'GET',
        `/users?${args.toString()}`,
        token,
    ).then((data: any[]) => data.map((item) => User.fromObject(item)));
}

export async function apiGetUser(token: string, userId: string): Promise<User> {
    return sendAuthenticatedRequest('GET', `/users/${userId}`, token).then(
        (data: any) => User.fromObject(data),
    );
}

export async function apiUpdateUserPassword(
    token: string,
    userId: string,
    password: string,
): Promise<void> {
    return sendAuthenticatedRequest(
        'POST',
        `/users/${userId}/password`,
        token,
        { password },
    );
}

export async function apiAddUserToPermissionGroup(
    token: string,
    userId: string,
    groupId: string,
): Promise<void> {
    return sendAuthenticatedRequest(
        'POST',
        `/users/${userId}/permissions/${groupId}`,
        token,
    );
}

export async function apiRemoveUserFromPermissionGroup(
    token: string,
    userId: string,
    groupId: string,
): Promise<void> {
    return sendAuthenticatedRequest(
        'DELETE',
        `/users/${userId}/permissions/${groupId}`,
        token,
    );
}

///
/// Minions
///

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

    return sendAuthenticatedRequest(
        'GET',
        `/minions?${args.toString()}`,
        token,
    ).then((data: any[]) => data.map((item) => Minion.fromObject(item)));
}

export async function apiRefreshMinions(token: string): Promise<void> {
    await sendAuthenticatedRequest('POST', '/minions/refresh', token);
}

export async function apiGetMinionById(
    token: string,
    minionId: string,
): Promise<Minion> {
    return sendAuthenticatedRequest('GET', `/minions/${minionId}`, token).then(
        (data: any) => Minion.fromObject(data),
    );
}

///
/// Events
///

export async function apiListEvents(
    token: string,
    limit?: number,
    offset?: number,
): Promise<Array<SaltEvent>> {
    const args = new URLSearchParams();

    if (limit) args.append('limit', limit.toString());
    if (offset) args.append('offset', offset.toString());

    return sendAuthenticatedRequest(
        'GET',
        `/events?${args.toString()}`,
        token,
    ).then((data: any[]) => data.map((item) => SaltEvent.fromObject(item)));
}

///
/// Jobs
///

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

    return sendAuthenticatedRequest(
        'GET',
        `/jobs?${args.toString()}`,
        token,
    ).then((data: any[]) => data.map((item) => Job.fromObject(item)));
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
    return sendAuthenticatedRequest('GET', `/jobs/${jobId}`, token).then(
        (data: any) => Job.fromObject(data),
    );
}

///
/// Keys
///

export async function apiListKeys(token: string): Promise<Array<Key>> {
    return sendAuthenticatedRequest('GET', '/keys', token).then((data: any[]) =>
        data.map((item) => Key.fromObject(item)),
    );
}

export async function apiAcceptKey(token: string, key: Key): Promise<void> {
    await sendAuthenticatedRequest(
        'PUT',
        `/keys/${key.state}/${key.id}/accept`,
        token,
    );
}

export async function apiRejectKey(token: string, key: Key): Promise<void> {
    await sendAuthenticatedRequest(
        'PUT',
        `/keys/${key.state}/${key.id}/reject`,
        token,
    );
}

export async function apiDeleteKey(token: string, key: Key): Promise<void> {
    await sendAuthenticatedRequest(
        'DELETE',
        `/keys/${key.state}/${key.id}/delete`,
        token,
    );
}

///
/// Metrics
///

export async function apiListMetricResults(
    token: string,
): Promise<Array<MetricResult>> {
    return sendAuthenticatedRequest('GET', '/metrics', token).then(
        (data: any[]) => data.map((item) => MetricResult.fromObject(item)),
    );
}

///
/// Permission Groups
///

export async function apiListPermissionGroups(
    token: string,
    limit?: number,
    offset?: number,
): Promise<Array<PermissionGroup>> {
    const args = new URLSearchParams();

    if (limit) args.append('limit', limit.toString());
    if (offset) args.append('offset', offset.toString());

    return sendAuthenticatedRequest(
        'GET',
        `/permissions?${args.toString()}`,
        token,
    ).then((data: any[]) =>
        data.map((item) => PermissionGroup.fromObject(item)),
    );
}

export async function apiGetPermissionGroup(
    token: string,
    id: string,
): Promise<PermissionGroup> {
    return sendAuthenticatedRequest('GET', `/permissions/${id}`, token).then(
        (data: any) => PermissionGroup.fromObject(data),
    );
}

export async function apiCreatePermissionGroup(
    token: string,
    name: string,
): Promise<PermissionGroup> {
    return sendAuthenticatedRequest('POST', '/permissions', token, {
        name,
    }).then((data: any) => PermissionGroup.fromObject(data));
}

export async function apiDeletePermissionGroup(
    token: string,
    id: string,
): Promise<void> {
    await sendAuthenticatedRequest('DELETE', `/permissions/${id}`, token);
}

export async function apiUpdatePermissionGroup(
    token: string,
    id: string,
    name: string,
    perms: any[],
    ldapSync: string | null,
): Promise<void> {
    return sendAuthenticatedRequest('PUT', `/permissions/${id}`, token, {
        name,
        perms: JSON.stringify(perms),
        ldapSync,
    });
}
