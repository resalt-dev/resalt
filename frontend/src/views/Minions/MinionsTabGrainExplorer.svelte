<script lang="ts">
	import type { Writable } from 'svelte/store';
	import { Button, Col, Input } from 'sveltestrap';
	import { searchGrains } from '../../api';
	import ConsoleChangeBranch from '../../components/ConsoleChangeBranch.svelte';
	import FloatingRightButton from '../../components/FloatingRightButton.svelte';
	import Icon from '../../components/Icon.svelte';
	import JsonViewer from '../../components/JsonViewer.svelte';
	import ResaltProgress from '../../components/ResaltProgress.svelte';
	import TerminalBox from '../../components/TerminalBox.svelte';
	import type Filter from '../../models/Filter';
	import { MessageType } from '../../models/MessageType';
	import { theme, toasts } from '../../stores';
	import MinionsFiltersBox from './MinionsFiltersBox.svelte';

	export let filters: Writable<Filter[]>;

	let result: any[] | null = null;
	let loading: boolean = false;
	let rawData = false;

	let grainQueryFieldValue: string = '';
	let grainQueryFieldError: boolean = false;

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

<MinionsFiltersBox {filters} />

<hr class="text-light" />

<form action="javascript:void(0);" autocomplete="true">
	<div class="row">
		<Col xs="11" lg="6" xl="5">
			<div class="form-floating mb-0">
				<Input
					id="grainQuery"
					type="text"
					invalid={grainQueryFieldError}
					bind:value={grainQueryFieldValue}
				/>
				<label class="form-label" for="grainQuery">Grain Query (JSONPath)</label>
			</div>
		</Col>
		<Col xs="1" lg="6" xl="7">
			<Button
				color={null}
				class="btn-{$theme.color} py-2"
				on:click={updateData}
				disabled={loading}
			>
				<Icon name="search" class="mx-1 mt-1 pt-2 pb-1" size="1.5" />
			</Button>
		</Col>
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
