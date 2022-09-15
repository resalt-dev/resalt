import { get } from 'svelte/store';
import {
    auth as authStore,
    config as configStore,
    currentUser as currentUserStore,
    socket as socketStore,
    alerts,
} from './stores';
import {
    apiAcceptKey,
    apiAddUserToPermissionGroup,
    apiCreateEventConnection,
    apiCreatePermissionGroup,
    apiDeleteKey,
    apiDeletePermissionGroup,
    apiGetConfig,
    apiGetCurrentUser,
    apiGetJobById,
    apiGetMinionById,
    apiGetPermissionGroup,
    apiGetUser,
    apiListEvents,
    apiListJobs,
    apiListKeys,
    apiListMetricResults,
    apiListMinions,
    apiListPermissionGroups,
    apiListUsers,
    apiRefreshMinions,
    apiRejectKey,
    apiRequestAuthToken,
    apiRunJob,
    apiUpdatePermissionGroup,
} from './api';
import Alert from './models/Alert';
import type Minion from './models/Minion';
import type User from './models/User';
import type SaltEvent from './models/SaltEvent';
import type Job from './models/Job';
import type Key from './models/Key';
import type Config from './models/Config';
import type MetricResult from './models/MetricResult';
import type PermissionGroup from './models/PermissionGroup';

/*
 * INTERNAL UTILS
 */

function requireToken(): string {
    const token = get(authStore);
    if (!token) {
        throw new Error('No API token provided');
    }
    return token;
}

/*
 * UTIL
 */
export function showAlert(type: string, title: string, message: string): void {
    alerts.update((mAlerts) => [...mAlerts, new Alert(type, title, message)]);
}

/*
 * NETWORK API
 */

export async function login(username: string, password: string): Promise<void> {
    const token: String = await apiRequestAuthToken(username, password);
    authStore.set(token);
}

export async function logout(): Promise<void> {
    authStore.set(null);
    currentUserStore.set(null);
}

let source: EventSource;
export function closeEvents(): void {
    if (source) {
        source.close();
    }
}

export async function connectEvents(timeout: number = 1000): Promise<EventSource> {
    if (source && source.readyState === EventSource.OPEN) {
        console.log(
            'Tried connecting to SSE when already connected, returning same.',
        );
        return source;
    }
    if (get(socketStore).connected) {
        socketStore.set({ connected: false, last_ping: null });
    }

    const token = requireToken();
    source = await apiCreateEventConnection(token);

    source.addEventListener(
        'message',
        (e) => {
            const data = JSON.parse(e.data);
            console.log('data', data);

            // eslint-disable-next-line no-unused-vars
            const { content } = data;

            switch (data.type) {
            /* case 'update_minion':
                minionsStore.update((minions: Array<Minion>) => {
                    // minions is a Vector of Minions.
                    // If minion exists, replace it. If not, then add it.
                    const index = minions.findIndex(
                        (minion) => minion.id === content.minion.id,
                    );
                    if (index >= 0) {
                        minions[index] = content.minion;
                    } else {
                        minions.push(content.minion);
                    }
                    return minions;
                });
                break; */
            default:
                console.log('Unknown event type', data.type);
            }
        },
        false,
    );

    source.addEventListener(
        'ping',
        (e) => {
            const time = new Date(`${JSON.parse(e.data).time}Z`);
            socketStore.update((s) => {
                s.last_ping = time;
                return s;
            });
            // console.log("ping", time);
        },
        false,
    );

    source.addEventListener(
        'open',
        () => {
            // Connection was opened.
            socketStore.set({ connected: true, last_ping: null });
            console.log('SSE Connected');
        },
        false,
    );

    source.addEventListener(
        'error',
        () => {
            // Connection was closed.
            socketStore.set({ connected: false, last_ping: null });
            console.log(
                `Retrying SSE connection in ${
                    Math.round(timeout / 1000)
                } seconds...`,
            );
            setTimeout(() => {
                connectEvents(Math.min(timeout * 2, 5 * 60 * 1000));
            }, timeout);
        },
        false,
    );

    return source;
}

export async function loadConfig(): Promise<Config> {
    try {
        const config = await apiGetConfig();
        configStore.set(config);
        return config;
    } catch (e) {
        console.log(e);
        throw e;
    }
}

export async function loadCurrentUser(): Promise<User> {
    const token = requireToken();

    try {
        const currentUser = await apiGetCurrentUser(token);
        currentUserStore.set(currentUser);
        return currentUser;
    } catch (e) {
        console.log(e);
        throw e;
    }
}

export async function getCurrentUser(): Promise<User> {
    const token = requireToken();
    return apiGetCurrentUser(token);
}

///
/// Users
///

export async function getUsers(limit?: number, offset?: number): Promise<Array<User>> {
    const token = requireToken();
    return apiListUsers(token, limit, offset);
}

export async function getUserById(id: string): Promise<User> {
    const token = requireToken();
    return apiGetUser(token, id);
}

export async function addUserToPermissionGroup(
    userId: string,
    permissionGroupId: string,
): Promise<void> {
    const token = requireToken();
    return apiAddUserToPermissionGroup(token, userId, permissionGroupId);
}

export async function removeUserFromPermissionGroup(
    userId: string,
    permissionGroupId: string,
): Promise<void> {
    const token = requireToken();
    return apiAddUserToPermissionGroup(token, userId, permissionGroupId);
}

///
/// Minions
///

export async function getMinions(
    sort?: string,
    limit?: number,
    offset?: number,
): Promise<Array<Minion>> {
    const token = requireToken();
    return apiListMinions(token, sort, limit, offset);
}

export async function refreshMinions(): Promise<void> {
    const token = requireToken();
    await apiRefreshMinions(token);
}

export async function getMinionById(id: string): Promise<Minion> {
    const token = requireToken();
    return apiGetMinionById(token, id);
}

///
/// Events
///

export async function getEvents(limit?: number, offset?: number): Promise<Array<SaltEvent>> {
    const token = requireToken();
    return apiListEvents(token, limit, offset);
}

///
/// Jobs
///

export async function getJobs(
    user?: string,
    startDate?: Date,
    endDate?: Date,
    limit?: number,
    offset?: number,
): Promise<Array<Job>> {
    const token = requireToken();
    return apiListJobs(token, user, startDate, endDate, limit, offset);
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
    const token = requireToken();
    return apiRunJob(token, client, tgtType, tgt, fun, arg, kwarg, batchSize, timeout);
}

export async function getJobById(id: string): Promise<Job> {
    const token = requireToken();
    return apiGetJobById(token, id);
}

///
/// Keys
///

export async function getKeys(): Promise<Array<Key>> {
    const token = requireToken();
    return apiListKeys(token);
}

export async function acceptKey(key: Key): Promise<void> {
    const token = requireToken();
    await apiAcceptKey(token, key);
}

export async function rejectKey(key: Key): Promise<void> {
    const token = requireToken();
    await apiRejectKey(token, key);
}

export async function deleteKey(key: Key): Promise<void> {
    const token = requireToken();
    await apiDeleteKey(token, key);
}

///
/// Metrics
///

export async function getMetricResults(): Promise<Array<MetricResult>> {
    const token = requireToken();
    return apiListMetricResults(token);
}

///
/// Permission Groups
///

export async function getPermissionGroups(limit: number, offset: number): Promise<Array<PermissionGroup>> {
    const token = requireToken();
    return apiListPermissionGroups(token, limit, offset);
}

export async function getPermissionGroup(id: string): Promise<PermissionGroup> {
    const token = requireToken();
    return apiGetPermissionGroup(token, id);
}

export async function createPermissionGroup(
    name: string,
): Promise<PermissionGroup> {
    const token = requireToken();
    return apiCreatePermissionGroup(token, name);
}

export async function deletePermissionGroup(id: string): Promise<void> {
    const token = requireToken();
    return apiDeletePermissionGroup(token, id);
}

export async function updatePermissionGroup(
    id: string,
    name: string,
    perms: any[],
    ldapSync: string | null,
): Promise<void> {
    const token = requireToken();
    return apiUpdatePermissionGroup(token, id, name, perms, ldapSync);
}
