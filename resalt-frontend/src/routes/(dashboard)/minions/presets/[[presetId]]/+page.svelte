<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import Clickable from '$component/Clickable.svelte';
	import Icon from '$component/Icon.svelte';
	import ResaltProgress from '$component/ResaltProgress.svelte';
	import TableBottom from '$component/TableBottom.svelte';
	import {
		createMinionPreset,
		deleteMinionPreset,
		getMinionPresetById,
		getMinionPresets,
		updateMinionPreset,
	} from '$lib/api';
	import paths from '$lib/paths';
	import { P_MINION_PRESETS_MANAGE, hasResaltPermission } from '$lib/perms';
	import { currentUser, filters, replacementParams, theme, toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';
	import MinionPreset from '$model/MinionPreset';
	import { onMount } from 'svelte';
	import { writable, type Writable } from 'svelte/store';
	import MinionsFiltersBox from '../../MinionsFiltersBox.svelte';
	import MinionsListTable from '../../MinionsListTable.svelte';
	import type Filter from '$model/Filter';

	$: selected = $page.params.presetId as string | undefined;

	// Temporary data
	let presets: Writable<MinionPreset[] | null> = writable(null);
	let selectedPreset: Writable<MinionPreset> = writable(new MinionPreset('None', '', [], true));

	function randNumSuffix(): string {
		// Random between 1000 and 9999
		return Math.floor(Math.random() * 9000 + 1000).toString();
	}

	function loadPresets(): void {
		getMinionPresets().then((data: MinionPreset[]) => {
			presets.set(data);
			if (!selected) {
				// If no preset is selected, select the first one
				if (data.length > 0) {
					goto(paths.minions_presets.getPath({ presetId: data[0].id }), {
						invalidateAll: true,
					});
				}
			}
		});
	}

	function loadPreset(presetId: string): void {
		getMinionPresetById(presetId)
			.then((data: MinionPreset) => {
				selectedPreset.set(data);
				replacementParams.set({ ...$replacementParams, presetId: data.name });
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
					// Clone using .clone instead of JSON.parse(JSON.stringify()) because it's faster
					let deepCopy: Filter[] = [...data.filters.map((f: Filter) => f.clone())];
					filters.set(deepCopy);
				}
			})
			.catch((error: Error) => {
				toasts.add(MessageType.ERROR, 'Failed to load preset', error);
				console.error(error);
				goto(paths.minions_presets.getPath(), { invalidateAll: true });
			});
	}

	function createNew(): void {
		createMinionPreset('#Preset#' + randNumSuffix(), $filters)
			.then((data: MinionPreset) => {
				toasts.add(
					MessageType.SUCCESS,
					'Preset saved',
					'New preset "' + data.name + '" saved.',
				);
				goto(paths.minions_presets.getPath({ presetId: data.id }), { invalidateAll: true });
			})
			.catch((error: Error) => {
				toasts.add(MessageType.ERROR, 'Failed to save new preset', error);
				console.error(error);
			});
	}

	function copySelected(): void {
		let name: string;
		// Check if $selectedPresets.name is same as the object in $presets with matching ID
		let preset = ($presets ?? []).find((p: MinionPreset) => p.id === $selectedPreset.id);
		if (preset && preset.name === $selectedPreset.name) {
			// If it is, add a suffix to the name
			name = $selectedPreset.name + ' Copy';
		} else {
			// If it isn't, use the name as is
			name = $selectedPreset.name;
		}
		createMinionPreset(name, $filters)
			.then((data: MinionPreset) => {
				toasts.add(
					MessageType.SUCCESS,
					'Preset saved',
					'New preset "' + data.name + '" saved.',
				);
				goto(paths.minions_presets.getPath({ presetId: data.id }), { invalidateAll: true });
			})
			.catch((error: Error) => {
				toasts.add(MessageType.ERROR, 'Failed to save new preset', error);
				console.error(error);
			});
	}

	function saveSelected(): void {
		if ($selectedPreset.name.length === 0) {
			toasts.add(MessageType.WARNING, 'Invalid preset name', 'Preset name cannot be empty.');
			return;
		}
		updateMinionPreset($selectedPreset.id, $selectedPreset.name, $filters)
			.then((data: MinionPreset) => {
				toasts.add(
					MessageType.SUCCESS,
					'Preset updated',
					'Preset "' + data.name + '" updated.',
				);
				logic($page.url.pathname);
				goto(paths.minions_presets.getPath({ presetId: data.id }), { invalidateAll: true });
			})
			.catch((error: Error) => {
				toasts.add(MessageType.ERROR, 'Failed to update preset', error);
				console.error(error);
			});
	}

	function deleteSelected(): void {
		deleteMinionPreset($selectedPreset.id)
			.then(() => {
				toasts.add(
					MessageType.SUCCESS,
					'Preset deleted',
					'Preset "' + $selectedPreset.name + '" deleted.',
				);
				goto(paths.minions_presets.getPath(), { invalidateAll: true });
			})
			.catch((error: Error) => {
				toasts.add(MessageType.ERROR, 'Failed to delete preset', error);
				console.error(error);
			});
	}

	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	function logic(_pathname: string) {
		// _pathname is required to re-trigger render when $page.url.pathname changes
		loadPresets();
		if (selected) {
			loadPreset(selected);
		} else {
			selectedPreset.set(new MinionPreset('None', '', [], true));
		}
	}

	$: logic($page.url.pathname);
	onMount(() => {
		logic($page.url.pathname);
	});

	function getTitle(selectedPreset: MinionPreset): string {
		if (selected) {
			if (selectedPreset.id !== 'None') {
				return 'Preset :: ' + selectedPreset.name;
			} else {
				return 'Preset :: ' + selected;
			}
		} else {
			return 'Presets';
		}
	}

	$: {
		document.title = getTitle($selectedPreset);
	}
</script>

<svelte:head>
	<title>{getTitle($selectedPreset)}</title>
</svelte:head>

<div class="row">
	<div class="col-3">
		{#if hasResaltPermission($currentUser, P_MINION_PRESETS_MANAGE)}
			<div>
				<button type="button" class="btn btn-success me-2 mb-2" on:click={createNew}>
					New
				</button>
				<button
					type="button"
					class="btn btn-info me-2 mb-2"
					on:click={copySelected}
					disabled={$selectedPreset.id === 'None'}
				>
					Copy
				</button>
				<button
					type="button"
					class="btn btn-primary me-2 mb-2"
					on:click={saveSelected}
					disabled={$selectedPreset.id === 'None' || $selectedPreset.name.length === 0}
				>
					Save
				</button>
				<button
					type="button"
					class="btn btn-danger me-2 mb-2"
					on:click={deleteSelected}
					disabled={$selectedPreset.id === 'None'}
				>
					Delete
				</button>
			</div>
		{/if}
		<div class="table-responsive border-bottom-0 mb-3">
			<table class="table b-0 mb-0">
				<thead class="border-0">
					<tr>
						<th class="border-secondary bg-dark text-white">
							<div class="row g-1">
								<div class="col-auto align-self-center ps-2">Preset Name</div>
							</div>
						</th>
					</tr>
				</thead>
				<tbody class="align-middle">
					{#if presets}
						{#each $presets ?? [] as preset}
							<Clickable
								type="tr"
								event={() =>
									goto(paths.minions_presets.getPath({ presetId: preset.id }))}
								class={$selectedPreset.id === preset.id ? 'text-success' : ''}
							>
								<th
									class={$selectedPreset.id === preset.id
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
			<TableBottom />
			{#if !presets}
				<ResaltProgress />
			{/if}
		</div>
	</div>
	<div class="col-9">
		<div class="row">
			<div class="col-6">
				<div class="form-floating mb-3">
					<input
						id="presetName"
						type="text"
						class="form-control {$selectedPreset.id !== 'None' &&
						$selectedPreset.name.length === 0
							? 'is-invalid'
							: ''}"
						bind:value={$selectedPreset.name}
						disabled={$selectedPreset.id === 'None'}
					/>
					<label class="form-label" for="presetName">Name</label>
				</div>
			</div>
			<div class="col-6">
				<div class="form-floating mb-3">
					<input
						id="presetId"
						type="text"
						class="form-control"
						value={$selectedPreset.id}
						required
						disabled
					/>
					<label class="form-label" for="presetId">Selected Preset</label>
				</div>
			</div>
		</div>

		<MinionsFiltersBox />

		<hr class="text-light" />

		<MinionsListTable filters={$filters} />
	</div>
</div>
