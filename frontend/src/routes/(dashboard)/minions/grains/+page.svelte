<script lang="ts">
	import { searchGrains } from '$lib/api';
	import ConsoleChangeBranch from '../../../../components/ConsoleChangeBranch.svelte';
	import FloatingRightButton from '../../../../components/FloatingRightButton.svelte';
	import Icon from '../../../../components/Icon.svelte';
	import JsonViewer from '../../../../components/JsonViewer.svelte';
	import ResaltProgress from '../../../../components/ResaltProgress.svelte';
	import TerminalBox from '../../../../components/TerminalBox.svelte';
	import { MessageType } from '../../../../models/MessageType';
	import { theme, toasts, filters } from '$lib/stores';
	import MinionsFiltersBox from '../MinionsFiltersBox.svelte';

	let result: unknown[] | null = null;
	let loading = false;
	let rawData = false;

	let grainQueryFieldValue = '';
	let grainQueryFieldError = false;

	function updateData() {
		if (!_validate()) {
			return;
		}

		loading = true;
		searchGrains(grainQueryFieldValue, $filters)
			.then((data) => {
				result = data;
				loading = false;
			})
			.catch((error) => {
				toasts.add(MessageType.ERROR, 'Failed fetching grains', error);
				loading = false;
			});
	}

	/*
    // VALIDATION
    */

	function _validate(): boolean {
		validateGrainQueryField();

		return !grainQueryFieldError;
	}

	function validateGrainQueryField(): void {
		if (grainQueryFieldValue === '') {
			grainQueryFieldError = true;
			return;
		}

		grainQueryFieldError = false;
	}
</script>

<MinionsFiltersBox />

<hr class="text-light" />

<form action="javascript:void(0);" autocomplete="true">
	<div class="row">
		<div class="col-11 col-lg-6 col-xl-5">
			<div class="form-floating mb-0">
				<input
					id="grainQuery"
					type="text"
					class="form-control {grainQueryFieldError ? 'is-invalid' : ''}"
					bind:value={grainQueryFieldValue}
				/>
				<label class="form-label" for="grainQuery">Grain Query (JSONPath)</label>
			</div>
		</div>
		<div class="col-1 col-lg-6 col-xl-7">
			<button
				type="button"
				class="btn btn-{$theme.color} py-2"
				on:click={updateData}
				disabled={loading}
			>
				<Icon name="search" class="mx-1 mt-1 pt-2 pb-1" size="1.5" />
			</button>
		</div>
	</div>
</form>

<hr class="text-light" />

{#if loading}
	<ResaltProgress />
{/if}

<FloatingRightButton
	onclick={() => (rawData = !rawData)}
	label={rawData ? 'View List' : 'View JSON'}
/>

{#if rawData}
	<JsonViewer data={result} />
{:else}
	<TerminalBox collapsed={result !== null} class="mb-0">
		<div slot="header">Grains</div>
		<div slot="body">
			{#if result}
				<ConsoleChangeBranch data={result} />
			{/if}
		</div>
	</TerminalBox>
{/if}
