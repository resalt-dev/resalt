<script lang="ts">
	import type { Writable } from 'svelte/store';
	import { filters } from '$lib/stores';
	import type Filter from '../../../models/Filter';
	import MinionsFiltersBox from './MinionsFiltersBox.svelte';
	import MinionsListTable from './MinionsListTable.svelte';
</script>

<MinionsFiltersBox />

<hr class="text-light" />

<MinionsListTable filters={$filters} />
