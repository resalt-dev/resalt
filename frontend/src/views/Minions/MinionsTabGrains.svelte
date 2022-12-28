<script lang="ts">
	import type { Writable } from 'svelte/store';
	import type Filter from '../../models/Filter';
	import MinionsFiltersBox from './MinionsFiltersBox.svelte';

	export let filters: Writable<Filter[]>;
</script>

<MinionsFiltersBox {filters} />

<hr class="bg-light" />

<div>
	Not yet implemented. "Grains" will allow you to view grains across all minions even if they are
	offline. This will be useful for viewing grains that are not updated frequently, such as
	hardware information.
</div>
