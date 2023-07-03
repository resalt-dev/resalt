<script lang="ts">
	import { page } from '$app/stores';
	import FloatingRightButton from '$component/FloatingRightButton.svelte';
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

	let rawData = false;

	$: pkgs = JSON.parse($minion?.pkgs ?? '{}');
</script>

{#if !$minion?.pkgs}
	<div class="p-3">No packages data. Please refresh minion.</div>
{:else}
	<FloatingRightButton
		onclick={() => (rawData = !rawData)}
		label={rawData ? 'View List' : 'View JSON'}
	/>
	{#if rawData}
		<JsonViewer data={JSON.parse($minion?.pkgs)} />
	{:else}
		<div class="card table-responsive border-bottom-0">
			<table class="table table-hover b-0 mb-0">
				<thead class="border-0">
					<tr>
						<th class="border-secondary bg-dark text-white"> Package </th>
						<th class="border-secondary bg-dark text-white"> Version </th>
						<th class="border-secondary bg-dark text-white" />
					</tr>
				</thead>
				<tbody>
					{#each Object.entries(pkgs) as pkg}
						<tr>
							<td>{pkg[0]}</td>
							<td>{pkg[1]}</td>
							<td />
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
{/if}
