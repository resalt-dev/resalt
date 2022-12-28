<script lang="ts">
	import paths from '../../paths';
	import Tabs from '../../components/Tabs.svelte';
	import type { NavigateFn } from 'svelte-navigator';
	import type TabPage from '../../models/TabPage';

	import DashboardTabInfo from './DashboardTabInfo.svelte';
	import DashboardTabMetrics from './DashboardTabMetrics.svelte';
	import DashboardTabReports from './DashboardTabReports.svelte';

	// svelte-ignore unused-export-let
	export let location: Location;
	// svelte-ignore unused-export-let
	export let navigate: NavigateFn;
	export let subPage: string = '';

	let tabs: TabPage[] = [];
	$: tabs = [
		{
			key: 'info',
			label: 'General',
			path: paths.dashboard.getPath(),
			component: DashboardTabInfo,
		},
		{
			key: 'reports',
			label: 'Reports',
			path: paths.dashboard.getPath('reports'),
			component: DashboardTabReports,
			data: {
				report: subPage.split('/')[1] || null,
			},
		},
		{
			key: 'metrics',
			label: 'Metrics (old)',
			path: paths.dashboard.getPath('metrics'),
			component: DashboardTabMetrics,
		},
	];
</script>

<Tabs {tabs} current={subPage.split('/')[0]} />
