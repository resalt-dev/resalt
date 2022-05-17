<script>
    import { onMount } from "svelte";
    import { load_minions } from "../controller";
    import { minions } from "../stores";
    import constants from "../constants";
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
            datatable_sidecolor:
                minion.last_updated_conformity == null
                    ? "purple"
                    : minion.conformity_error > 0
                    ? "red"
                    : minion.conformity_incorrect > 0
                    ? "orange"
                    : "green",
        };
    });

    onMount(() => {
        load_minions(navigate);
    });
</script>

<h1>Minions</h1>

<div class="bg-light text-white p-2">
    <div class="row">
        <div class="col-4" />
        <div class="col-8">
            <button
                class="btn btn-secondary btn-sm"
                on:click={() => load_minions(navigate)}>Load minions</button
            >
            <button
                class="btn btn-info btn-sm"
                on:click={() => load_minions(navigate, true)}
                >Force reload minions</button
            >
        </div>
    </div>
</div>

{#if !$minions}
    <div class="p-3">No minions detected. Try force reload.</div>
{:else}
    <div class="table-responsive">
        <table id="minionListTable" class="table table-hover">
            <thead class="bg-dark text-white border-0">
                <tr>
                    <th scope="col">
                        <div class="row g-1">
                            <div class="col-auto align-self-center ps-2">
                                ID
                            </div>
                            <div class="col-auto align-self-center d-grid">
                                <Icon
                                    size="1.125"
                                    name="chevron-up"
                                    class="sort-icon mouse-pointer"
                                />
                                <Icon
                                    size="1.125"
                                    name="chevron-down"
                                    class="sort-icon mouse-pointer"
                                />
                            </div>
                            <div class="col-auto align-self-center">
                                <input
                                    type="text"
                                    class="ms-1 lh-1"
                                    size="20"
                                />
                            </div>
                        </div>
                    </th>
                    <th scope="col">
                        <div class="row g-1">
                            <div class="col-auto align-self-center">Type</div>
                            <div class="col-auto align-self-center d-grid">
                                <Icon
                                    size="1.125"
                                    name="chevron-up"
                                    class="sort-icon mouse-pointer"
                                />
                                <Icon
                                    size="1.125"
                                    name="chevron-down"
                                    class="sort-icon mouse-pointer"
                                />
                            </div>
                            <div class="col-auto align-self-center">
                                <input
                                    type="text"
                                    class="ms-1 lh-1"
                                    size="20"
                                />
                            </div>
                        </div>
                    </th>
                    <th scope="col">
                        <div class="row g-1">
                            <div class="col-auto align-self-center">
                                Last seen
                            </div>
                            <div class="col-auto align-self-center d-grid">
                                <Icon
                                    size="1.125"
                                    name="chevron-up"
                                    class="sort-icon mouse-pointer"
                                />
                                <Icon
                                    size="1.125"
                                    name="chevron-down"
                                    class="sort-icon mouse-pointer"
                                />
                            </div>
                        </div>
                    </th>
                    <th scope="col">
                        <div class="row g-1">
                            <div class="col-auto align-self-center">
                                Conformity
                            </div>
                            <div class="col-auto align-self-center d-grid">
                                <Icon
                                    size="1.125"
                                    name="chevron-up"
                                    class="sort-icon mouse-pointer"
                                />
                                <Icon
                                    size="1.125"
                                    name="chevron-down"
                                    class="sort-icon mouse-pointer"
                                />
                            </div>
                        </div>
                    </th>
                    <th scope="col">Actions</th>
                </tr>
            </thead>
            <tbody class="align-middle">
                {#each mapped_minions as minion}
                    <tr>
                        <th
                            scope="row"
                            class="startside-{minion.datatable_sidecolor} mouse-pointer"
                            on:click={() =>
                                navigate(paths.minion.getPath(minion.id))}
                            >{minion.id}</th
                        >
                        <td>{minion.datatable_type}</td>
                        <td>{minion.last_seen}</td>
                        <td>
                            {#if minion.last_updated_conformity == null}
                                <span class="badge bg-purple"> Unknown </span>
                            {:else}
                                <span class="badge bg-green fw-bold">
                                    {minion.conformity_success ?? "?"}
                                </span>
                                /
                                <span class="badge bg-warning fw-bold">
                                    {minion.conformity_incorrect ?? "?"}
                                </span>
                                /
                                <span class="badge bg-red fw-bold">
                                    {minion.conformity_error ?? "?"}
                                </span>
                            {/if}
                        </td>
                        <td>
                            <button
                                class={`btn btn-primary btn-sm px-3`}
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

<style>
    :global(.sort-icon) {
        margin-top: -0.3rem;
        margin-bottom: -0.3rem;
    }
</style>
