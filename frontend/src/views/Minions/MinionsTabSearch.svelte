<script lang="ts">
    import type Filter from '../../models/Filter';
    import MinionsFiltersBox from './MinionsFiltersBox.svelte';

    export let tabData: { update: (filters: Filter[]) => void };
</script>

<MinionsFiltersBox update={tabData.update} />
