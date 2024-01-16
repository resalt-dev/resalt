/* eslint-disable @typescript-eslint/ban-types */
import {
	BookInformationFilled,
	BookInformationRegular,
	CubeFilled,
	CubeRegular,
	EmojiSparkleRegular,
	FluentIcon,
	GroupFilled,
	GroupRegular,
	ListFilled,
	ListRegular,
	LockClosedKeyFilled,
	LockClosedKeyRegular,
	PasswordFilled,
	PasswordRegular,
	PeopleTeamFilled,
	PeopleTeamRegular,
	PersonKeyFilled,
	PersonKeyRegular,
	ServerFilled,
	ServerRegular,
	SignOutFilled,
	SignOutRegular,
	TaskListSquareRtlFilled,
	TaskListSquareRtlRegular,
	TasksAppFilled,
	TasksAppRegular,
	bundleIcon,
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

const LoginIcon = bundleIcon(PersonKeyFilled, PersonKeyRegular);
const LogoutIcon = bundleIcon(SignOutFilled, SignOutRegular);
const MinionIcon = bundleIcon(ServerFilled, ServerRegular);
const GrainsIcon = bundleIcon(BookInformationFilled, BookInformationRegular);
const ConformityIcon = bundleIcon(TaskListSquareRtlFilled, TaskListSquareRtlRegular);
const PillarsIcon = bundleIcon(PasswordFilled, PasswordRegular);
const PackagesIcon = bundleIcon(CubeFilled, CubeRegular);
const JobsIcon = bundleIcon(TasksAppFilled, TasksAppRegular);
const EventsIcon = bundleIcon(ListFilled, ListRegular);
const UsersIcon = bundleIcon(PeopleTeamFilled, PeopleTeamRegular);
const GroupIcon = bundleIcon(GroupFilled, GroupRegular);
const PermissionsIcon = bundleIcon(LockClosedKeyFilled, LockClosedKeyRegular);

export const paths = {
	login: new Path('/login', 'Login', LoginRoute, LoginIcon),
	logout: new Path('/logout', 'Logout', LogoutRoute, LogoutIcon),
	dashboard: new Path('/', 'Dashboard', DashboardRoute, null),
	minions: new Path('/minions', 'Minions', MinionsRoute, MinionIcon),
	minion: new Path('/minions/:minionId', 'Minion', MinionRoute, MinionIcon),
	minion_grains: new Path('/minions/:minionId/grains', 'Grains', MinionGrainsRoute, GrainsIcon),
	minion_conformity: new Path(
		'/minions/:minionId/conformity',
		'Conformity',
		MinionConformityRoute,
		ConformityIcon,
	),
	minion_pillars: new Path(
		'/minions/:minionId/pillars',
		'Pillars',
		MinionPillarsRoute,
		PillarsIcon,
	),
	minion_packages: new Path(
		'/minions/:minionId/packages',
		'Packages',
		MinionPackagesRoute,
		PackagesIcon,
	),
	grains: new Path('/grains', 'Grains', GrainsRoute, GrainsIcon),
	pillars: new Path('/pillars', 'Pillars', PillarsRoute, PillarsIcon),
	packages: new Path('/packages', 'Packages', PackagesRoute, PackagesIcon),
	jobs: new Path('/jobs', 'Jobs', JobsRoute, JobsIcon),
	job: new Path('/jobs/:jobId', 'Job', JobRoute, null),
	events: new Path('/events', 'Events', EventsRoute, EventsIcon),
	event: new Path('/events/:eventId', 'Event', EventRoute, null),
	users: new Path('/users', 'Users', UsersRoute, UsersIcon),
	user: new Path('/users/:userId', 'User', UserRoute, null),
	groups: new Path('/groups', 'Groups', GroupsRoute, GroupIcon),
	group: new Path('/groups/:groupId', 'Group', GroupRoute, null),
	permissions: new Path('/permissions', 'Permissions', PermissionsRoute, PermissionsIcon),
	// // settings: new Path('/settings', 'Settings', null, ServerIcon),
};

export const sidebar: { title: string; shortTitle: string; items: Path[] }[] = [
	{
		title: 'Minions',
		shortTitle: 'Min',
		items: [paths.minions, paths.grains, paths.pillars, paths.packages],
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
