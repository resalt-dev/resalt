<script lang="ts">
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import { getMinions, showAlert } from "../../controller";
    import { theme } from "../../stores";
    import Icon from "../../components/Icon.svelte";
    import paths from "../../paths";
    import { Link } from "svelte-navigator";
    import { Table } from "sveltestrap";
    import TablePaginate from "../../components/TablePaginate.svelte";
    import Tabs from "../../components/Tabs.svelte";
    import MinionsTabSearch from "./MinionsTabSearch.svelte";
    import MinionsTabGroups from "./MinionsTabGroups.svelte";
    import { AlertType } from "../../models/AlertType";

    let sort: string = null;
    let paginationSize: number = 10;
    let paginationPage: number = 1;

    const SORT_COLOR = "text-orange";
    const minions = writable(null);

    function updateData() {
        getMinions(sort, paginationSize, (paginationPage - 1) * paginationSize)
            .then((data) => {
                minions.set(data);
            })
            .catch((err) => {
                showAlert(AlertType.ERROR, "Failed fetching minions", err);
            });
    }

    function toggleSort(field: string) {
        // field.order
        let parts = (sort || "null.null").split(".");
        if (parts[0] === field) {
            switch (parts[1]) {
                case "asc":
                    sort = field + ".desc";
                    break;
                case "desc":
                    sort = null;
                    break;
                default:
                    sort = field + ".asc";
                    break;
            }
        } else {
            sort = field + ".asc";
        }

        updateData();
    }

    onMount(() => {
        updateData();
    });
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
                                class="sort-icon mouse-pointer {sort ===
                                'id.asc'
                                    ? SORT_COLOR
                                    : ''}"
                                on:click={() => {
                                    toggleSort("id");
                                }}
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer {sort ===
                                'id.desc'
                                    ? SORT_COLOR
                                    : ''}"
                                on:click={() => {
                                    toggleSort("id");
                                }}
                            />
                        </div>
                    </div>
                </th>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">OS</div>
                        <div class="col-auto align-self-center d-grid">
                            <Icon
                                size="1.125"
                                name="chevron-up"
                                class="sort-icon mouse-pointer {sort ===
                                'osType.asc'
                                    ? SORT_COLOR
                                    : ''}"
                                on:click={() => {
                                    toggleSort("osType");
                                }}
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer {sort ===
                                'osType.desc'
                                    ? SORT_COLOR
                                    : ''}"
                                on:click={() => {
                                    toggleSort("osType");
                                }}
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
                                class="sort-icon mouse-pointer {sort ===
                                'lastSeen.asc'
                                    ? SORT_COLOR
                                    : ''}"
                                on:click={() => {
                                    toggleSort("lastSeen");
                                }}
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer {sort ===
                                'lastSeen.desc'
                                    ? SORT_COLOR
                                    : ''}"
                                on:click={() => {
                                    toggleSort("lastSeen");
                                }}
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
                                class="sort-icon mouse-pointer {sort ===
                                'conformitySuccess.asc'
                                    ? SORT_COLOR
                                    : 'text-success'}"
                                on:click={() => {
                                    toggleSort("conformitySuccess");
                                }}
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer {sort ===
                                'conformitySuccess.desc'
                                    ? SORT_COLOR
                                    : 'text-success'}"
                                on:click={() => {
                                    toggleSort("conformitySuccess");
                                }}
                            />
                        </div>
                        <div class="col-auto align-self-center d-grid">
                            <Icon
                                size="1.125"
                                name="chevron-up"
                                class="sort-icon mouse-pointer {sort ===
                                'conformityIncorrect.asc'
                                    ? SORT_COLOR
                                    : 'text-warning'}"
                                on:click={() => {
                                    toggleSort("conformityIncorrect");
                                }}
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer {sort ===
                                'conformityIncorrect.desc'
                                    ? SORT_COLOR
                                    : 'text-warning'}"
                                on:click={() => {
                                    toggleSort("conformityIncorrect");
                                }}
                            />
                        </div>
                        <div class="col-auto align-self-center d-grid">
                            <Icon
                                size="1.125"
                                name="chevron-up"
                                class="sort-icon mouse-pointer {sort ===
                                'conformityError.asc'
                                    ? SORT_COLOR
                                    : 'text-danger'}"
                                on:click={() => {
                                    toggleSort("conformityError");
                                }}
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer {sort ===
                                'conformityError.desc'
                                    ? SORT_COLOR
                                    : 'text-danger'}"
                                on:click={() => {
                                    toggleSort("conformityError");
                                }}
                            />
                        </div>
                    </div></th
                >
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
                        <td>{minion.osType}</td>
                        <td>{minion.lastSeen}</td>
                        <td>
                            {#if minion.lastUpdatedConformity === null}
                                <span class="badge bg-purple"> Unknown </span>
                            {:else}
                                <span class="badge bg-success">
                                    {minion.conformitySuccess ?? "?"}
                                </span>
                                /
                                <span class="badge bg-warning">
                                    {minion.conformityIncorrect ?? "?"}
                                </span>
                                /
                                <span class="badge bg-danger">
                                    {minion.conformityError ?? "?"}
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
