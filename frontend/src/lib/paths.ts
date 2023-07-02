import type User from '../models/User';
import {
	P_ADMIN_GROUP,
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

export class Path {
	order: number;

	name: string;

	private path: string;

	label: string;

	icon: string;

	showInNav: boolean;

	private perms?: string[] | null;

	constructor(
		order: number,
		name: string,
		path: string,
		label: string,
		icon: string | null,
		perms: string[] | null,
	) {
		this.order = order;
		this.name = name;
		this.path = path;
		this.label = label;
		this.icon = icon || '';
		this.showInNav = icon !== null;
		this.perms = perms;
	}

	getPath(...args: string[]): string {
		let { path } = this;

		// Substitute url arguments (.e.g ":id" or ":group") with args
		path = path.replace(/:([^/]+)/g, () => args.shift() || '');

		if (args.length > 0) {
			throw new Error(`Too many arguments for path ${this.name}: ${args}`);
		}

		// Trim trailing slashes
		return path.replace(/\/+$/, '');
	}

	getBarePath(): string {
		// Split by :, take first, and remove trailing slash
		return this.path.split(':')[0].replace(/\/+$/, '');
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
	login: new Path(0, 'login', '/login', 'Login', null, null),

	dashboard: new Path(10, 'dashboard', '/dashboard', 'Dashboard', 'home', null),

	run: new Path(20, 'run', '/run/:subPage', 'Run', 'play', [
		P_RUN_LIVE,
		P_RUN_APPROVAL_LIST,
		P_RUN_APPROVAL_SUBMIT,
		P_RUN_TEMPLATE_LIST,
		P_RUN_TEMPLATE_LOCAL,
		P_RUN_TEMPLATE_GLOBAL,
	]),

	minion: new Path(30, 'minion', '/minion/:minionId', 'Minion', null, null),
	minion_grains: new Path(31, 'minion_grains', '/minion/:minionId/grains', 'Grains', null, null),
	minion_conformity: new Path(
		32,
		'minion_conformity',
		'/minion/:minionId/conformity',
		'Conformity',
		null,
		[P_MINION_CONFORMITY],
	),
	minion_pillars: new Path(33, 'minion_pillars', '/minion/:minionId/pillars', 'Pillars', null, [
		P_MINION_PILLARS,
	]),
	minion_packages: new Path(
		34,
		'minion_packages',
		'/minion/:minionId/packages',
		'Packages',
		null,
		[P_MINION_PACKAGES],
	),

	minions: new Path(40, 'minions', '/minions', 'Minions', 'server', [P_MINION_LIST]),
	minions_presets: new Path(
		41,
		'minions_presets',
		'/minions/presets/:selected',
		'Presets',
		null,
		[P_MINION_PRESETS_LIST],
	),
	minions_grains: new Path(42, 'minions_grains', '/minions/grains', 'Grains', null, [
		P_MINION_GRAINEXPLORER,
	]),

	job: new Path(50, 'job', '/job/:jobId', 'Job', null, null),

	jobs: new Path(60, 'jobs', '/jobs', 'Jobs', 'briefcase', [P_JOB_LIST]),

	events: new Path(70, 'events', '/events', 'Events', 'list-ul', [P_EVENT_LIST]),

	keys: new Path(80, 'keys', '/keys', 'Keys', 'lock', [P_SALTKEY_LIST]),

	_1: new Path(99, '_', '/_', '', '', null),

	user: new Path(100, 'user', '/user/:userId', 'User', null, null),

	users_list: new Path(110, 'users_list', '/users', 'Users', 'user-circle', [
		P_USER_LIST,
	]),
	users_new: new Path(110, 'users_new', '/users/:usersPage', 'Create New', null, [
		P_USER_ADMIN,
	]),

	settings_config: new Path(120, 'settings_config', '/settings', 'Settings', 'cog', []), // Config
	settings_groups: new Path(121, 'settings_groups', '/settings/groups', 'Groups', null, [
		P_ADMIN_GROUP,
	]),

	preferences: new Path(
		120,
		'preferences',
		'/preferences/:preferencesPage',
		'Preferences',
		'wrench',
		null,
	),

	// _2: new Path(999, '_', '/_', '', '', null),

	notFound: new Path(1000, 'notFound', '/not-found', 'Not Found', null, null),
};

export const getPathByName = (name: string): Path | undefined => {
	return Object.values(paths).find((p) => p.name === name);
};

export default paths;
