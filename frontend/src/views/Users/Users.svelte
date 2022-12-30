<script lang="ts">
	import type { NavigateFn } from 'svelte-navigator';
	import paths from '../../paths';
	import Tabs from '../../components/Tabs.svelte';
	import type TabPage from '../../models/TabPage';

	import UsersTabList from './UsersTabList.svelte';
	import UsersTabCreate from './UsersTabCreate.svelte';

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
		},
		{
			key: 'create',
			label: 'Create',
			path: paths.users.getPath('create'),
			component: UsersTabCreate,
			data: { navigate },
		},
	];
</script>

<Tabs {tabs} current={subPage} />
