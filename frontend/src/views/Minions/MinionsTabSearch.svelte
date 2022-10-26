<script lang="ts">
    import type Filter from '../../models/Filter';
    import MinionsFiltersBox from './MinionsFiltersBox.svelte';

    export let setFilters: (filters: Filter[]) => void;
</script>

<MinionsFiltersBox {setFilters} />
