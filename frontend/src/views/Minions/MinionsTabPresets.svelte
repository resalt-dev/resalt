<script lang="ts">
	import type { Writable } from 'svelte/store';
	import { Col, Row } from 'sveltestrap';
	import type Filter from '../../models/Filter';
	import MinionsFiltersBox from './MinionsFiltersBox.svelte';

	export let filters: Writable<Filter[]>;

	function updateData() {
		// TODO: implement
	}
</script>

<Row>
	<Col xs="2">
		<h2>Presets</h2>
	</Col>
	<Col xs="10">
		<MinionsFiltersBox {filters} {updateData} />

		<hr class="bg-light" />

		<div>
			<br />
			Not yet implemented. "Presets" will allow you to save collections of filters for quick access.
			Presets can be used as a target for scheduling and running jobs.
			<br /><br />
		</div>
	</Col>
</Row>
