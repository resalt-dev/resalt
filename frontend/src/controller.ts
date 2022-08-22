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
    apiCreateEventConnection,
    apiDeleteKey,
    apiGetConfig,
    apiGetCurrentUser,
    apiGetJobById,
    apiGetMinionById,
    apiGetUser,
    apiListEvents,
    apiListJobs,
    apiListKeys,
    apiListMinions,
    apiListUsers,
    apiRefreshMinions,
    apiRejectKey,
    apiRequestAuthToken,
    apiRunJob,
} from './api';
import Alert from './models/Alert';
import type Minion from './models/Minion';
import type PublicUser from './models/PublicUser';
import type SaltEvent from './models/SaltEvent';
import type Job from './models/Job';
import type Key from './models/Key';
import type Config from './models/Config';

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
// eslint-disable-next-line no-shadow
export enum AlertType {
    // eslint-disable-next-line no-unused-vars
    INFO = 'info',
    // eslint-disable-next-line no-unused-vars
    SUCCESS = 'success',
    // eslint-disable-next-line no-unused-vars
    WARNING = 'warning',
    // eslint-disable-next-line no-unused-vars
    ERROR = 'danger',
}

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

            const { content } = data;

            switch (data.type) {
            /*case 'update_minion':
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
                break;*/
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

export async function loadCurrentUser(): Promise<PublicUser> {
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

export async function getCurrentUser(): Promise<PublicUser> {
    const token = requireToken();
    return apiGetCurrentUser(token);
}

export async function getMinions(limit?: number, offset?: number): Promise<Array<Minion>> {
    const token = requireToken();
    return apiListMinions(token, limit, offset);
}

export async function refreshMinions(): Promise<void> {
    const token = requireToken();
    await apiRefreshMinions(token);
}

export async function getMinionById(id: string): Promise<Minion> {
    const token = requireToken();
    return apiGetMinionById(token, id);
}

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
    tgtType: string,
    tgt: string,
    fun: string,
    arg: Array<string>,
    kwarg: Map<string, string>,
    batchSize: string,
    timeout: number,
): Promise<Job> {
    const token = requireToken();
    return apiRunJob(token, tgtType, tgt, fun, arg, kwarg, batchSize, timeout);
}

export async function getJobById(id: string): Promise<Job> {
    const token = requireToken();
    return apiGetJobById(token, id);
}

export async function getEvents(limit?: number, offset?: number): Promise<Array<SaltEvent>> {
    const token = requireToken();
    return apiListEvents(token, limit, offset);
}

export async function getUsers(limit?: number, offset?: number): Promise<Array<PublicUser>> {
    const token = requireToken();
    return apiListUsers(token, limit, offset);
}

export async function getUserById(id: string): Promise<PublicUser> {
    const token = requireToken();
    return apiGetUser(token, id);
}

export async function getKeys(): Promise<Array<Key>> {
    const token = requireToken();
    return apiListKeys(token);
}

export async function acceptKey(id: string): Promise<void> {
    const token = requireToken();
    await apiAcceptKey(token, id);
}

export async function rejectKey(id: string): Promise<void> {
    const token = requireToken();
    await apiRejectKey(token, id);
}

export async function deleteKey(id: string): Promise<void> {
    const token = requireToken();
    await apiDeleteKey(token, id);
}