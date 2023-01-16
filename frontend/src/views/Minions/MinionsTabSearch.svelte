<script lang="ts">
	import type { NavigateFn } from 'svelte-navigator';
	import type { Writable } from 'svelte/store';
	import type Filter from '../../models/Filter';
	import MinionsFiltersBox from './MinionsFiltersBox.svelte';
	import MinionsListTable from './MinionsListTable.svelte';

	export let navigate: NavigateFn;
	export let filters: Writable<Filter[]>;
</script>

<MinionsFiltersBox {filters} />

<hr class="text-light" />

<MinionsListTable {navigate} filters={$filters} />
