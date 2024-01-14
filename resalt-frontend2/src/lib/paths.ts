/* eslint-disable @typescript-eslint/ban-types */
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
import DashboardRoute from '../routes/dashboard/DashboardRoute';
import EventRoute from '../routes/events/:eventId/EventRoute';
import EventsRoute from '../routes/events/EventsRoute';
import GrainsRoute from '../routes/grains/GrainsRoute';
import GroupRoute from '../routes/groups/:groupId/GroupRoute';
import GroupsRoute from '../routes/groups/GroupsRoute';
import JobRoute from '../routes/jobs/:jobId/JobRoute';
import JobsRoute from '../routes/jobs/JobsRoute';
import LoginRoute from '../routes/login/LoginRoute';
import LogoutRoute from '../routes/logout/LogoutRoute';
import MinionRoute from '../routes/minions/:minionId/MinionRoute';
import MinionConformityRoute from '../routes/minions/:minionId/conformity/MinionConformityRoute';
import MinionGrainsRoute from '../routes/minions/:minionId/grains/MinionGrainsRoute';
import MinionPackagesRoute from '../routes/minions/:minionId/packages/MinionPackagesRoute';
import MinionPillarsRoute from '../routes/minions/:minionId/pillars/MinionPillarsRoute';
import MinionsRoute from '../routes/minions/MinionsRoute';
import PackagesRoute from '../routes/packages/PackagesRoute';
import PermissionsRoute from '../routes/permissions/PermissionsRoute';
import PillarsRoute from '../routes/pillars/PillarsRoute';
import UserRoute from '../routes/users/:userId/UserRoute';
import UsersRoute from '../routes/users/UsersRoute';

// export class Path<T = object> {
export class Path {
	path: string;

	name: string;

	element: Function;

	Icon: FluentIcon;

	constructor(path: string, name: string, element: Function, icon: FluentIcon | null) {
		this.path = path;
		this.name = name;
		this.element = element;
		this.Icon = icon ?? EmojiSparkleRegular;
	}
}

export const paths = {
	login: new Path('/login', 'Login', LoginRoute, PersonKeyRegular),
	logout: new Path('/logout', 'Logout', LogoutRoute, SignOutRegular),
	dashboard: new Path('/', 'Dashboard', DashboardRoute, null),
	minions: new Path('/minions', 'Minions', MinionsRoute, LaptopRegular),
	minion: new Path('/minions/:minionId', 'Minion', MinionRoute, ServerRegular),
	minion_grains: new Path('/minions/:minionId/grains', 'Grains', MinionGrainsRoute, InfoRegular),
	minion_conformity: new Path(
		'/minions/:minionId/conformity',
		'Conformity',
		MinionConformityRoute,
		TaskListSquareRtlRegular,
	),
	minion_pillars: new Path(
		'/minions/:minionId/pillars',
		'Pillars',
		MinionPillarsRoute,
		PasswordRegular,
	),
	minion_packages: new Path(
		'/minions/:minionId/packages',
		'Packages',
		MinionPackagesRoute,
		CubeRegular,
	),
	grains: new Path('/grains', 'Grains', GrainsRoute, InfoRegular),
	pillars: new Path('/pillars', 'Pillars', PillarsRoute, PasswordRegular),
	packages: new Path('/packages', 'Packages', PackagesRoute, CubeRegular),
	jobs: new Path('/jobs', 'Jobs', JobsRoute, TasksAppRegular),
	job: new Path('/jobs/:jobId', 'Job', JobRoute, null),
	events: new Path('/events', 'Events', EventsRoute, ListRegular),
	event: new Path('/events/:eventId', 'Event', EventRoute, null),
	users: new Path('/users', 'Users', UsersRoute, PeopleTeamRegular),
	user: new Path('/users/:userId', 'User', UserRoute, null),
	groups: new Path('/groups', 'Groups', GroupsRoute, GroupRegular),
	group: new Path('/groups/:groupId', 'Group', GroupRoute, null),
	permissions: new Path('/permissions', 'Permissions', PermissionsRoute, LockClosedKeyRegular),
	// // settings: new Path('/settings', 'Settings', null, ServerRegular),
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
