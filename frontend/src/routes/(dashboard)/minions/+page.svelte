<script lang="ts">
	import { filters } from '$lib/stores';
	import MinionsFiltersBox from './MinionsFiltersBox.svelte';
	import MinionsListTable from './MinionsListTable.svelte';
</script>

<MinionsFiltersBox />

<hr class="text-light" />

<MinionsListTable filters={$filters} />
