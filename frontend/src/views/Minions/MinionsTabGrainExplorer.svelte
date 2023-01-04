<script lang="ts">
	import type { Writable } from 'svelte/store';
	import { Col, Input, Label, Row } from 'sveltestrap';
	import { searchGrains } from '../../api';
	import ConsoleChangeBranch from '../../components/ConsoleChangeBranch.svelte';
	import FloatingRightButton from '../../components/FloatingRightButton.svelte';
	import JsonViewer from '../../components/JsonViewer.svelte';
	import TerminalBox from '../../components/TerminalBox.svelte';
	import type Filter from '../../models/Filter';
	import { FilterFieldType } from '../../models/FilterFieldType';
	import { MessageType } from '../../models/MessageType';
	import { toasts } from '../../stores';
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
		searchGrains(
			grainQueryFieldValue,
			$filters
				.filter((f) => f.fieldType !== FilterFieldType.NONE)
				.filter((f) => f.field !== '')
				// Filter out where field is 'last_seen' and value is empty
				.filter((f) => !(f.field === 'last_seen' && f.value === '')),
		)
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

<MinionsFiltersBox {filters} {updateData} />

<hr class="text-light" />

<Row>
	<Col xs="12" lg="6" xl="5">
		<div class="form-floating mb-0">
			<Input
				id="grainQuery"
				type="text"
				invalid={grainQueryFieldError}
				bind:value={grainQueryFieldValue}
				on:blur={updateData}
			/>
			<Label for="grainQuery">Grain Query (JSONPath)</Label>
		</div>
	</Col>
</Row>

<hr class="text-light" />

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
