<script lang="ts">
    import { onMount } from 'svelte';
    import { Button, Card, Table } from 'sveltestrap';
    import { Link, type NavigateFn } from 'svelte-navigator';
    import { MessageType } from '../../models/MessageType';
    import { refreshMinions, getMinions } from '../../api';
    import { SortOrder } from '../../models/SortOrder';
    import { theme } from '../../stores';
    import { toasts } from '../../stores';
    import { writable, type Writable } from 'svelte/store';
    import Icon from '../../components/Icon.svelte';
    import paths from '../../paths';
    import ResaltProgress from '../../components/ResaltProgress.svelte';
    import TablePaginate from '../../components/TablePaginate.svelte';
    import Tabs from '../../components/Tabs.svelte';
    import type Filter from '../../models/Filter';
    import type Minion from '../../models/Minion';
    import type TabPage from '../../models/TabPage';

    import MinionsTabPresets from './MinionsTabPresets.svelte';
    import MinionsTabSearch from './MinionsTabSearch.svelte';
    import SortIcon from '../../components/SortIcon.svelte';

    // svelte-ignore unused-export-let
    export let location: Location;
    export let navigate: NavigateFn;
    export let subPage: string = '';

    const loading = writable<boolean>(true);

    let filters: Filter[] = [];
    let sortField: string | null = null;
    let sortOrder: SortOrder = SortOrder.Down;
    let paginationSize: number = 20;
    let paginationPage: number = 1;
    $: active = sortField + ':' + sortOrder;

    const minions: Writable<Minion[] | null> = writable(null);

    function updateData(): void {
        loading.set(true);
        getMinions(
            filters,
            sortField === null ? null : sortField + '.' + sortOrder,
            paginationSize,
            (paginationPage - 1) * paginationSize,
        )
            .then((data) => {
                minions.set(data);
                loading.set(false);
            })
            .catch((err) => {
                toasts.add(MessageType.ERROR, 'Failed fetching minions', err);
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
                sortOrder = SortOrder.Down;
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

    let tabs: TabPage[] = [];
    $: tabs = [
        {
            key: 'search',
            label: 'Search',
            path: paths.minions.getPath('search'),
            component: MinionsTabSearch,
            data: { setFilters },
        },
        {
            key: 'presets',
            label: 'Presets',
            path: paths.minions.getPath('presets'),
            component: MinionsTabPresets,
        },
    ];
</script>

<h1>Minions</h1>

<Tabs {tabs} current={subPage} />

<Card class="table-responsive border-bottom-0">
    <Table hover class="b-0 mb-0">
        <thead class="bg-dark border-0 text-white">
            <tr>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center ps-2">ID</div>
                        <div class="col-auto">
                            <SortIcon
                                {active}
                                field="id"
                                order={SortOrder.Down}
                                click={toggleSort}
                            />
                            <SortIcon
                                {active}
                                field="id"
                                order={SortOrder.Up}
                                click={toggleSort}
                            />
                        </div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">OS</div>
                        <div class="col-auto">
                            <SortIcon
                                {active}
                                field="osType"
                                order={SortOrder.Down}
                                click={toggleSort}
                            />
                            <SortIcon
                                {active}
                                field="osType"
                                order={SortOrder.Up}
                                click={toggleSort}
                            />
                        </div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Last seen</div>
                        <div class="col-auto">
                            <SortIcon
                                {active}
                                field="lastSeen"
                                order={SortOrder.Down}
                                click={toggleSort}
                            />
                            <SortIcon
                                {active}
                                field="lastSeen"
                                order={SortOrder.Up}
                                click={toggleSort}
                            />
                        </div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Conformity</div>
                        <div class="col-auto">
                            <SortIcon
                                {active}
                                field="conformitySuccess"
                                order={SortOrder.Down}
                                click={toggleSort}
                                color="success"
                            />
                            <SortIcon
                                {active}
                                field="conformitySuccess"
                                order={SortOrder.Up}
                                click={toggleSort}
                                color="success"
                            />
                        </div>
                        <div class="col-auto">
                            <SortIcon
                                {active}
                                field="conformityIncorrect"
                                order={SortOrder.Down}
                                click={toggleSort}
                                color="warning"
                            />
                            <SortIcon
                                {active}
                                field="conformityIncorrect"
                                order={SortOrder.Up}
                                click={toggleSort}
                                color="warning"
                            />
                        </div>
                        <div class="col-auto">
                            <SortIcon
                                {active}
                                field="conformityError"
                                order={SortOrder.Down}
                                click={toggleSort}
                                color="danger"
                            />
                            <SortIcon
                                {active}
                                field="conformityError"
                                order={SortOrder.Up}
                                click={toggleSort}
                                color="danger"
                            />
                        </div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Actions</div>
                        <div class="col align-self-bottom canRotate">
                            <Icon
                                size="1.5"
                                name="refresh"
                                class="float-end hover-icon mouse-pointer"
                                on:click={() => {
                                    updateData();
                                }}
                            />
                        </div>
                    </div>
                </th>
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
                            >
                                View
                            </Link>
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

{#if $loading}
    <ResaltProgress />
{/if}

<br />

<Button color="secondary" size="sm" on:click={() => refreshMinions()}>
    Force reload minions
</Button>
