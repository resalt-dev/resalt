<script lang="ts">
    import { onMount } from 'svelte';
    import { acceptKey, deleteKey, getKeys, rejectKey } from '../../api';
    import { toasts } from '../../stores';
    import { Badge, Button, Card, Table } from 'sveltestrap';
    import { writable, type Writable } from 'svelte/store';
    import TablePaginate from '../../components/TablePaginate.svelte';
    import paths from '../../paths';
    import type Key from '../../models/Key';
    import { MessageType } from '../../models/MessageType';
    import type { NavigateFn } from 'svelte-navigator';

    // svelte-ignore unused-export-let
    export let location: Location;
    export let navigate: NavigateFn;

    let paginationSize: number = 20;
    let paginationPage: number = 1;
    let keysView: Key[] = [];

    const keys: Writable<Key[] | null> = writable(null);

    function updateData() {
        getKeys()
            .then((data) => {
                keys.set(data);
                fakePaginate();
            })
            .catch((err) => {
                toasts.add(MessageType.ERROR, 'Failed fetching keys', err);
            });
    }

    function fakePaginate() {
        keysView = $keys.slice(
            paginationSize * (paginationPage - 1),
            paginationSize * paginationPage,
        );
    }

    function onClickAccept(key: Key) {
        acceptKey(key)
            .then(() => {
                updateData();
                toasts.add(
                    MessageType.SUCCESS,
                    'Key accepted',
                    `Key ${key.id} accepted`,
                );
            })
            .catch((err) => {
                toasts.add(MessageType.ERROR, 'Failed accepting key', err);
            });
    }

    function onClickReject(key: Key) {
        rejectKey(key)
            .then(() => {
                updateData();
                toasts.add(
                    MessageType.SUCCESS,
                    'Key rejected',
                    `Key ${key.id} rejected`,
                );
            })
            .catch((err) => {
                toasts.add(MessageType.ERROR, 'Failed rejecting key', err);
            });
    }

    function onClickDelete(key: Key) {
        deleteKey(key)
            .then(() => {
                updateData();
                toasts.add(
                    MessageType.SUCCESS,
                    'Key deleted',
                    `Key ${key.id} deleted`,
                );
            })
            .catch((err) => {
                toasts.add(MessageType.ERROR, 'Failed deleting key', err);
            });
    }

    onMount(() => {
        updateData();
    });
</script>

<h1>Key Management</h1>

<Card class="table-responsive border-bottom-0">
    <Table hover class="b-0 mb-0">
        <thead class="bg-dark border-0 text-white">
            <tr>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center ps-2">ID</div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Status</div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Public Key</div>
                    </div>
                </th>
                <th class="border-secondary" />
            </tr>
        </thead>
        <tbody class="align-middle">
            {#if $keys === null}
                <p>Loading</p>
            {:else if $keys.length === 0 && paginationPage === 1}
                <div class="p-3">No keys exist.</div>
            {:else}
                {#each keysView as key}
                    <tr>
                        <th
                            class="mouse-pointer"
                            on:click={() =>
                                navigate(paths.minion.getPath(key.id))}
                        >
                            {key.id}
                        </th>
                        <td>
                            {#if key.state === 'minions'}
                                <Badge color="success">Accepted</Badge>
                            {:else if key.state === 'minions_pre'}
                                <Badge color="danger">Unaccepted</Badge>
                            {:else if key.state === 'minions_rejected'}
                                <Badge color="warning">Rejected</Badge>
                            {:else if key.state === 'minions_denied'}
                                <Badge color={null} class="bg-purple"
                                    >Denied</Badge
                                >
                            {:else}
                                <Badge color={null} class="bg-secondary"
                                    >Unknown</Badge
                                >
                            {/if}
                        </td>
                        <td>{key.finger}</td>
                        <td>
                            {#if key.state === 'minions'}
                                <Button
                                    color="warning"
                                    size="sm"
                                    class="key-btn me-1"
                                    on:click={() => {
                                        onClickReject(key);
                                    }}>Reject</Button
                                >{:else if key.state === 'minions_pre'}
                                <Button
                                    color="success"
                                    size="sm"
                                    class="key-btn me-1"
                                    on:click={() => {
                                        onClickAccept(key);
                                    }}>Accept</Button
                                >{:else if key.state === 'minions_rejected'}
                                <Button
                                    color="success"
                                    size="sm"
                                    class="key-btn me-1"
                                    on:click={() => {
                                        onClickAccept(key);
                                    }}>Accept</Button
                                >{:else if key.state === 'minions_denied'}
                                <Button
                                    color="success"
                                    size="sm"
                                    class="key-btn me-1"
                                    on:click={() => {
                                        onClickAccept(key);
                                    }}>Accept</Button
                                >{/if}<Button
                                color="danger"
                                size="sm"
                                class="key-btn"
                                on:click={() => {
                                    onClickDelete(key);
                                }}>Delete</Button
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
    last={$keys === null || $keys.length < paginationSize}
    updateData={fakePaginate}
/>

<style>
    :global(.key-btn) {
        width: 4.5rem;
    }
</style>
