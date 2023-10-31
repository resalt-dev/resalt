import type User from '$model/User';
import {
	P_ADMIN_GROUP,
	P_ADMIN_SUPERADMIN,
	P_EVENT_LIST,
	P_JOB_LIST,
	P_MINION_CONFORMITY,
	P_MINION_GRAINEXPLORER,
	P_MINION_LIST,
	P_MINION_PACKAGES,
	P_MINION_PILLARS,
	P_MINION_PRESETS_LIST,
	P_RUN_APPROVAL_LIST,
	P_RUN_APPROVAL_SUBMIT,
	P_RUN_LIVE,
	P_RUN_TEMPLATE_GLOBAL,
	P_RUN_TEMPLATE_LIST,
	P_RUN_TEMPLATE_LOCAL,
	P_SALTKEY_LIST,
	P_USER_ADMIN,
	P_USER_LIST,
	hasResaltPermission,
} from './perms';

// Type parameter T is used to define the required params for the path
// T MUST be an object with string properties
// e.g. {userId: string} or {minionId: string}
export class Path<T = {}> {
	order: number;

	private path: string;

	label: string;

	icon: string;

	showInNav: boolean;

	private perms?: string[] | null;

	constructor(
		order: number,
		path: string,
		label: string,
		icon: string | null,
		perms: string[] | null,
	) {
		this.order = order;
		this.path = path;
		this.label = label;
		this.icon = icon || '';
		this.showInNav = icon !== null;
		this.perms = perms;
	}

	getRawPath(): string {
		return this.path;
	}

	getPath(params?: T | Record<string, string>, searchParams?: Record<string, string>): string {
		console.log(this.path, params, searchParams);
		let { path } = this;

		// Check if any params are REQUIRED (match \/\[([A-Za-z]+])\] in path)
		const requiredParams = path.match(/\/\[([A-Za-z]+])\]/g);
		if (requiredParams !== null) {
			if (typeof params !== 'object' || params === null) {
				throw new Error(`Path "${this.label}" requires params, but none were provided`);
			}

			// Check if all required params are present
			for (const param of requiredParams) {
				const key = param.replace(/\/\[(.+)]/, '$1');
				if ((params as Record<string, unknown>)[key] === undefined) {
					throw new Error(
						`Path "${this.label}" requires param "${key}", but it was not provided`,
					);
				}
			}
		}

		// If params is an object, substitute url arguments (.e.g "[userId]" or "[minionId]") with params
		if (typeof params === 'object' && params !== null) {
			path = path.replace(
				/\[([A-Za-z]+)\]/g,
				(_, key) => (params as Record<string, unknown>)[key] + '' || '',
			);
		}

		// If searchParams is an object, add query string to url
		if (typeof searchParams === 'object' && searchParams !== null) {
			const search = new URLSearchParams(searchParams);
			path += `?${search.toString()}`;
		}

		// Trim trailing slashes
		return path.replace(/\/+$/, '');
	}

	hasPermission(user: User | null): boolean {
		if (user === null) {
			return false;
		}
		if (this.perms === undefined || this.perms === null || this.perms.length === 0) {
			return true;
		}

		for (const perm of this.perms) {
			if (hasResaltPermission(user, perm)) {
				return true;
			}
		}

		return false;
	}
}

const paths = {
	login: new Path(0, '/login', 'Login', null, null),
	logout: new Path(1, '/logout', 'Logout', null, null),

	dashboard: new Path(10, '/dashboard', 'Dashboard', 'home', null),

	run: new Path(20, '/run', 'Run', 'play', [
		P_RUN_LIVE,
		P_RUN_APPROVAL_LIST,
		P_RUN_APPROVAL_SUBMIT,
		P_RUN_TEMPLATE_LIST,
		P_RUN_TEMPLATE_LOCAL,
		P_RUN_TEMPLATE_GLOBAL,
	]),

	minion: new Path<{ minionId: string }>(30, '/minion/[minionId]', 'Minion', null, null),
	minion_grains: new Path<{ minionId: string }>(
		31,
		'/minion/[minionId]/grains',
		'Grains',
		null,
		null,
	),
	minion_conformity: new Path(32, '/minion/[minionId]/conformity', 'Conformity', null, [
		P_MINION_CONFORMITY,
	]),
	minion_pillars: new Path(33, '/minion/[minionId]/pillars', 'Pillars', null, [P_MINION_PILLARS]),
	minion_packages: new Path(34, '/minion/[minionId]/packages', 'Packages', null, [
		P_MINION_PACKAGES,
	]),

	minions: new Path(40, '/minions', 'Minions', 'server', [P_MINION_LIST]),
	minions_presets: new Path<{ presetId?: string }>(
		41,
		'/minions/presets/[[presetId]]',
		'Presets',
		null,
		[P_MINION_PRESETS_LIST],
	),
	minions_grains: new Path(42, '/minions/grains', 'Grains', null, [P_MINION_GRAINEXPLORER]),

	job: new Path<{ jobId: string }>(50, '/job/[jobId]', 'Job', null, null),

	jobs: new Path(60, '/jobs', 'Jobs', 'briefcase', [P_JOB_LIST]),

	events: new Path(70, '/events', 'Events', 'list-ul', [P_EVENT_LIST]),

	keys: new Path(80, '/keys', 'Keys', 'lock', [P_SALTKEY_LIST]),

	_1: new Path(99, '/_', '_', '', null),

	user_info: new Path<{ userId: string }>(100, '/user/[userId]', 'User', null, null),
	user_security: new Path<{ userId: string }>(
		101,
		'/user/[userId]/security',
		'Security',
		null,
		null,
	),
	user_permissions: new Path<{ userId: string }>(
		102,
		'/user/[userId]/permissions',
		'Permissions',
		null,
		null,
	),

	users_list: new Path(110, '/users', 'Users', 'user-circle', [P_USER_LIST]),
	users_add: new Path(110, '/users/add', 'Add user', null, [P_USER_ADMIN]),

	settings_config: new Path(120, '/settings', 'Settings', 'cog', []), // Config
	settings_groups: new Path(121, '/settings/groups', 'Groups', null, [P_ADMIN_GROUP]),
	settings_export: new Path(122, '/settings/export', 'Export', null, [P_ADMIN_SUPERADMIN]),
	settings_import: new Path(123, '/settings/import', 'Import', null, [P_ADMIN_SUPERADMIN]),

	preferences: new Path(120, '/preferences', 'Preferences', 'wrench', null),

	// _2: new Path(999, '/_', '_', '', null),

	notFound: new Path(1000, '/not-found', 'Not Found', null, null),
};

export default paths;
