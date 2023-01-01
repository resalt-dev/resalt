<script lang="ts">
	import { writable, type Writable } from 'svelte/store';
	import Tabs from '../../components/Tabs.svelte';
	import type Filter from '../../models/Filter';
	import type TabPage from '../../models/TabPage';
	import paths from '../../paths';

	import type { NavigateFn } from 'svelte-navigator';
	import { FilterFieldType } from '../../models/FilterFieldType';
	import { FilterOperand } from '../../models/FilterOperand';
	import MinionsTabGrains from './MinionsTabGrains.svelte';
	import MinionsTabPresets from './MinionsTabPresets.svelte';
	import MinionsTabSearch from './MinionsTabSearch.svelte';
	import {
		hasResaltPermission,
		P_MINION_GRAINEXPLORER,
		P_MINION_LIST,
		P_MINION_PRESETS_LIST,
	} from '../../perms';
	import { currentUser } from '../../stores';

	// svelte-ignore unused-export-let
	export let location: Location;
	export let navigate: NavigateFn;
	export let subPage: string = '';

	const filters: Writable<Filter[]> = writable([
		{
			fieldType: FilterFieldType.NONE,
			field: '',
			operand: FilterOperand.CONTAINS,
			value: '',
		},
	]);

	let tabs: TabPage[] = [];
	$: tabs = [
		{
			key: 'search',
			label: 'Search',
			path: paths.minions.getPath('search'),
			component: MinionsTabSearch,
			data: { filters, navigate },
			hidden: !hasResaltPermission($currentUser.perms, P_MINION_LIST),
		},
		{
			key: 'presets',
			label: 'Presets',
			path: paths.minions.getPath('presets'),
			component: MinionsTabPresets,
			data: { filters },
			hidden: !hasResaltPermission($currentUser.perms, P_MINION_PRESETS_LIST),
		},
		{
			key: 'grains',
			label: 'Grains',
			path: paths.minions.getPath('grains'),
			component: MinionsTabGrains,
			data: { filters },
			hidden: !hasResaltPermission($currentUser.perms, P_MINION_GRAINEXPLORER),
		},
	];
</script>

<Tabs {tabs} current={subPage} />
