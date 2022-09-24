<script lang="ts">
    import { onMount } from 'svelte';
    import { theme } from '../../stores';
    import {
        acceptKey,
        deleteKey,
        getKeys,
        rejectKey,
        showAlert,
    } from '../../controller';
    import { Badge, Button, Card, Table } from 'sveltestrap';
    import { writable, type Writable } from 'svelte/store';
    import TablePaginate from '../../components/TablePaginate.svelte';
    import paths from '../../paths';
    import { Link } from 'svelte-navigator';
    import type Key from '../../models/Key';
    import { AlertType } from '../../models/AlertType';

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
                showAlert(AlertType.ERROR, 'Failed fetching keys', err);
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
                showAlert(
                    AlertType.SUCCESS,
                    'Key accepted',
                    `Key ${key.id} accepted`,
                );
            })
            .catch((err) => {
                showAlert(AlertType.ERROR, 'Failed accepting key', err);
            });
    }

    function onClickReject(key: Key) {
        rejectKey(key)
            .then(() => {
                updateData();
                showAlert(
                    AlertType.SUCCESS,
                    'Key rejected',
                    `Key ${key.id} rejected`,
                );
            })
            .catch((err) => {
                showAlert(AlertType.ERROR, 'Failed rejecting key', err);
            });
    }

    function onClickDelete(key: Key) {
        deleteKey(key)
            .then(() => {
                updateData();
                showAlert(
                    AlertType.SUCCESS,
                    'Key deleted',
                    `Key ${key.id} deleted`,
                );
            })
            .catch((err) => {
                showAlert(AlertType.ERROR, 'Failed deleting key', err);
            });
    }

    onMount(() => {
        updateData();
    });
</script>

<h1>Key Management</h1>

<Card class="table-responsive border-bottom-0 {$theme.dark ? 'bg-dark' : ''}">
    <Table
        dark={$theme.dark}
        hover
        class="b-0 mb-0 {$theme.dark ? 'text-light border-secondary' : ''}"
    >
        <thead
            class="bg-dark border-0 {$theme.dark ? 'text-light' : 'text-white'}"
        >
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
                        <th>
                            <Link
                                to={paths.minion.getPath(key.id)}
                                class="text-reset text-decoration-none"
                            >
                                {key.id}
                            </Link>
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
