<script lang="ts">
	import type { Writable } from 'svelte/store';
	import { Col, Input, Label, Row } from 'sveltestrap';
	import type Filter from '../../models/Filter';
	import MinionsFiltersBox from './MinionsFiltersBox.svelte';

	export let filters: Writable<Filter[]>;

	let grainFilterFieldValue: string = '';
	let grainFilterFieldError: boolean = false;

	function updateData() {
		// TODO: implement
	}

	/*
    // VALIDATION
    */

	function validateGrainFilterField(): void {
		// TODO: implement
	}
</script>

<MinionsFiltersBox {filters} {updateData} />

<hr class="bg-light" />

<Row>
	<Col xs="12" lg="6" xl="5">
		<div class="form-floating mb-0">
			<Input
				id="grainFilter"
				type="text"
				invalid={grainFilterFieldError}
				bind:value={grainFilterFieldValue}
				on:blur={validateGrainFilterField}
			/>
			<Label for="grainFilter">Grain Filter (JSONPath)</Label>
		</div>
	</Col>
</Row>

<hr class="bg-light" />

<div>
	Not yet implemented. "Grains" will allow you to view grains across all minions even if they are
	offline. This will be useful for viewing grains that are not updated frequently, such as
	hardware information.
</div>
