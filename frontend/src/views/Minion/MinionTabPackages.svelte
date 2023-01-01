<script lang="ts">
	import type { Writable } from 'svelte/store';
	import { Card, Table } from 'sveltestrap';
	import FloatingRightButton from '../../components/FloatingRightButton.svelte';
	import JsonViewer from '../../components/JsonViewer.svelte';
	import type Minion from '../../models/Minion';

	export let minion: Writable<Minion>;
	let rawData = false;

	$: pkgs = JSON.parse($minion.pkgs);
</script>

{#if !$minion.pkgs}
	<div class="p-3">No packages data. Please refresh minion.</div>
{:else}
	<FloatingRightButton
		onclick={() => (rawData = !rawData)}
		label={rawData ? 'View List' : 'View JSON'}
	/>
	{#if rawData}
		<JsonViewer data={JSON.parse($minion.pkgs)} />
	{:else}
		<Card class="table-responsive border-bottom-0">
			<Table hover class="b-0 mb-0">
				<thead class="bg-dark border-0 text-white">
					<tr>
						<th class="border-secondary"> Package </th>
						<th class="border-secondary"> Version </th>
						<th class="border-secondary" />
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
			</Table>
		</Card>
	{/if}
{/if}
