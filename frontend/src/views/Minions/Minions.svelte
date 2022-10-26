<script lang="ts">
    import { Button, Card, Table } from 'sveltestrap';
    import { getMinions, showToast } from '../../controller';
    import { Link, type NavigateFn } from 'svelte-navigator';
    import { MessageType } from '../../models/MessageType';
    import { onMount } from 'svelte';
    import { refreshMinions } from '../../controller';
    import { SortOrder } from '../../models/SortOrder';
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

    // svelte-ignore unused-export-let
    export let location: Location;
    export let navigate: NavigateFn;

    let filters: Filter[] = [];
    let sortField: string | null = null;
    let sortOrder: SortOrder = SortOrder.Up;
    let paginationSize: number = 20;
    let paginationPage: number = 1;

    const SORT_COLOR = `text-orange sort-active`;
    const minions: Writable<Minion[] | null> = writable(null);

    function updateData(): void {
        getMinions(
            filters,
            sortField === null ? null : sortField + '.' + sortOrder,
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

    function toggleSort(field: string, order: SortOrder): void {
        if (sortField === null) {
            sortField = field;
            sortOrder = order;
        } else if (sortField === field) {
            if (order !== sortOrder) {
                sortOrder = order;
            } else {
                sortField = null;
                sortOrder = SortOrder.Up;
            }
        } else {
            sortField = field;
            sortOrder = order;
        }

        console.log('toggleSort', field, order, sortField, sortOrder);

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
                        <div class="col-auto">
                            <Icon
                                size="1.25"
                                name="chevron-up"
                                class="sort-icon-up {sortField === 'id' &&
                                sortOrder === SortOrder.Up
                                    ? SORT_COLOR
                                    : ''}"
                                on:click={() => {
                                    toggleSort('id', SortOrder.Up);
                                }}
                            />
                            <Icon
                                size="1.25"
                                name="chevron-down"
                                class="sort-icon-down {sortField === 'id' &&
                                sortOrder === SortOrder.Down
                                    ? SORT_COLOR
                                    : ''}"
                                on:click={() => {
                                    toggleSort('id', SortOrder.Down);
                                }}
                            />
                        </div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">OS</div>
                        <div class="col-auto">
                            <Icon
                                size="1.25"
                                name="chevron-up"
                                class="sort-icon-up {sortField === 'osType'
                                    ? sortOrder === SortOrder.Up
                                        ? SORT_COLOR
                                        : ''
                                    : ''}"
                                on:click={() => {
                                    toggleSort('osType', SortOrder.Up);
                                }}
                            />
                            <Icon
                                size="1.25"
                                name="chevron-down"
                                class="sort-icon-down {sortField === 'osType'
                                    ? sortOrder === SortOrder.Down
                                        ? SORT_COLOR
                                        : ''
                                    : ''}"
                                on:click={() => {
                                    toggleSort('osType', SortOrder.Down);
                                }}
                            />
                        </div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Last seen</div>
                        <div class="col-auto">
                            <Icon
                                size="1.25"
                                name="chevron-up"
                                class="sort-icon-up {sortField === 'lastSeen' &&
                                sortOrder === SortOrder.Up
                                    ? SORT_COLOR
                                    : ''}"
                                on:click={() => {
                                    toggleSort('lastSeen', SortOrder.Up);
                                }}
                            />
                            <Icon
                                size="1.25"
                                name="chevron-down"
                                class="sort-icon-down {sortField ===
                                    'lastSeen' && sortOrder === SortOrder.Down
                                    ? SORT_COLOR
                                    : ''}"
                                on:click={() => {
                                    toggleSort('lastSeen', SortOrder.Down);
                                }}
                            />
                        </div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Conformity</div>
                        <div class="col-auto">
                            <Icon
                                size="1.25"
                                name="chevron-up"
                                class="sort-icon-up {sortField ===
                                    'conformitySuccess' &&
                                sortOrder === SortOrder.Up
                                    ? SORT_COLOR
                                    : 'text-success'}"
                                on:click={() => {
                                    toggleSort(
                                        'conformitySuccess',
                                        SortOrder.Up,
                                    );
                                }}
                            />
                            <Icon
                                size="1.25"
                                name="chevron-down"
                                class="sort-icon-down {sortField ===
                                    'conformitySuccess' &&
                                sortOrder === SortOrder.Down
                                    ? SORT_COLOR
                                    : 'text-success'}"
                                on:click={() => {
                                    toggleSort(
                                        'conformitySuccess',
                                        SortOrder.Down,
                                    );
                                }}
                            />
                        </div>
                        <div class="col-auto">
                            <Icon
                                size="1.25"
                                name="chevron-up"
                                class="sort-icon-up {sortField ===
                                    'conformityIncorrect' &&
                                sortOrder === SortOrder.Up
                                    ? SORT_COLOR
                                    : 'text-warning'}"
                                on:click={() => {
                                    toggleSort(
                                        'conformityIncorrect',
                                        SortOrder.Up,
                                    );
                                }}
                            />
                            <Icon
                                size="1.25"
                                name="chevron-down"
                                class="sort-icon-down {sortField ===
                                    'conformityIncorrect' &&
                                sortOrder === SortOrder.Down
                                    ? SORT_COLOR
                                    : 'text-warning'}"
                                on:click={() => {
                                    toggleSort(
                                        'conformityIncorrect',
                                        SortOrder.Down,
                                    );
                                }}
                            />
                        </div>
                        <div class="col-auto">
                            <Icon
                                size="1.25"
                                name="chevron-up"
                                class="sort-icon-up {sortField ===
                                    'conformityError' &&
                                sortOrder === SortOrder.Up
                                    ? SORT_COLOR
                                    : 'text-danger'}"
                                on:click={() => {
                                    toggleSort('conformityError', SortOrder.Up);
                                }}
                            />
                            <Icon
                                size="1.25"
                                name="chevron-down"
                                class="sort-icon-down {sortField ===
                                    'conformityError' &&
                                sortOrder === SortOrder.Down
                                    ? SORT_COLOR
                                    : 'text-danger'}"
                                on:click={() => {
                                    toggleSort(
                                        'conformityError',
                                        SortOrder.Down,
                                    );
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
