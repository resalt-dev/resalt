<script lang="ts">
	import { onMount } from 'svelte';
	import type { NavigateFn } from 'svelte-navigator';
	import { writable, type Writable } from 'svelte/store';
	import { Col, Row, Table } from 'sveltestrap';
	import { getMinionPresets } from '../../api';
	import ResaltProgress from '../../components/ResaltProgress.svelte';
	import TablePaginate from '../../components/TablePaginate.svelte';
	import type Filter from '../../models/Filter';
	import type MinionPreset from '../../models/MinionPreset';
	import paths from '../../paths';
	import { theme } from '../../stores';
	import MinionsFiltersBox from './MinionsFiltersBox.svelte';

	export let navigate: NavigateFn;
	export let filters: Writable<Filter[]>;
	export let selected: string;

	let search: string = '';
	let paginationSize: number = 20;
	let paginationPage: number = 1;

	const presets: Writable<MinionPreset[] | null> = writable(null);
	const selectedPreset: Writable<MinionPreset | null> = writable(null);

	function updateData(): void {
		getMinionPresets(search, paginationSize, (paginationPage - 1) * paginationSize).then(
			(data: MinionPreset[]) => {
				presets.set(data);
			},
		);
	}

	onMount(() => {
		updateData();
	});
</script>

<Row>
	<Col xs="3">
		<div class="table-responsive border-bottom-0 mb-3">
			<Table class="b-0 mb-0">
				<thead class="bg-dark border-0 text-white">
					<tr>
						<th class="border-secondary">
							<div class="row g-1">
								<div class="col-auto align-self-center ps-2">Preset Name</div>
							</div>
						</th>
					</tr>
				</thead>
				<tbody class="align-middle">
					{#if $presets}
						{#each $presets as preset}
							<tr
								class="mouse-pointer {$selectedPreset?.id === preset.id
									? 'text-white'
									: ''}"
								on:click={() => {
									navigate(paths.minions.getPath('presets', preset.id));
								}}
							>
								<th
									class={$selectedPreset?.id === preset.id
										? 'bg-' + $theme.color + ' border-' + $theme.color
										: ''}
								>
									{preset.name}
								</th>
							</tr>
						{/each}
					{/if}
				</tbody>
			</Table>
			<TablePaginate
				bind:size={paginationSize}
				bind:page={paginationPage}
				last={$presets === null || $presets.length < paginationSize}
				{updateData}
				resizeable={false}
			/>
			{#if !$presets}
				<ResaltProgress />
			{/if}
		</div>
	</Col>
	<Col xs="9">
		<MinionsFiltersBox {filters} {updateData} />

		<hr class="text-light" />

		<div>
			<br />
			Implementation in progress. "Presets" will allow you to save collections of filters for quicker
			access. Presets can be used as a target for scheduling and running jobs.
			<br /><br />
		</div>
	</Col>
</Row>
