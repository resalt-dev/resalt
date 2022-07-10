import { get } from 'svelte/store';
import {
    auth as authStore,
    user as userStore,
    minions as minionsStore,
    socket as socketStore,
    alerts,
} from './stores';
import {
    apiCreateEventConnection,
    apiListMinions,
    apiListEvents,
    apiRequestAuthToken,
    apiListJobs,
    apiRefreshMinions,
    apiFetchUser,
} from './api';
import {
    Alert, SaltEvent, User, Job, Minion,
} from './models';

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
    userStore.set(null);
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
            case 'update_minion':
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
                break;
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

export async function loadUser(): Promise<void> {
    const token = requireToken();

    try {
        const user = await apiFetchUser(token);
        userStore.set(user);
    } catch (e) {
        console.log(e);
        throw e;
    }
}

export async function loadMinions() {
    const token = requireToken();

    const minions = await apiListMinions(token);
    minionsStore.set(minions);
}

export async function getUser(): Promise<User> {
    const token = requireToken();
    return apiFetchUser(token);
}

export async function getMinions(limit?: number, offset?: number): Promise<Array<Minion>> {
    const token = requireToken();
    return apiListMinions(token, limit, offset);
}

export async function refreshMinions(): Promise<void> {
    const token = requireToken();
    await apiRefreshMinions(token);
}

export async function getEvents(limit?: number, offset?: number): Promise<Array<SaltEvent>> {
    const token = requireToken();
    return apiListEvents(token, limit, offset);
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
