<script lang="ts">
	import { onMount } from 'svelte';
	import type { NavigateFn } from 'svelte-navigator';
	import type { Writable } from 'svelte/store';
	import { Button, Col, Row, Table } from 'sveltestrap';
	import { createMinionPreset, getMinionPresetById, getMinionPresets } from '../../api';
	import Clickable from '../../components/Clickable.svelte';
	import Icon from '../../components/Icon.svelte';
	import ResaltProgress from '../../components/ResaltProgress.svelte';
	import TablePaginate from '../../components/TablePaginate.svelte';
	import type Filter from '../../models/Filter';
	import { MessageType } from '../../models/MessageType';
	import type MinionPreset from '../../models/MinionPreset';
	import paths from '../../paths';
	import { hasResaltPermission, P_MINION_PRESETS_MANAGE } from '../../perms';
	import { currentUser, theme, toasts } from '../../stores';
	import MinionsFiltersBox from './MinionsFiltersBox.svelte';
	import MinionsListTable from './MinionsListTable.svelte';

	export let navigate: NavigateFn;
	export let filters: Writable<Filter[]>;
	export let selected: string;

	let search: string = '';
	let paginationSize: number = 20;
	let paginationPage: number = 1;

	let presets: MinionPreset[] | null = null;
	let selectedPreset: MinionPreset | null = null;

	function updateData(): void {
		getMinionPresets(search, paginationSize, (paginationPage - 1) * paginationSize).then(
			(data: MinionPreset[]) => {
				presets = data;
			},
		);
	}

	function loadPreset(presetId: string): void {
		if (!presetId || presetId.length === 0) {
			return;
		}

		if (selected != presetId) {
			navigate(paths.minions.getPath('presets', presetId));
			return;
		}

		selectedPreset = null;
		getMinionPresetById(presetId).then((data: MinionPreset) => {
			selectedPreset = data;
			if (data.invalidData) {
				// Don't load invalid preset
				toasts.add(
					MessageType.WARNING,
					'Invalid preset',
					'The selected preset has invalid data. Please delete and recreate it.',
				);
			} else if (data.filters.length === 0) {
				// Never allow completely empty filters
				toasts.add(
					MessageType.WARNING,
					'Empty preset',
					'The selected preset has no filters. This is not allowed.',
				);
			} else {
				let deepCopy = JSON.parse(JSON.stringify(data.filters));
				filters.set(deepCopy);
				updateData();
			}
		});
	}

	function saveAsNew(): void {
		createMinionPreset('New Preset', $filters)
			.then((data: MinionPreset) => {
				toasts.add(
					MessageType.SUCCESS,
					'Preset saved',
					'New preset "' + data.name + '" saved.',
				);
				loadPreset(data.id);
			})
			.catch((error: Error) => {
				toasts.add(MessageType.ERROR, 'Failed to save new preset', error);
				console.error(error);
			});
	}

	$: loadPreset(selected);

	onMount(() => {
		loadPreset(selected);
	});
</script>

<Row>
	<Col xs="3">
		{#if hasResaltPermission($currentUser, P_MINION_PRESETS_MANAGE)}
			<div>
				<Button class="me-2 mb-2" color="success" on:click={saveAsNew}>Save as New</Button>
				<!-- <Button class="me-2 mb-2" color="primary">Update Preset</Button> -->
			</div>
		{/if}
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
					{#if presets}
						{#each presets as preset}
							<Clickable
								type="tr"
								event={() => loadPreset(preset.id)}
								class={selectedPreset?.id === preset.id ? 'text-success' : ''}
							>
								<th
									class={selectedPreset?.id === preset.id
										? 'bg-' +
										  $theme.color +
										  ' text-' +
										  ($theme.color === 'yellow' ? 'black' : 'white')
										: ''}
								>
									{preset.name}

									{#if preset.invalidData}
										<Icon
											class="text-warning ms-2"
											name="error"
											size="1.5"
											tooltip="Invalid data! Please delete and re-create this preset."
										/>
									{/if}
								</th>
							</Clickable>
						{/each}
					{/if}
				</tbody>
			</Table>
			<TablePaginate
				bind:size={paginationSize}
				bind:page={paginationPage}
				last={presets === null || presets.length < paginationSize}
				{updateData}
				resizeable={false}
			/>
			{#if !presets}
				<ResaltProgress />
			{/if}
		</div>
	</Col>
	<Col xs="9">
		<MinionsFiltersBox {filters} />

		<hr class="text-light" />

		<MinionsListTable {navigate} filters={$filters} />
	</Col>
</Row>
