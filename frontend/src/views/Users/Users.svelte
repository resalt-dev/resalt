<script lang="ts">
	import type { NavigateFn } from 'svelte-navigator';
	import paths from '../../paths';
	import Tabs from '../../components/Tabs.svelte';
	import type TabPage from '../../models/TabPage';

	import UsersTabList from './UsersTabList.svelte';
	import UsersTabCreate from './UsersTabCreate.svelte';
	import { hasResaltPermission, P_USER_ADMIN, P_USER_LIST } from '../../perms';
	import { currentUser } from '../../stores';

	// svelte-ignore unused-export-let
	export let location: Location;
	export let navigate: NavigateFn;
	export let subPage: string = '';

	let tabs: TabPage[] = [];
	$: tabs = [
		{
			key: 'list',
			label: 'List',
			path: paths.users.getPath('list'),
			component: UsersTabList,
			data: { navigate },
			hidden: !hasResaltPermission($currentUser.perms, P_USER_LIST),
		},
		{
			key: 'create',
			label: 'Create',
			path: paths.users.getPath('create'),
			component: UsersTabCreate,
			data: { navigate },
			hidden: !hasResaltPermission($currentUser.perms, P_USER_ADMIN),
		},
	];
</script>

<Tabs {tabs} current={subPage} />
