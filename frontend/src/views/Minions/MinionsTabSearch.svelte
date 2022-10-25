<script lang="ts">
    import type Filter from '../../models/Filter';
    import MinionsFiltersBox from './MinionsFiltersBox.svelte';

    export let tabData: { setFilters: (filters: Filter[]) => void };
</script>

<MinionsFiltersBox setFilters={tabData.setFilters} />
