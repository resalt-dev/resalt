<script lang="ts">
    import type { Writable } from 'svelte/store';
    import JsonViewer from '../../components/JsonViewer.svelte';
    import type Minion from '../../models/Minion';

    export let tabData: Writable<Minion>;
</script>

{#if !$tabData.grains}
    <div class="p-3">No grains data. Please refresh minion.</div>
{:else}
    <JsonViewer data={JSON.parse($tabData.grains)} />
{/if}
