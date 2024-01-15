import Config from '../models/Config';
import type Filter from '../models/Filter';
import Job from '../models/Job';
import Key from '../models/Key';
import Minion from '../models/Minion';
import MinionPreset from '../models/MinionPreset';
import PermissionGroup, { type fPerm } from '../models/PermissionGroup';
import type RunCommand from '../models/RunCommand';
import SaltEvent from '../models/SaltEvent';
import SystemStatus from '../models/SystemStatus';
import User from '../models/User';

const API_URL = '/api';
const u = undefined;

function sendRequest(
	method: string,
	path: string,
	data: unknown,
	signal: AbortSignal,
): Promise<unknown> {
	// console.log('Sending authenticated request', method, path, data);
	const body = data ? JSON.stringify(data) : undefined;
	// console.log('Sending body', body);

	return new Promise((resolve, reject) => {
		const errFunc = (err: Error) => {
			if (signal.aborted) return;
			console.log('API ERROR', err);
			reject(err);
		};
		fetch(API_URL + path, {
			method,
			headers: {
				'Content-Type': 'application/json',
			},
			body,
			signal,
		})
			.then((res) => {
				if (res.status !== 200) {
					errFunc(new Error(res.statusText + ' (' + res.status + ')'));
					return;
				}
				res.text()
					.then((text) => {
						resolve(JSON.parse(text));
					})
					.catch(errFunc);
			})
			.catch(errFunc);
	});
}

///
/// Auth
///

export async function login(username: string, password: string, abort: AbortSignal): Promise<void> {
	await sendRequest(
		'POST',
		'/login',
		{
			username,
			password,
		},
		abort,
	);
}

export async function logout(abort: AbortSignal): Promise<void> {
	await sendRequest('POST', '/logout', u, abort);
}

export async function getConfig(abort: AbortSignal): Promise<Config> {
	return await sendRequest('GET', '/config', u, abort).then((data: unknown) =>
		Config.fromObject(data),
	);
}

export async function getSystemStatus(abort: AbortSignal): Promise<SystemStatus> {
	return await sendRequest('GET', '/status', u, abort).then((data: unknown) =>
		SystemStatus.fromObject(data),
	);
}

export async function getCurrentUser(abort: AbortSignal): Promise<User> {
	return sendRequest('GET', '/myself', u, abort).then((data) => User.fromObject(data));
}

///
/// Users
///

export async function getUsers(
	limit: number | null,
	offset: number | null,
	abort: AbortSignal,
): Promise<User[]> {
	const args = new URLSearchParams();

	if (limit) args.append('limit', limit.toString());
	if (offset) args.append('offset', offset.toString());

	return sendRequest('GET', `/users?${args.toString()}`, u, abort).then((data: unknown) => {
		return (data as unknown[]).map((u) => User.fromObject(u));
	});
}

export async function createUser(
	username: string,
	email: string | null,
	abort: AbortSignal,
): Promise<User> {
	return sendRequest(
		'POST',
		'/users',
		{
			username,
			email,
		},
		abort,
	).then((data: unknown) => User.fromObject(data));
}

export async function getUserById(userId: string, abort: AbortSignal): Promise<User> {
	return sendRequest('GET', `/users/${userId}`, u, abort).then((data: unknown) =>
		User.fromObject(data),
	);
}

export async function deleteUser(userId: string, abort: AbortSignal): Promise<void> {
	await sendRequest('DELETE', `/users/${userId}`, u, abort);
}

export async function updateUserPassword(
	userId: string,
	password: string,
	abort: AbortSignal,
): Promise<void> {
	await sendRequest(
		'POST',
		`/users/${userId}/password`,

		{ password },
		abort,
	);
}

export async function addUserToPermissionGroup(
	userId: string,
	groupId: string,
	abort: AbortSignal,
): Promise<void> {
	await sendRequest('POST', `/users/${userId}/permissions/${groupId}`, u, abort);
}

export async function removeUserFromPermissionGroup(
	userId: string,
	groupId: string,
	abort: AbortSignal,
): Promise<void> {
	await sendRequest('DELETE', `/users/${userId}/permissions/${groupId}`, u, abort);
}

///
/// Minions
///

export async function getMinions(
	filters: Filter[],
	sort: string | null,
	limit: number | null,
	offset: number | null,
	abort: AbortSignal,
): Promise<Minion[]> {
	const filteredFilters = filters.filter((f) => f.isValid());
	const args = new URLSearchParams();

	if (filteredFilters.length > 0)
		args.append('filter', encodeURIComponent(JSON.stringify(filteredFilters)));
	if (sort) args.append('sort', sort);
	if (limit) args.append('limit', limit.toString());
	if (offset) args.append('offset', offset.toString());

	return sendRequest('GET', `/minions?${args.toString()}`, u, abort).then((data: unknown) =>
		(data as unknown[]).map((m) => Minion.fromObject(m)),
	);
}

export async function getMinionById(minionId: string, abort: AbortSignal): Promise<Minion> {
	return sendRequest('GET', `/minions/${minionId}`, u, abort).then((data: unknown) =>
		Minion.fromObject(data),
	);
}

export async function refreshMinion(minionId: string, abort: AbortSignal): Promise<void> {
	await sendRequest('POST', `/minions/${minionId}/refresh`, u, abort);
}

export async function searchGrains(
	query: string,
	filters: Filter[],
	abort: AbortSignal,
): Promise<unknown[]> {
	const filteredFilters = filters.filter((f) => f.isValid());
	const args = new URLSearchParams();

	if (query) args.append('query', encodeURIComponent(query));
	if (filteredFilters.length > 0)
		args.append('filter', encodeURIComponent(JSON.stringify(filteredFilters)));

	return (await sendRequest('GET', `/grains?${args.toString()}`, u, abort)) as unknown[];
}

///
/// Minion Presets
///

export async function getMinionPresets(abort: AbortSignal): Promise<MinionPreset[]> {
	return sendRequest('GET', '/presets', u, abort).then((data: unknown) =>
		(data as unknown[]).map((p) => MinionPreset.fromObject(p)),
	);
}

export async function createMinionPreset(
	name: string,
	filters: Filter[],
	abort: AbortSignal,
): Promise<MinionPreset> {
	const filteredFilters = filters.filter((f) => f.isValid());
	return sendRequest(
		'POST',
		'/presets',
		{
			name,
			filter: JSON.stringify(filteredFilters),
		},
		abort,
	).then((data: unknown) => MinionPreset.fromObject(data));
}

export async function getMinionPresetById(id: string, abort: AbortSignal): Promise<MinionPreset> {
	if (!id) return Promise.reject('Invalid preset ID');
	if (id.length === 0) return Promise.reject('Invalid preset ID');
	return sendRequest('GET', `/presets/${id}`, u, abort).then((data: unknown) =>
		MinionPreset.fromObject(data),
	);
}

export async function updateMinionPreset(
	id: string,
	name: string,
	filters: Filter[],
	abort: AbortSignal,
): Promise<MinionPreset> {
	const filteredFilters = filters.filter((f) => f.isValid());
	return sendRequest(
		'PUT',
		`/presets/${id}`,
		{
			name,
			filter: JSON.stringify(filteredFilters),
		},
		abort,
	).then((data: unknown) => MinionPreset.fromObject(data));
}

export async function deleteMinionPreset(id: string, abort: AbortSignal): Promise<void> {
	await sendRequest('DELETE', `/presets/${id}`, u, abort);
}

///
/// Events
///

export async function getEvents(
	limit: number | null,
	offset: number | null,
	abort: AbortSignal,
): Promise<SaltEvent[]> {
	const args = new URLSearchParams();

	if (limit) args.append('limit', limit.toString());
	if (offset) args.append('offset', offset.toString());

	return sendRequest('GET', `/events?${args.toString()}`, u, abort).then((data: unknown) =>
		(data as unknown[]).map((e) => SaltEvent.fromObject(e)),
	);
}

///
/// Jobs
///

export async function getJobs(
	sort: string | null,
	limit: number | null,
	offset: number | null,
	abort: AbortSignal,
): Promise<Job[]> {
	const args = new URLSearchParams();

	if (sort) args.append('sort', sort);
	if (limit) args.append('limit', limit.toString());
	if (offset) args.append('offset', offset.toString());

	return sendRequest('GET', `/jobs?${args.toString()}`, u, abort).then((data: unknown) =>
		(data as unknown[]).map((j) => Job.fromObject(j)),
	);
}

export async function runJob(command: RunCommand, abort: AbortSignal): Promise<unknown> {
	return sendRequest(
		'POST',
		'/jobs',
		{
			client: command.client,
			tgtType: command.targetType,
			tgt: command.target,
			fun: command.fun,
			arg: command.arg,
			kwarg: Object.fromEntries(command.kwarg), // Map<>'s are invisible to JSON.stringify
			batchSize: command.batchSize,
		},
		abort,
	);
}

export async function getJobById(jobId: string, abort: AbortSignal): Promise<Job> {
	return sendRequest('GET', `/jobs/${jobId}`, u, abort).then((data: unknown) =>
		Job.fromObject(data),
	);
}

///
/// Keys
///

export async function getKeys(abort: AbortSignal): Promise<Key[]> {
	return sendRequest('GET', '/keys', u, abort).then((data: unknown) =>
		(data as unknown[]).map((k) => Key.fromObject(k)),
	);
}

export async function acceptKey(key: Key, abort: AbortSignal): Promise<void> {
	await sendRequest('PUT', `/keys/${key.state}/${key.id}/accept`, u, abort);
}

export async function rejectKey(key: Key, abort: AbortSignal): Promise<void> {
	await sendRequest('PUT', `/keys/${key.state}/${key.id}/reject`, u, abort);
}

export async function deleteKey(key: Key, abort: AbortSignal): Promise<void> {
	await sendRequest('DELETE', `/keys/${key.state}/${key.id}/delete`, u, abort);
}

///
/// Permission Groups
///

export async function getPermissionGroups(
	limit: number | null,
	offset: number | null,
	abort: AbortSignal,
): Promise<PermissionGroup[]> {
	const args = new URLSearchParams();

	if (limit) args.append('limit', limit.toString());
	if (offset) args.append('offset', offset.toString());

	return sendRequest('GET', `/permissions?${args.toString()}`, u, abort).then((data: unknown) =>
		(data as unknown[]).map((p) => PermissionGroup.fromObject(p)),
	);
}

export async function getPermissionGroup(id: string, abort: AbortSignal): Promise<PermissionGroup> {
	return sendRequest('GET', `/permissions/${id}`, u, abort).then((data: unknown) =>
		PermissionGroup.fromObject(data),
	);
}

export async function createPermissionGroup(
	name: string,
	abort: AbortSignal,
): Promise<PermissionGroup> {
	return sendRequest(
		'POST',
		'/permissions',
		{
			name,
		},
		abort,
	).then((data: unknown) => PermissionGroup.fromObject(data));
}

export async function deletePermissionGroup(id: string, abort: AbortSignal): Promise<void> {
	await sendRequest('DELETE', `/permissions/${id}`, u, abort);
}

export async function updatePermissionGroup(
	id: string,
	name: string,
	perms: fPerm[],
	abort: AbortSignal,
): Promise<void> {
	await sendRequest(
		'PUT',
		`/permissions/${id}`,
		{
			name,
			perms: JSON.stringify(perms),
		},
		abort,
	);
}

///
/// Settings
///

export async function getExport(abort: AbortSignal): Promise<unknown> {
	return sendRequest('GET', '/settings/export', u, abort);
}

export async function importData(data: unknown, abort: AbortSignal): Promise<void> {
	await sendRequest('POST', '/settings/import', data, abort);
}
