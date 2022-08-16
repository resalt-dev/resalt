<script lang="ts">
    import JsonViewer from "../../components/JsonViewer.svelte";

    export let minion;
</script>

{#if !$minion.pillars}
    <div class="p-3">No pillars data. Please refresh minion.</div>
{:else}
    <JsonViewer code={$minion.pillars} />
{/if}
