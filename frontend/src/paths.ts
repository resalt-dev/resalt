import {
	P_EVENT_LIST,
	P_JOB_LIST,
	P_MINION_LIST,
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

	hasParams: boolean;

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
		if (icon === null) {
			if (perms !== null) {
				throw new Error('A path with no icon must have no permissions');
			}
		}
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

		// Trim trailing slashes
		return path.replace(/\/+$/, '');
	}

	hasPermission(userPermissions: any[]): boolean {
		if (this.perms === undefined || this.perms === null || this.perms.length === 0) {
			return true;
		}

		for (const perm of this.perms) {
			if (hasResaltPermission(userPermissions, perm)) {
				return true;
			}
		}

		return false;
	}
}

const paths = {
	login: new Path(0, 'login', '/auth/login', 'Login', null, null),

	dashboard: new Path(10, 'dashboard', '/dashboard/:subPage', 'Dashboard', 'home', null),

	run: new Path(21, 'run', '/run/:subPage', 'Run', 'play', [
		P_RUN_LIVE,
		P_RUN_APPROVAL_LIST,
		P_RUN_APPROVAL_SUBMIT,
		P_RUN_TEMPLATE_LIST,
		P_RUN_TEMPLATE_LOCAL,
		P_RUN_TEMPLATE_GLOBAL,
	]),

	minion: new Path(30, 'minion', '/minion/:minionId/:subPage', 'Minion', null, null),
	minions: new Path(31, 'minions', '/minions/:subPage', 'Minions', 'server', [P_MINION_LIST]),

	job: new Path(40, 'job', '/job/:jobId', 'Job', null, null),
	jobs: new Path(41, 'jobs', '/jobs', 'Jobs', 'briefcase', [P_JOB_LIST]),

	events: new Path(50, 'events', '/events', 'Events', 'list-ul', [P_EVENT_LIST]),

	keys: new Path(60, 'keys', '/keys', 'Keys', 'lock', [P_SALTKEY_LIST]),

	_1: new Path(99, '_', '/_', '', '', null),

	user: new Path(100, 'user', '/user/:userId', 'User', null, null),
	users: new Path(101, 'users', '/users/:usersPage', 'Users', 'user-circle', [
		P_USER_ADMIN,
		P_USER_LIST,
	]),

	settings: new Path(110, 'settings', '/settings/:settingsPage', 'Settings', 'cog', []),

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

export default paths;
