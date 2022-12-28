<script lang="ts">
	import { currentUser } from '../../stores';
	import { hasResaltPermission, P_ADMIN_GROUP } from '../../perms';
	import paths from '../../paths';
	import Tabs from '../../components/Tabs.svelte';
	import type TabPage from '../../models/TabPage';
	import type { NavigateFn } from 'svelte-navigator';

	import SettingsTabConfig from './SettingsTabConfig.svelte';
	import SettingsTabGroups from './SettingsTabGroups.svelte';

	// svelte-ignore unused-export-let
	export let location: Location;
	// svelte-ignore unused-export-let
	export let navigate: NavigateFn;
	export let subPage: string = '';

	let tabs: TabPage[] = [];
	$: tabs = [
		{
			key: 'config',
			label: 'Config',
			path: paths.settings.getPath('config'),
			component: SettingsTabConfig,
		},
		{
			key: 'groups',
			label: 'Groups',
			path: paths.settings.getPath('groups'),
			component: SettingsTabGroups,
			hidden: !hasResaltPermission($currentUser.perms, P_ADMIN_GROUP),
		},
	];
</script>

<Tabs {tabs} current={subPage} />
