<script lang="ts">
	import { onMount } from 'svelte';
	import { MessageType } from '../../../models/MessageType';
	import { getMinionById } from '$lib/api';
	import { toasts } from '$lib/stores';
	import { writable, type Writable } from 'svelte/store';
	import paths from '$lib/paths';
	import Tabs from '../../../components/Tabs.svelte';
	import type Minion from '../../../models/Minion';

	export let minionId: string;

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
</script>

<svelte:head>
	<title>Minion {minionId}</title>
</svelte:head>

{#if !$minion}
	<h1>Loading...</h1>
{:else}
	<Tabs
		tabs={[
			paths.minion,
			paths.minion_grains,
			paths.minion_conformity,
			paths.minion_pillars,
			paths.minion_packages,
		]}
	>
		<slot />
	</Tabs>
{/if}
