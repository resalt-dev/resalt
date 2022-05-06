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

<table class="table table-striped table-hover align-middle">
    <thead>
        <tr>
            <th scope="col">ID</th>
            <th scope="col">Status</th>
            <th scope="col">Last Seen</th>
            <th scope="col">Conformity</th>
            <th scope="col">Actions</th>
        </tr>
    </thead>
    <tbody>
        {#if !$minions}
            <tr>
                <td colspan="5">No minions found...</td>
            </tr>
        {:else}
            {#each $minions as minion}
                <tr>
                    <th
                        scope="row"
                        on:click={() =>
                            navigate(paths.minion.getPath(minion.id))}
                        class="mouse-pointer">{minion.id}</th
                    >
                    <td>
                        {#if minion.last_updated_conformity == null}
                            <span class="badge bg-purple">Unknown</span>
                        {:else if minion.conformity_error > 0}
                            <span class="badge bg-red">Error</span>
                        {:else if minion.conformity_incorrect > 0}
                            <span class="badge bg-gold">Incorrect</span>
                        {:else if minion.conformity_succeeded > 0}
                            <span class="badge bg-green">OK</span>
                        {/if}
                    </td>
                    <td>{minion.last_seen}</td>
                    <td>
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
                    </td>
                    <td>
                        <button
                            class="btn btn-primary btn-sm"
                            on:click={() =>
                                navigate(paths.minion.getPath(minion.id))}
                            >Details</button
                        >
                        <button
                            class="btn btn-secondary btn-sm"
                            on:click={() =>
                                navigate(`minions/${minion.id}/refresh`)}
                            >Refresh</button
                        >
                        <button
                            class="btn btn-warning btn-sm"
                            on:click={() =>
                                navigate(`minions/${minion.id}/execute`)}
                            >Run Job</button
                        >
                    </td>
                </tr>
            {/each}
        {/if}
    </tbody>
</table>
