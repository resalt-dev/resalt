<script>
    import { onMount } from "svelte";
    import { load_minions } from "../controller";
    import { minions } from "../stores";
    import Icon from "../components/Icon.svelte";
    import paths from "../paths";

    import { useNavigate } from "svelte-navigator";
    const navigate = useNavigate();

    $: mapped_minions = ($minions ?? []).map((minion) => {
        const grains = JSON.parse(minion.grains ?? "{}");
        return {
            ...minion,
            datatable_type: (
                (grains["osfullname"] ?? "Unknown") +
                " " +
                (grains["osrelease"] ?? "")
            ).trim(),
        };
    });

    onMount(() => {
        load_minions(navigate);
    });
</script>

<h1>Minions</h1>

<div class="bg-dark text-white p-3">
    <div class="row">
        <div class="col-4">
            <div class="input-group flex-nowrap">
                <span class="input-group-text" id="addon-wrapping">
                    <Icon name="search" />
                </span>
                <input
                    type="text"
                    class="form-control"
                    placeholder="Search"
                    aria-label="Search"
                    aria-describedby="addon-wrapping"
                />
            </div>
        </div>
        <div class="col-8">
            <button class="btn btn-gold" on:click={() => load_minions(navigate)}
                >Load minions</button
            >
            <button
                class="btn btn-gold"
                on:click={() => load_minions(navigate, true)}
                >Force reload minions</button
            >
        </div>
    </div>
</div>

<br />

{#if !$minions}
    <div class="p-3">No conformity data. Please refresh minion.</div>
{:else}
    <div class="table-responsive">
        <table
            id="minionListTable"
            class="table table-striped table-hover align-middle"
        >
            <thead class="bg-dark text-white border-0">
                <tr>
                    <th scope="col">#</th>
                    <th scope="col">Type</th>
                    <th scope="col">Last seen</th>
                    <th scope="col">Conformity</th>
                    <th scope="col">Actions</th>
                </tr>
            </thead>
            <tbody>
                {#each mapped_minions as minion}
                    <tr>
                        <th
                            scope="row"
                            class="startside-success mouse-pointer"
                            on:click={() =>
                                navigate(paths.minion.getPath(minion.id))}
                            >{minion.id}</th
                        >
                        <td>{minion.datatable_type}</td>
                        <td>{minion.last_seen}</td>
                        <td>
                            {#if minion.last_updated_conformity == null}
                                <span class="badge bg-purple fs-6">Unknown</span
                                >
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
                                class="btn btn-gold btn-sm px-3"
                                on:click={() =>
                                    navigate(paths.minion.getPath(minion.id))}
                                >View</button
                            >
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/if}
