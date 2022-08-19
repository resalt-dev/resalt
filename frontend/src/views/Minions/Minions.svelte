<script lang="ts">
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import { AlertType, getMinions, showAlert } from "../../controller";
    import { theme } from "../../stores";
    import Icon from "../../components/Icon.svelte";
    import paths from "../../paths";
    import { Link } from "svelte-navigator";
    import { Table } from "sveltestrap";
    import TablePaginate from "../../components/TablePaginate.svelte";
    import Tabs from "../../components/Tabs.svelte";
    import MinionsTabSearch from "./MinionsTabSearch.svelte";
    import MinionsTabGroups from "./MinionsTabGroups.svelte";

    let paginationSize: number = 10;
    let paginationPage: number = 1;

    const minions = writable(null);

    function updateData() {
        getMinions(paginationSize, (paginationPage - 1) * paginationSize)
            .then((data) => {
                minions.set(
                    data.map((minion) => {
                        const grains = JSON.parse(minion.grains ?? "{}");
                        return {
                            ...minion,
                            datatable_type: (
                                (grains["osfullname"] ?? "Unknown") +
                                " " +
                                (grains["osrelease"] ?? "")
                            ).trim(),
                        };
                    })
                );
            })
            .catch((err) => {
                showAlert(AlertType.ERROR, "Failed fetching minions", err);
            });
    }

    onMount(() => {
        updateData();
    });

    let mappingA = new Map<string, any>();
    mappingA.set("search", MinionsTabSearch);
    mappingA.set("groups", MinionsTabGroups);
</script>

<h1>Minions</h1>

<Tabs
    children={[
        {
            label: "Search",
            component: MinionsTabSearch,
        },
        {
            label: "Groups",
            component: MinionsTabGroups,
        },
    ]}
/>

<div class="table-responsive card {$theme.dark ? 'bg-dark' : ''}">
    <Table
        dark={$theme.dark}
        hover
        id="minionListTable"
        class="b-0 mb-0 {$theme.dark ? 'text-light border-secondary' : ''}"
    >
        <thead
            class="bg-dark border-0 {$theme.dark ? 'text-light' : 'text-white'}"
        >
            <tr>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center ps-2">ID</div>
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
                <th scope="col" class="border-secondary">
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
                    </div>
                </th>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Last seen</div>
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
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Conformity</div>
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
                <th scope="col" class="border-secondary">Actions</th>
            </tr>
        </thead>
        <tbody class="align-middle">
            {#if $minions === null}
                <p>Loading</p>
            {:else if $minions.length === 0 && paginationPage === 1}
                <div class="p-3">No minions detected. Try force reload.</div>
            {:else}
                {#each $minions as minion}
                    <tr>
                        <th scope="row">
                            <Link
                                to={paths.minion.getPath(minion.id)}
                                class="text-reset text-decoration-none"
                                >{minion.id}</Link
                            >
                        </th>
                        <td>{minion.datatable_type}</td>
                        <td>{minion.last_seen}</td>
                        <td>
                            {#if minion.last_updated_conformity === null}
                                <span class="badge bg-purple"> Unknown </span>
                            {:else}
                                <span class="badge bg-green">
                                    {minion.conformity_success ?? "?"}
                                </span>
                                /
                                <span class="badge bg-warning">
                                    {minion.conformity_incorrect ?? "?"}
                                </span>
                                /
                                <span class="badge bg-red">
                                    {minion.conformity_error ?? "?"}
                                </span>
                            {/if}
                        </td>
                        <td>
                            <Link
                                to={paths.minion.getPath(minion.id)}
                                class="btn btn-{$theme.color} btn-sm px-3"
                                >View</Link
                            >
                        </td>
                    </tr>
                {/each}
            {/if}
        </tbody>
    </Table>
</div>

<TablePaginate
    bind:size={paginationSize}
    bind:page={paginationPage}
    last={$minions === null || $minions.length < paginationSize}
    {updateData}
/>
