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
import type RunCommand from './models/RunCommand';
import AuthToken from './models/AuthToken';
import SystemStatus from './models/SystemStatus';
import MinionPreset from './models/MinionPreset';
import { FilterFieldType } from './models/FilterFieldType';

export class ApiError extends Error {
	code: number;
	// "error" is the `"error": {}` object inside the response
	constructor(error: any) {
		if (typeof error !== 'object' || error === null) {
			throw new Error('ApiError must be constructed with an object');
		}
		if (!error.message || typeof error.message !== 'string') {
			throw new Error('ApiError must have a message');
		}
		if (!error.code || typeof error.code !== 'number') {
			throw new Error('ApiError must have a code');
		}
		super(error.message);
		this.name = 'ApiError';
		this.code = error.code;
	}

	toString(): string {
		return `${this.name}: ${this.message} (${this.code})`;
	}
}

function filterFilters(filters: Filter[]): Filter[] {
	return (
		filters
			.filter((f) => f.fieldType !== FilterFieldType.NONE)
			.filter((f) => f.field !== '')
			// Filter out where field is 'last_seen' and value is empty
			.filter((f) => !(f.field === 'last_seen' && f.value === ''))
	);
}

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

export async function createSSESocket(): Promise<EventSource> {
	const token = getToken();
	const stream = new EventSource(`${constants.apiUrl}/pipeline?token=${token}`);
	return stream;
}

async function sendRequest(url: string, options: any): Promise<any> {
	const res = await fetch(url, options);

	if (res.status === 200) {
		const text = await res.text();
		return JSON.parse(text);
	} else {
		const body = await res.text();
		// Try parse JSON
		let parsed = null;
		try {
			parsed = JSON.parse(body);
		} catch (e) {
			// If it fails, just return the text
			throw new Error(body);
		}

		// Check if body has value "error"
		if (Object.prototype.hasOwnProperty.call(parsed, 'error')) {
			throw new ApiError(parsed.error);
		} else {
			console.error('FAILED PARSING ERROR', body);
			throw new Error('Failed parsing error');
		}
	}
}

async function sendAuthenticatedRequest(method: string, path: string, data?: any): Promise<any> {
	const token = getToken();
	console.log('Sending authenticated request', method, path, data);
	let body = data ? JSON.stringify(data) : undefined;
	console.log('Sending body', body);
	return await sendRequest(constants.apiUrl + path, {
		method,
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${token}`,
		},
		body,
	});
}

async function sendUnauthenticatedRequest(method: string, path: string, body?: any): Promise<any> {
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
	return await sendUnauthenticatedRequest('POST', '/auth/login', {
		username,
		password,
	}).then((data: any) => AuthToken.fromObject(data));
}

export async function getConfig(): Promise<Config> {
	return await sendUnauthenticatedRequest('GET', '/config').then((data: any) =>
		Config.fromObject(data),
	);
}

export async function getSystemStatus(): Promise<SystemStatus> {
	return await sendAuthenticatedRequest('GET', '/status').then((data: any) =>
		SystemStatus.fromObject(data),
	);
}

export async function getCurrentUser(): Promise<User> {
	return sendAuthenticatedRequest('GET', '/auth/user').then((data) => User.fromObject(data));
}

export async function logout(): Promise<void> {
	// No remote function call for this yet.
	currentUserStore.set(null);
	authStore.set(null);
}

///
/// Users
///

export async function getUsers(limit?: number, offset?: number): Promise<Array<User>> {
	const args = new URLSearchParams();

	if (limit) args.append('limit', limit.toString());
	if (offset) args.append('offset', offset.toString());

	return sendAuthenticatedRequest('GET', `/users?${args.toString()}`).then((data: any[]) =>
		data.map((item) => User.fromObject(item)),
	);
}

export async function createUser(
	username: string,
	email: string | null,
	ldapSync: string | null,
): Promise<User> {
	return sendAuthenticatedRequest('POST', '/users', {
		username,
		email,
		ldapSync,
	}).then((data: any) => User.fromObject(data));
}

export async function getUserById(userId: string): Promise<User> {
	return sendAuthenticatedRequest('GET', `/users/${userId}`).then((data: any) =>
		User.fromObject(data),
	);
}

export async function deleteUser(userId: string): Promise<void> {
	return sendAuthenticatedRequest('DELETE', `/users/${userId}`);
}

export async function updateUserPassword(userId: string, password: string): Promise<void> {
	return sendAuthenticatedRequest(
		'POST',
		`/users/${userId}/password`,

		{ password },
	);
}

export async function addUserToPermissionGroup(userId: string, groupId: string): Promise<void> {
	return sendAuthenticatedRequest('POST', `/users/${userId}/permissions/${groupId}`);
}

export async function removeUserFromPermissionGroup(
	userId: string,
	groupId: string,
): Promise<void> {
	return sendAuthenticatedRequest('DELETE', `/users/${userId}/permissions/${groupId}`);
}

///
/// Minions
///

export async function getMinions(
	filters: Filter[],
	sort?: string,
	limit?: number,
	offset?: number,
): Promise<Array<Minion>> {
	const filteredFilters = filterFilters(filters);
	const args = new URLSearchParams();

	if (filteredFilters && filteredFilters.length > 0)
		args.append('filter', encodeURIComponent(JSON.stringify(filteredFilters)));
	if (sort) args.append('sort', sort);
	if (limit) args.append('limit', limit.toString());
	if (offset) args.append('offset', offset.toString());

	return sendAuthenticatedRequest('GET', `/minions?${args.toString()}`).then((data: any[]) =>
		data.map((item) => Minion.fromObject(item)),
	);
}

export async function getMinionById(minionId: string): Promise<Minion> {
	return sendAuthenticatedRequest('GET', `/minions/${minionId}`).then((data: any) =>
		Minion.fromObject(data),
	);
}

export async function refreshMinion(minionId: string): Promise<void> {
	await sendAuthenticatedRequest('POST', `/minions/${minionId}/refresh`);
}

export async function searchGrains(query: string, filters: Filter[]): Promise<any[]> {
	const filteredFilters = filterFilters(filters);
	const args = new URLSearchParams();

	if (query) args.append('query', encodeURIComponent(query));
	if (filteredFilters && filteredFilters.length > 0)
		args.append('filter', encodeURIComponent(JSON.stringify(filteredFilters)));

	return sendAuthenticatedRequest('GET', `/grains?${args.toString()}`);
}

///
/// Minion Presets
///

export async function getMinionPresets(
	search?: string,
	limit?: number,
	offset?: number,
): Promise<Array<MinionPreset>> {
	const args = new URLSearchParams();

	if (search && search.length > 0) args.append('search', search);
	if (limit) args.append('limit', limit.toString());
	if (offset) args.append('offset', offset.toString());

	return sendAuthenticatedRequest('GET', `/presets?${args.toString()}`).then((data: any[]) =>
		data.map((item) => MinionPreset.fromObject(item)),
	);
}

export async function createMinionPreset(name: string, filters: Filter[]): Promise<MinionPreset> {
	const filteredFilters = filterFilters(filters);
	return sendAuthenticatedRequest('POST', '/presets', {
		name,
		filter: JSON.stringify(filteredFilters),
	}).then((data: any) => MinionPreset.fromObject(data));
}

export async function getMinionPresetById(id: string): Promise<MinionPreset> {
	if (!id) return Promise.reject('Invalid preset ID');
	if (id.length === 0) return Promise.reject('Invalid preset ID');
	return sendAuthenticatedRequest('GET', `/presets/${id}`).then((data: any) =>
		MinionPreset.fromObject(data),
	);
}

export async function updateMinionPreset(
	id: string,
	name: string,
	filters: Filter[],
): Promise<MinionPreset> {
	const filteredFilters = filterFilters(filters);
	return sendAuthenticatedRequest('PUT', `/presets/${id}`, {
		name,
		filter: JSON.stringify(filteredFilters),
	}).then((data: any) => MinionPreset.fromObject(data));
}

export async function deleteMinionPreset(id: string): Promise<void> {
	return sendAuthenticatedRequest('DELETE', `/presets/${id}`);
}

///
/// Events
///

export async function getEvents(limit?: number, offset?: number): Promise<Array<SaltEvent>> {
	const args = new URLSearchParams();

	if (limit) args.append('limit', limit.toString());
	if (offset) args.append('offset', offset.toString());

	return sendAuthenticatedRequest('GET', `/events?${args.toString()}`).then((data: any[]) =>
		data.map((item) => SaltEvent.fromObject(item)),
	);
}

///
/// Jobs
///

export async function getJobs(sort?: string, limit?: number, offset?: number): Promise<Array<Job>> {
	const args = new URLSearchParams();

	if (sort) args.append('sort', sort);
	if (limit) args.append('limit', limit.toString());
	if (offset) args.append('offset', offset.toString());

	return sendAuthenticatedRequest('GET', `/jobs?${args.toString()}`).then((data: any[]) =>
		data.map((item) => Job.fromObject(item)),
	);
}

export async function runJob(command: RunCommand): Promise<any> {
	console.log(command);
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
	await sendAuthenticatedRequest('PUT', `/keys/${key.state}/${key.id}/accept`);
}

export async function rejectKey(key: Key): Promise<void> {
	await sendAuthenticatedRequest('PUT', `/keys/${key.state}/${key.id}/reject`);
}

export async function deleteKey(key: Key): Promise<void> {
	await sendAuthenticatedRequest('DELETE', `/keys/${key.state}/${key.id}/delete`);
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

	return sendAuthenticatedRequest('GET', `/permissions?${args.toString()}`).then((data: any[]) =>
		data.map((item) => PermissionGroup.fromObject(item)),
	);
}

export async function getPermissionGroup(id: string): Promise<PermissionGroup> {
	return sendAuthenticatedRequest('GET', `/permissions/${id}`).then((data: any) =>
		PermissionGroup.fromObject(data),
	);
}

export async function createPermissionGroup(name: string): Promise<PermissionGroup> {
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
