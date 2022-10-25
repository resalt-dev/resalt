<script lang="ts">
    import { Button, Card, Table } from 'sveltestrap';
    import { getMinions, showToast } from '../../controller';
    import { Link, type NavigateFn } from 'svelte-navigator';
    import { MessageType } from '../../models/MessageType';
    import { onMount } from 'svelte';
    import { refreshMinions } from '../../controller';
    import { theme } from '../../stores';
    import { writable, type Writable } from 'svelte/store';
    import Icon from '../../components/Icon.svelte';
    import MinionsTabGroups from './MinionsTabGroups.svelte';
    import MinionsTabSearch from './MinionsTabSearch.svelte';
    import paths from '../../paths';
    import TablePaginate from '../../components/TablePaginate.svelte';
    import Tabs from '../../components/Tabs.svelte';
    import type Filter from '../../models/Filter';
    import type Minion from '../../models/Minion';

    export let navigate: NavigateFn;

    let filters: Filter[] = [];
    let sort: string = null;
    let paginationSize: number = 20;
    let paginationPage: number = 1;

    const SORT_COLOR: string = 'text-orange';
    const minions: Writable<Minion[] | null> = writable(null);

    function updateData(): void {
        getMinions(
            filters,
            sort,
            paginationSize,
            (paginationPage - 1) * paginationSize,
        )
            .then((data) => {
                minions.set(data);
            })
            .catch((err) => {
                showToast(MessageType.ERROR, 'Failed fetching minions', err);
            });
    }

    function setFilters(newFilters: Filter[]): void {
        filters = newFilters;
        updateData();
    }

    function toggleSort(field: string): void {
        // field.order
        let parts = (sort || 'null.null').split('.');
        if (parts[0] === field) {
            switch (parts[1]) {
                case 'asc':
                    sort = field + '.desc';
                    break;
                case 'desc':
                    sort = null;
                    break;
                default:
                    sort = field + '.asc';
                    break;
            }
        } else {
            sort = field + '.asc';
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
            label: 'Search',
            component: MinionsTabSearch,
            data: { setFilters },
        },
        {
            label: 'Groups',
            component: MinionsTabGroups,
        },
    ]}
/>

<Card class="table-responsive border-bottom-0">
    <Table hover class="b-0 mb-0">
        <thead class="bg-dark border-0 text-white">
            <tr>
                <th class="border-secondary">
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
                                    toggleSort('id');
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
                                    toggleSort('id');
                                }}
                            />
                        </div>
                    </div>
                </th>
                <th class="border-secondary">
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
                                    toggleSort('osType');
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
                                    toggleSort('osType');
                                }}
                            />
                        </div>
                    </div>
                </th>
                <th class="border-secondary">
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
                                    toggleSort('lastSeen');
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
                                    toggleSort('lastSeen');
                                }}
                            />
                        </div>
                    </div>
                </th>
                <th class="border-secondary">
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
                                    toggleSort('conformitySuccess');
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
                                    toggleSort('conformitySuccess');
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
                                    toggleSort('conformityIncorrect');
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
                                    toggleSort('conformityIncorrect');
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
                                    toggleSort('conformityError');
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
                                    toggleSort('conformityError');
                                }}
                            />
                        </div>
                    </div></th
                >
                <th class="border-secondary">Actions</th>
            </tr>
        </thead>
        <tbody class="align-middle">
            {#if $minions === null}
                <p>Loading</p>
            {:else if $minions.length === 0 && paginationPage === 1}
                <div class="p-3">No minions returned.</div>
            {:else}
                {#each $minions as minion}
                    <tr>
                        <th
                            class="mouse-pointer"
                            on:click={() =>
                                navigate(paths.minion.getPath(minion.id))}
                        >
                            {minion.id}
                        </th>
                        <td>{minion.osType ?? 'Unknown'}</td>
                        <td>{minion.lastSeen}</td>
                        <td>
                            {#if minion.lastUpdatedConformity === null}
                                <span class="badge bg-purple"> Unknown </span>
                            {:else}
                                <span class="badge bg-success">
                                    {minion.conformitySuccess ?? '?'}
                                </span>
                                /
                                <span class="badge bg-warning">
                                    {minion.conformityIncorrect ?? '?'}
                                </span>
                                /
                                <span class="badge bg-danger">
                                    {minion.conformityError ?? '?'}
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
</Card>

<TablePaginate
    bind:size={paginationSize}
    bind:page={paginationPage}
    last={$minions === null || $minions.length < paginationSize}
    {updateData}
/>

<br />

<Button color="secondary" size="sm" on:click={() => refreshMinions()}>
    Force reload minions
</Button>
