<script lang="ts">
	import { page } from '$app/stores';
	import JsonViewer from '$component/JsonViewer.svelte';
	import { getMinionById } from '$lib/api';
	import { toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';
	import type Minion from '$model/Minion';
	import { onMount } from 'svelte';
	import { writable, type Writable } from 'svelte/store';

	$: minionId = $page.params.minionId;
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
	<title>Pillars :: {minionId}</title>
</svelte:head>

{#if !$minion?.pillars}
	<div class="p-3">No pillars data. Please refresh minion.</div>
{:else}
	<JsonViewer data={JSON.parse($minion.pillars)} />
{/if}
