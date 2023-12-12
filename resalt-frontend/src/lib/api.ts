import AuthToken from '$model/AuthToken';
import Config from '$model/Config';
import type Filter from '$model/Filter';
import Job from '$model/Job';
import Key from '$model/Key';
import Minion from '$model/Minion';
import MinionPreset from '$model/MinionPreset';
import PermissionGroup, { type fPerm } from '$model/PermissionGroup';
import type RunCommand from '$model/RunCommand';
import SaltEvent from '$model/SaltEvent';
import SystemStatus from '$model/SystemStatus';
import User from '$model/User';
import { get } from 'svelte/store';
import constants from './constants';
import { auth as authStore, currentUser as currentUserStore } from './stores';

function getToken(): string {
	const auth = get(authStore);
	if (!auth) {
		throw new Error('Missing API token');
	}
	if (auth.expiry < Date.now() / 1000) {
		throw new Error('Auth token has expired');
	}
	return auth.token;
}

async function sendRequest(url: string, options: RequestInit): Promise<unknown> {
	const res = await fetch(url, options);

	if (res.status === 200) {
		const text = await res.text();
		return JSON.parse(text);
	} else {
		console.log('API ERROR', res.status);
		throw new Error(res.statusText + ' (' + res.status + ')');
	}
}

async function sendAuthenticatedRequest(
	method: string,
	path: string,
	data?: unknown,
): Promise<unknown> {
	const token = getToken();
	// console.log('Sending authenticated request', method, path, data);
	const body = data ? JSON.stringify(data) : undefined;
	// console.log('Sending body', body);
	return await sendRequest(constants.apiAuthUrl + path, {
		method,
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${token}`,
		},
		body,
	});
}

async function sendUnauthenticatedRequest(
	method: string,
	path: string,
	body?: unknown,
): Promise<unknown> {
	return await sendRequest(constants.apiUrl + path, {
		method,
		headers: {
			'Content-Type': 'application/json',
		},
		body: body ? JSON.stringify(body) : undefined,
	});
}

///
/// Auth
///

export async function login(username: string, password: string): Promise<AuthToken> {
	return await sendUnauthenticatedRequest('POST', '/login', {
		username,
		password,
	}).then((data: unknown) => AuthToken.fromObject(data));
}

export async function getConfig(): Promise<Config> {
	return await sendUnauthenticatedRequest('GET', '/config').then((data: unknown) =>
		Config.fromObject(data),
	);
}

export async function getSystemStatus(): Promise<SystemStatus> {
	return await sendAuthenticatedRequest('GET', '/status').then((data: unknown) =>
		SystemStatus.fromObject(data),
	);
}

export async function getCurrentUser(): Promise<User> {
	return sendAuthenticatedRequest('GET', '/myself').then((data) => User.fromObject(data));
}

export async function logout(): Promise<void> {
	// No remote function call for this yet.
	currentUserStore.set(null);
	authStore.set(null);
}

///
/// Users
///

export async function getUsers(limit: number | null, offset: number | null): Promise<Array<User>> {
	const args = new URLSearchParams();

	if (limit) args.append('limit', limit.toString());
	if (offset) args.append('offset', offset.toString());

	return sendAuthenticatedRequest('GET', `/users?${args.toString()}`).then((data: unknown) => {
		return (data as Array<unknown>).map((u) => User.fromObject(u));
	});
}

export async function createUser(username: string, email: string | null): Promise<User> {
	return sendAuthenticatedRequest('POST', '/users', {
		username,
		email,
	}).then((data: unknown) => User.fromObject(data));
}

export async function getUserById(userId: string): Promise<User> {
	return sendAuthenticatedRequest('GET', `/users/${userId}`).then((data: unknown) =>
		User.fromObject(data),
	);
}

export async function deleteUser(userId: string): Promise<void> {
	await sendAuthenticatedRequest('DELETE', `/users/${userId}`);
}

export async function updateUserPassword(userId: string, password: string): Promise<void> {
	await sendAuthenticatedRequest(
		'POST',
		`/users/${userId}/password`,

		{ password },
	);
}

export async function addUserToPermissionGroup(userId: string, groupId: string): Promise<void> {
	await sendAuthenticatedRequest('POST', `/users/${userId}/permissions/${groupId}`);
}

export async function removeUserFromPermissionGroup(
	userId: string,
	groupId: string,
): Promise<void> {
	await sendAuthenticatedRequest('DELETE', `/users/${userId}/permissions/${groupId}`);
}

///
/// Minions
///

export async function getMinions(
	filters: Filter[],
	sort: string | null,
	limit: number | null,
	offset: number | null,
): Promise<Array<Minion>> {
	const filteredFilters = filters.filter((f) => f.isValid());
	const args = new URLSearchParams();

	if (filteredFilters && filteredFilters.length > 0)
		args.append('filter', encodeURIComponent(JSON.stringify(filteredFilters)));
	if (sort) args.append('sort', sort);
	if (limit) args.append('limit', limit.toString());
	if (offset) args.append('offset', offset.toString());

	return sendAuthenticatedRequest('GET', `/minions?${args.toString()}`).then((data: unknown) =>
		(data as Array<unknown>).map((m) => Minion.fromObject(m)),
	);
}

export async function getMinionById(minionId: string): Promise<Minion> {
	return sendAuthenticatedRequest('GET', `/minions/${minionId}`).then((data: unknown) =>
		Minion.fromObject(data),
	);
}

export async function refreshMinion(minionId: string): Promise<void> {
	await sendAuthenticatedRequest('POST', `/minions/${minionId}/refresh`);
}

export async function searchGrains(query: string, filters: Filter[]): Promise<unknown[]> {
	const filteredFilters = filters.filter((f) => f.isValid());
	const args = new URLSearchParams();

	if (query) args.append('query', encodeURIComponent(query));
	if (filteredFilters && filteredFilters.length > 0)
		args.append('filter', encodeURIComponent(JSON.stringify(filteredFilters)));

	return (await sendAuthenticatedRequest('GET', `/grains?${args.toString()}`)) as Array<unknown>;
}

///
/// Minion Presets
///

export async function getMinionPresets(): Promise<Array<MinionPreset>> {
	return sendAuthenticatedRequest('GET', '/presets').then((data: unknown) =>
		(data as Array<unknown>).map((p) => MinionPreset.fromObject(p)),
	);
}

export async function createMinionPreset(name: string, filters: Filter[]): Promise<MinionPreset> {
	const filteredFilters = filters.filter((f) => f.isValid());
	return sendAuthenticatedRequest('POST', '/presets', {
		name,
		filter: JSON.stringify(filteredFilters),
	}).then((data: unknown) => MinionPreset.fromObject(data));
}

export async function getMinionPresetById(id: string): Promise<MinionPreset> {
	if (!id) return Promise.reject('Invalid preset ID');
	if (id.length === 0) return Promise.reject('Invalid preset ID');
	return sendAuthenticatedRequest('GET', `/presets/${id}`).then((data: unknown) =>
		MinionPreset.fromObject(data),
	);
}

export async function updateMinionPreset(
	id: string,
	name: string,
	filters: Filter[],
): Promise<MinionPreset> {
	const filteredFilters = filters.filter((f) => f.isValid());
	return sendAuthenticatedRequest('PUT', `/presets/${id}`, {
		name,
		filter: JSON.stringify(filteredFilters),
	}).then((data: unknown) => MinionPreset.fromObject(data));
}

export async function deleteMinionPreset(id: string): Promise<void> {
	await sendAuthenticatedRequest('DELETE', `/presets/${id}`);
}

///
/// Events
///

export async function getEvents(
	limit: number | null,
	offset: number | null,
): Promise<Array<SaltEvent>> {
	const args = new URLSearchParams();

	if (limit) args.append('limit', limit.toString());
	if (offset) args.append('offset', offset.toString());

	return sendAuthenticatedRequest('GET', `/events?${args.toString()}`).then((data: unknown) =>
		(data as Array<unknown>).map((e) => SaltEvent.fromObject(e)),
	);
}

///
/// Jobs
///

export async function getJobs(
	sort: string | null,
	limit: number | null,
	offset: number | null,
): Promise<Array<Job>> {
	const args = new URLSearchParams();

	if (sort) args.append('sort', sort);
	if (limit) args.append('limit', limit.toString());
	if (offset) args.append('offset', offset.toString());

	return sendAuthenticatedRequest('GET', `/jobs?${args.toString()}`).then((data: unknown) =>
		(data as Array<unknown>).map((j) => Job.fromObject(j)),
	);
}

export async function runJob(command: RunCommand): Promise<unknown> {
	return sendAuthenticatedRequest('POST', '/jobs', {
		client: command.client,
		tgtType: command.targetType,
		tgt: command.target,
		fun: command.fun,
		arg: command.arg,
		kwarg: Object.fromEntries(command.kwarg), // Map<>'s are invisible to JSON.stringify
		batchSize: command.batchSize,
	});
}

export async function getJobById(jobId: string): Promise<Job> {
	return sendAuthenticatedRequest('GET', `/jobs/${jobId}`).then((data: unknown) =>
		Job.fromObject(data),
	);
}

///
/// Keys
///

export async function getKeys(): Promise<Array<Key>> {
	return sendAuthenticatedRequest('GET', '/keys').then((data: unknown) =>
		(data as Array<unknown>).map((k) => Key.fromObject(k)),
	);
}

export async function acceptKey(key: Key): Promise<void> {
	await sendAuthenticatedRequest('PUT', `/keys/${key.state}/${key.id}/accept`);
}

export async function rejectKey(key: Key): Promise<void> {
	await sendAuthenticatedRequest('PUT', `/keys/${key.state}/${key.id}/reject`);
}

export async function deleteKey(key: Key): Promise<void> {
	await sendAuthenticatedRequest('DELETE', `/keys/${key.state}/${key.id}/delete`);
}

///
/// Permission Groups
///

export async function getPermissionGroups(
	limit: number | null,
	offset: number | null,
): Promise<Array<PermissionGroup>> {
	const args = new URLSearchParams();

	if (limit) args.append('limit', limit.toString());
	if (offset) args.append('offset', offset.toString());

	return sendAuthenticatedRequest('GET', `/permissions?${args.toString()}`).then(
		(data: unknown) => (data as Array<unknown>).map((p) => PermissionGroup.fromObject(p)),
	);
}

export async function getPermissionGroup(id: string): Promise<PermissionGroup> {
	return sendAuthenticatedRequest('GET', `/permissions/${id}`).then((data: unknown) =>
		PermissionGroup.fromObject(data),
	);
}

export async function createPermissionGroup(name: string): Promise<PermissionGroup> {
	return sendAuthenticatedRequest('POST', '/permissions', {
		name,
	}).then((data: unknown) => PermissionGroup.fromObject(data));
}

export async function deletePermissionGroup(id: string): Promise<void> {
	await sendAuthenticatedRequest('DELETE', `/permissions/${id}`);
}

export async function updatePermissionGroup(
	id: string,
	name: string,
	perms: fPerm[],
): Promise<void> {
	await sendAuthenticatedRequest('PUT', `/permissions/${id}`, {
		name,
		perms: JSON.stringify(perms),
	});
}

///
/// Settings
///

export async function getExport(): Promise<unknown> {
	return sendAuthenticatedRequest('GET', '/settings/export');
}

export async function importData(data: unknown): Promise<void> {
	await sendAuthenticatedRequest('POST', '/settings/import', data);
}
