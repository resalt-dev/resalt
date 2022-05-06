<script>
    import { onMount } from "svelte";
    import { load_minions } from "../controller";
    import { minions } from "../stores";
    import paths from "../paths";

    import { useNavigate } from "svelte-navigator";
    const navigate = useNavigate();

    console.log($minions);

    onMount(() => {
        load_minions(navigate);
    });
</script>

<h1>Minions</h1>

<button class="btn btn-gold" on:click={() => load_minions(navigate)}
    >Load minions</button
>
<button class="btn btn-gold" on:click={() => load_minions(navigate, true)}
    >Force reload minions</button
>

<br />
<br />

{#if !$minions}
    <div class="p-3">No conformity data. Please refresh minion.</div>
{:else}
    {#each $minions as minion}
        <div class="card bg-light mb-3">
            <div class="card-header bg-light">
                <h5 class="fw-bold mb-0">{minion.id}</h5>
            </div>
            <div class="card-body py-1  ">
                <div class="row">
                    <div class="col-3">
                        Status:
                        {#if minion.last_updated_conformity == null}
                            <span class="badge bg-purple">Unknown</span>
                        {:else if minion.conformity_error > 0}
                            <span class="badge bg-red">Error</span>
                        {:else if minion.conformity_incorrect > 0}
                            <span class="badge bg-gold">Incorrect</span>
                        {:else if minion.conformity_succeeded > 0}
                            <span class="badge bg-green">OK</span>
                        {/if}
                    </div>
                    <div class="col-3">
                        Last updated: {minion.last_updated_conformity}
                    </div>
                    <div class="col-3">
                        Conformity:
                        {#if minion.last_updated_conformity == null}
                            <span class="badge bg-purple">Unknown</span>
                        {:else}
                            <span class="badge bg-green fs-6"
                                >{minion.conformity_success ?? "?"}</span
                            >
                            /
                            <span class="badge bg-gold fs-6"
                                >{minion.conformity_incorrect ?? "?"}</span
                            >
                            /
                            <span class="badge bg-red fs-6"
                                >{minion.conformity_error ?? "?"}</span
                            >
                        {/if}
                    </div>
                    <div class="col-3">
                        <button
                            class="btn btn-gold"
                            on:click={() =>
                                navigate(paths.minion.getPath(minion.id))}
                            >View</button
                        >
                        <button
                            class="btn btn-primary"
                            on:click={() =>
                                navigate(paths.minion.getPath(minion.id))}
                            >Details</button
                        >
                        <button
                            class="btn btn-secondary"
                            on:click={() =>
                                navigate(`minions/${minion.id}/refresh`)}
                            >Refresh</button
                        >
                        <button
                            class="btn btn-warning"
                            on:click={() =>
                                navigate(`minions/${minion.id}/execute`)}
                            >Run Job</button
                        >
                    </div>
                </div>
            </div>
        </div>
    {/each}
{/if}
