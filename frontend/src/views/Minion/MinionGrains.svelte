<script lang="ts">
    import JsonViewer from "../../components/JsonViewer.svelte";

    export let minion;
</script>

{#if !minion.grains}
    <div class="p-3">No grains data. Please refresh minion.</div>
{:else}
    <JsonViewer code={minion.grains} />
{/if}
