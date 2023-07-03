<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { MessageType } from '../../../../../models/MessageType';
	import { getMinionById } from '$lib/api';
	import { toasts } from '$lib/stores';
	import { writable, type Writable } from 'svelte/store';
	import JsonViewer from '../../../../../components/JsonViewer.svelte';
	import type Minion from '../../../../../models/Minion';

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

{#if !$minion?.pillars}
	<div class="p-3">No pillars data. Please refresh minion.</div>
{:else}
	<JsonViewer data={JSON.parse($minion.pillars)} />
{/if}
