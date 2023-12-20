import {
	CubeRegular,
	EmojiSparkleRegular,
	FluentIcon,
	GroupRegular,
	InfoRegular,
	LaptopRegular,
	ListRegular,
	LockClosedKeyRegular,
	PasswordRegular,
	PeopleTeamRegular,
	PersonKeyRegular,
	ServerRegular,
	SignOutRegular,
	TaskListSquareRtlRegular,
	TasksAppRegular,
} from '@fluentui/react-icons';

// export class Path<T = object> {
export class Path {
	path: string;

	name: string;

	Icon: FluentIcon;

	constructor(path: string, name: string, icon: FluentIcon | null) {
		this.path = path;
		this.name = name;
		this.Icon = icon ?? EmojiSparkleRegular;
	}
}

export const paths = {
	login: new Path('/login', 'Login', PersonKeyRegular),
	logout: new Path('/logout', 'Logout', SignOutRegular),

	dashboard: new Path('/', 'Dashboard', null),

	minions: new Path('/minions', 'Minions', LaptopRegular),
	minion: new Path('/minions/:minionId', 'Minion', ServerRegular),
	minion_conformity: new Path(
		'/minions/:minionId/conformity',
		'Conformity',
		TaskListSquareRtlRegular,
	),
	minion_grains: new Path('/minions/:minionId/grains', 'Grains', InfoRegular),
	minion_pillars: new Path('/minions/:minionId/pillars', 'Pillars', PasswordRegular),
	minion_packages: new Path('/minions/:minionId/packages', 'Packages', CubeRegular),

	grains: new Path('/grains', 'Grains', InfoRegular),
	pillars: new Path('/pillars', 'Pillars', PasswordRegular),
	packages: new Path('/packages', 'Packages', CubeRegular),

	jobs: new Path('/jobs', 'Jobs', TasksAppRegular),
	job: new Path('/jobs/:jobId', 'Job', null),

	events: new Path('/events', 'Events', ListRegular),
	event: new Path('/events/:eventId', 'Event', null),

	users: new Path('/users', 'Users', PeopleTeamRegular),
	user: new Path('/users/:userId', 'User', null),
	groups: new Path('/groups', 'Groups', GroupRegular),
	group: new Path('/groups/:groupId', 'Group', null),
	permissions: new Path('/permissions', 'Permissions', LockClosedKeyRegular),

	settings: new Path('/settings', 'Settings', ServerRegular),
};

export const sidebar: { title: string; items: Path[] }[] = [
	{
		title: 'Minions',
		items: [paths.minions, paths.grains, paths.pillars, paths.packages],
	},
	{
		title: 'Monitoring',
		items: [paths.jobs, paths.events],
	},
	{
		title: 'Access Control',
		items: [paths.users, paths.groups, paths.permissions],
	},
];

export default paths;
