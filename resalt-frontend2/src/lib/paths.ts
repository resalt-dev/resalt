/* eslint-disable @typescript-eslint/ban-types */
import {
	AppsFilled,
	AppsRegular,
	BookInformationFilled,
	BookInformationRegular,
	FluentIcon,
	GroupFilled,
	GroupRegular,
	HomeFilled,
	HomeRegular,
	ListFilled,
	ListRegular,
	LockClosedKeyFilled,
	LockClosedKeyRegular,
	PeopleTeamFilled,
	PeopleTeamRegular,
	PersonKeyFilled,
	PersonKeyRegular,
	ReceiptPlayFilled,
	ReceiptPlayRegular,
	ServerFilled,
	ServerRegular,
	SignOutFilled,
	SignOutRegular,
	TaskListSquareRtlFilled,
	TaskListSquareRtlRegular,
	TasksAppFilled,
	TasksAppRegular,
	VaultFilled,
	VaultRegular,
	WindowPlayFilled,
	WindowPlayRegular,
	bundleIcon,
} from '@fluentui/react-icons';
import { PathParam, generatePath } from 'react-router-dom';
import DashboardRoute from '../routes/dashboard/DashboardRoute';
import DeployRoute from '../routes/deploy/DeployRoute';
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
import MinionTerminalRoute from '../routes/minions/:minionId/terminal/MinionTerminalRoute';
import MinionsRoute from '../routes/minions/MinionsRoute';
import PackagesRoute from '../routes/packages/PackagesRoute';
import PermissionsRoute from '../routes/permissions/PermissionsRoute';
import PillarsRoute from '../routes/pillars/PillarsRoute';
import UserRoute from '../routes/users/:userId/UserRoute';
import UsersRoute from '../routes/users/UsersRoute';

export type PathParams = {
	[key in PathParam<string>]: string;
};
export class Path<T extends PathParams = PathParams> {
	path: string;

	name: string;

	element: Function;

	Icon: FluentIcon;

	constructor(path: string, name: string, element: Function, icon: FluentIcon) {
		this.path = path;
		this.name = name;
		this.element = element;
		this.Icon = icon;
	}

	getPath(params: T): string;
	getPath(): string;
	getPath(params?: T): string {
		return generatePath(this.path, params);
	}
}

const LoginIcon = bundleIcon(PersonKeyFilled, PersonKeyRegular);
const LogoutIcon = bundleIcon(SignOutFilled, SignOutRegular);
const DashboardIcon = bundleIcon(HomeFilled, HomeRegular);
const MinionIcon = bundleIcon(ServerFilled, ServerRegular);
const GrainsIcon = bundleIcon(BookInformationFilled, BookInformationRegular);
const ConformityIcon = bundleIcon(TaskListSquareRtlFilled, TaskListSquareRtlRegular);
const PillarsIcon = bundleIcon(VaultFilled, VaultRegular);
const PackagesIcon = bundleIcon(AppsFilled, AppsRegular);
const DeployIcon = bundleIcon(ReceiptPlayFilled, ReceiptPlayRegular);
const TerminalIcon = bundleIcon(WindowPlayFilled, WindowPlayRegular);
const JobsIcon = bundleIcon(TasksAppFilled, TasksAppRegular);
const EventsIcon = bundleIcon(ListFilled, ListRegular);
const UsersIcon = bundleIcon(PeopleTeamFilled, PeopleTeamRegular);
const GroupIcon = bundleIcon(GroupFilled, GroupRegular);
const PermissionsIcon = bundleIcon(LockClosedKeyFilled, LockClosedKeyRegular);

export const paths = {
	login: new Path<{}>('/login', 'Login', LoginRoute, LoginIcon),
	logout: new Path<{}>('/logout', 'Logout', LogoutRoute, LogoutIcon),
	dashboard: new Path<{}>('/', 'Dashboard', DashboardRoute, DashboardIcon),
	minions: new Path<{}>('/minions', 'Minions', MinionsRoute, MinionIcon),
	minion: new Path<{ minionId: string }>('/minions/:minionId', 'Minion', MinionRoute, MinionIcon),
	minion_grains: new Path<{ minionId: string }>(
		'/minions/:minionId/grains',
		'Grains',
		MinionGrainsRoute,
		GrainsIcon,
	),
	minion_conformity: new Path<{ minionId: string }>(
		'/minions/:minionId/conformity',
		'Conformity',
		MinionConformityRoute,
		ConformityIcon,
	),
	minion_pillars: new Path<{ minionId: string }>(
		'/minions/:minionId/pillars',
		'Pillars',
		MinionPillarsRoute,
		PillarsIcon,
	),
	minion_packages: new Path<{ minionId: string }>(
		'/minions/:minionId/packages',
		'Packages',
		MinionPackagesRoute,
		PackagesIcon,
	),
	minion_terminal: new Path<{ minionId: string }>(
		'/minions/:minionId/terminal',
		'Terminal',
		MinionTerminalRoute,
		TerminalIcon,
	),
	grains: new Path<{}>('/grains', 'Grains', GrainsRoute, GrainsIcon),
	pillars: new Path<{}>('/pillars', 'Pillars', PillarsRoute, PillarsIcon),
	packages: new Path<{}>('/packages', 'Packages', PackagesRoute, PackagesIcon),
	deploy: new Path<{}>('/deploy', 'Deploy', DeployRoute, DeployIcon),
	jobs: new Path<{}>('/jobs', 'Jobs', JobsRoute, JobsIcon),
	job: new Path<{ jobId: string }>('/jobs/:jobId', 'Job', JobRoute, JobsIcon),
	events: new Path<{}>('/events', 'Events', EventsRoute, EventsIcon),
	event: new Path<{ eventId: string }>('/events/:eventId', 'Event', EventRoute, EventsIcon),
	users: new Path<{}>('/users', 'Users', UsersRoute, UsersIcon),
	user: new Path<{ userId: string }>('/users/:userId', 'User', UserRoute, UsersIcon),
	groups: new Path<{}>('/groups', 'Groups', GroupsRoute, GroupIcon),
	group: new Path<{ groupId: string }>('/groups/:groupId', 'Group', GroupRoute, GroupIcon),
	permissions: new Path<{}>('/permissions', 'Permissions', PermissionsRoute, PermissionsIcon),
	// // settings: new Path('/settings', 'Settings', null, ServerIcon),
};

export const sidebar: { title: string; shortTitle: string; items: Path[] }[] = [
	{
		title: 'Minions',
		shortTitle: 'Min',
		items: [paths.minions, paths.grains, paths.pillars, paths.packages, paths.deploy],
	},
	{
		title: 'Monitoring',
		shortTitle: 'Mon',
		items: [paths.jobs, paths.events],
	},
	{
		title: 'Access Control',
		shortTitle: 'AC',
		items: [paths.users, paths.groups, paths.permissions],
	},
];
