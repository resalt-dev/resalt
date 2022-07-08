<script lang="ts">
    import JsonViewer from "../../components/JsonViewer.svelte";

    export let minion;
    let rawData = false;

    $: pkgs = JSON.parse(minion.pkgs);
</script>

{#if !minion.pkgs}
    <div class="p-3">No packages data. Please refresh minion.</div>
{:else}
    <button
        class="btn btn-light float-end border border-1 rounded-none"
        style="margin-top: -0rem;z-index: 4;position: absolute;right: 0;"
        on:click={() => (rawData = !rawData)}
    >
        {rawData ? "View List" : "View JSON"}
    </button>
    {#if rawData}
        <JsonViewer code={minion.pkgs} />
    {:else}
        <div class="p-3">
            <table class="table table-striped">
                <thead>
                    <tr>
                        <th scope="col">Package</th>
                        <th scope="col">Version</th>
                        <th scope="col" />
                    </tr>
                </thead>
                <tbody>
                    {#each Object.entries(pkgs) as pkg}
                        <tr>
                            <td>{pkg[0]}</td>
                            <td>{pkg[1]}</td>
                            <td />
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    {/if}
{/if}
