<script lang="ts">
	import { onMount } from 'svelte';
	import type { NavigateFn } from 'svelte-navigator';
	import type { Writable } from 'svelte/store';
	import {
		createMinionPreset,
		deleteMinionPreset,
		getMinionPresetById,
		getMinionPresets,
		updateMinionPreset,
	} from '../../api';
	import Clickable from '../../components/Clickable.svelte';
	import Icon from '../../components/Icon.svelte';
	import ResaltProgress from '../../components/ResaltProgress.svelte';
	import TablePaginate from '../../components/TablePaginate.svelte';
	import type Filter from '../../models/Filter';
	import { MessageType } from '../../models/MessageType';
	import MinionPreset from '../../models/MinionPreset';
	import paths from '../../paths';
	import { hasResaltPermission, P_MINION_PRESETS_MANAGE } from '../../perms';
	import { currentUser, theme, toasts } from '../../stores';
	import MinionsFiltersBox from './MinionsFiltersBox.svelte';
	import MinionsListTable from './MinionsListTable.svelte';

	// Constants coming from above
	export let navigate: NavigateFn;
	export let filters: Writable<Filter[]>;
	export let selected: string;

	// Query
	let search: string = '';
	let paginationSize: number = 20;
	let paginationPage: number = 1;

	// Temporary data
	let name: string = '';
	let presets: MinionPreset[] | null = null;
	let selectedPreset: MinionPreset = new MinionPreset('None', '', [], true);

	function randNumSuffix(): string {
		// Random between 1000 and 9999
		return Math.floor(Math.random() * 9000 + 1000).toString();
	}

	function updateData(): void {
		getMinionPresets(search, paginationSize, (paginationPage - 1) * paginationSize).then(
			(data: MinionPreset[]) => {
				presets = data;
			},
		);
	}

	let loadingPreset = false;
	function loadPreset(presetId: string): void {
		if (loadingPreset) {
			return;
		}
		loadingPreset = true;
		if (!presetId || presetId.length === 0) {
			selectedPreset = new MinionPreset('None', '', [], true);
			name = '';

			if (!presets || presets.length === 0) {
				updateData();
			}
			loadingPreset = false;
			return;
		}

		if (selected != presetId) {
			navigate(paths.minions.getPath('presets', presetId));
			loadingPreset = false;
			return;
		}

		getMinionPresetById(presetId)
			.then((data: MinionPreset) => {
				loadingPreset = false;
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
					name = data.name;
					filters.set(deepCopy);
					updateData();
				}
			})
			.catch((error: Error) => {
				loadingPreset = false;
				toasts.add(MessageType.ERROR, 'Failed to load preset', error);
				console.error(error);
				navigate(paths.minions.getPath('presets'));
			});
	}

	function saveAsNew(): void {
		createMinionPreset(name.length > 0 ? name : 'Preset #' + randNumSuffix(), $filters)
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

	function updateSelected(): void {
		updateMinionPreset(
			selectedPreset.id,
			name.length > 0 ? name : 'Preset #' + randNumSuffix(),
			$filters,
		)
			.then((data: MinionPreset) => {
				toasts.add(
					MessageType.SUCCESS,
					'Preset updated',
					'Preset "' + data.name + '" updated.',
				);
				loadPreset(data.id);
			})
			.catch((error: Error) => {
				toasts.add(MessageType.ERROR, 'Failed to update preset', error);
				console.error(error);
			});
	}

	function deleteSelected(): void {
		deleteMinionPreset(selectedPreset.id)
			.then(() => {
				toasts.add(
					MessageType.SUCCESS,
					'Preset deleted',
					'Preset "' + selectedPreset.name + '" deleted.',
				);
				loadPreset('');
			})
			.catch((error: Error) => {
				toasts.add(MessageType.ERROR, 'Failed to delete preset', error);
				console.error(error);
			});
	}

	$: loadPreset(selected);

	onMount(() => {
		loadPreset(selected);
	});
</script>

<div class="row">
	<div class="col-3">
		{#if hasResaltPermission($currentUser, P_MINION_PRESETS_MANAGE)}
			<div>
				<button type="button" class="btn btn-success me-2 mb-2" on:click={saveAsNew}>
					Copy as New
				</button>
				<button
					type="button"
					class="btn btn-primary me-2 mb-2"
					on:click={updateSelected}
					disabled={selectedPreset == null}
				>
					Save Preset
				</button>
				<button
					type="button"
					class="btn btn-danger me-2 mb-2"
					on:click={deleteSelected}
					disabled={selectedPreset == null}
				>
					Delete Preset
				</button>
			</div>
		{/if}
		<div class="table-responsive border-bottom-0 mb-3">
			<table class="table b-0 mb-0">
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
										  ' border-' +
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
			</table>
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
	</div>
	<div class="col-9">
		<div class="row">
			<div class="col-6">
				<div class="form-floating mb-3">
					<input id="presetName" type="text" class="form-control" bind:value={name} />
					<label class="form-label" for="presetName">Name</label>
				</div>
			</div>
			<div class="col-6">
				<div class="form-floating mb-3">
					<input
						id="presetId"
						type="text"
						class="form-control"
						bind:value={selectedPreset.id}
						required
						disabled
					/>
					<label class="form-label" for="presetId">Selected Preset</label>
				</div>
			</div>
		</div>

		<MinionsFiltersBox {filters} />

		<hr class="text-light" />

		<MinionsListTable {navigate} filters={$filters} />
	</div>
</div>
