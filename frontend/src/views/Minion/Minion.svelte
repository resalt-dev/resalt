<script lang="ts">
	import { onMount } from 'svelte';
	import { currentUser } from '../../stores';
	import {
		hasResaltPermission,
		P_ADMIN_GROUP,
		P_MINION_CONFORMITY,
		P_MINION_PACKAGES,
		P_MINION_PILLARS,
	} from '../../perms';
	import { MessageType } from '../../models/MessageType';
	import { getMinionById } from '../../api';
	import { toasts } from '../../stores';
	import { writable, type Writable } from 'svelte/store';
	import paths from '../../paths';
	import Tabs from '../../components/Tabs.svelte';
	import type { NavigateFn } from 'svelte-navigator';
	import type Minion from '../../models/Minion';
	import type TabPage from '../../models/TabPage';

	import MinionTabConformity from './MinionTabConformity.svelte';
	import MinionTabGrains from './MinionTabGrains.svelte';
	import MinionTabInfo from './MinionTabInfo.svelte';
	import MinionTabPackages from './MinionTabPackages.svelte';
	import MinionTabPillars from './MinionTabPillars.svelte';

	// svelte-ignore unused-export-let
	export let location: Location;
	// svelte-ignore unused-export-let
	export let navigate: NavigateFn;
	export let minionId: string;
	export let subPage: string = '';

	const minion: Writable<Minion | null> = writable(null);

	onMount(() => {
		getMinionById(minionId)
			.then((data) => {
				minion.set(data);
			})
			.catch((err) => {
				toasts.add(MessageType.ERROR, 'Failed fetching minion: ' + minionId, err);
			});
	});

	let tabs: TabPage[] = [];
	$: tabs = [
		{
			key: 'info',
			label: 'General',
			path: paths.minion.getPath(minionId),
			component: MinionTabInfo,
			data: { minion },
		},
		{
			key: 'conformity',
			label: 'Conformity',
			path: paths.minion.getPath(minionId, 'conformity'),
			component: MinionTabConformity,
			data: { minion },
			hidden: !hasResaltPermission($currentUser.perms, P_MINION_CONFORMITY),
		},
		{
			key: 'grains',
			label: 'Grains',
			path: paths.minion.getPath(minionId, 'grains'),
			component: MinionTabGrains,
			data: { minion },
		},
		{
			key: 'pillars',
			label: 'Pillars',
			path: paths.minion.getPath(minionId, 'pillars'),
			component: MinionTabPillars,
			data: { minion },
			hidden: !hasResaltPermission($currentUser.perms, P_MINION_PILLARS),
		},
		{
			key: 'packages',
			label: 'Packages',
			path: paths.minion.getPath(minionId, 'packages'),
			component: MinionTabPackages,
			data: { minion },
			hidden: !hasResaltPermission($currentUser.perms, P_MINION_PACKAGES),
		},
	];
</script>

{#if !$minion}
	<h1>Loading...</h1>
{:else}
	<Tabs {tabs} current={subPage} />
{/if}
