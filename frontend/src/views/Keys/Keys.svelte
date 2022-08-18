<script lang="ts">
    import { onMount } from "svelte";
    import { theme } from "../../stores";
    import { AlertType, getKeys, showAlert } from "../../controller";
    import { Badge, Button, Table } from "sveltestrap";
    import { writable } from "svelte/store";
    import TablePaginate from "../../components/TablePaginate.svelte";
    import paths from "../../paths";
    import { Link } from "svelte-navigator";

    let paginationSize: number = 20;
    let paginationPage: number = 1;
    let keysView = [];

    const keys = writable(null);

    function updateData() {
        getKeys()
            .then((data) => {
                keys.set(data);
                fakePaginate();
            })
            .catch((err) => {
                showAlert(AlertType.ERROR, "Failed fetching keys", err);
            });
    }

    function fakePaginate() {
        keysView = $keys.slice(
            paginationSize * (paginationPage - 1),
            paginationSize * paginationPage
        );
    }

    function onClickAccept(finger: string) {
        showAlert(AlertType.SUCCESS, "Key accepted", `Key ${finger} accepted`);
    }

    function onClickReject(finger: string) {
        showAlert(AlertType.SUCCESS, "Key rejected", `Key ${finger} rejected`);
    }

    function onClickDelete(finger: string) {
        showAlert(AlertType.SUCCESS, "Key deleted", `Key ${finger} deleted`);
    }

    onMount(() => {
        updateData();
    });
</script>

<h1>Key Management</h1>

<div class="table-responsive card {$theme.dark ? 'bg-dark' : ''}">
    <Table
        dark={$theme.dark}
        hover
        class="b-0 mb-0 {$theme.dark ? 'text-light border-secondary' : ''}"
    >
        <thead
            class="bg-dark border-0 {$theme.dark ? 'text-light' : 'text-white'}"
        >
            <tr>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center ps-2">ID</div>
                    </div>
                </th>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Status</div>
                    </div>
                </th>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Public Key</div>
                    </div>
                </th>
                <th scope="col" class="border-secondary" />
            </tr>
        </thead>
        <tbody class="align-middle">
            {#if $keys == null}
                <p>Loading</p>
            {:else if $keys.length == 0}
                <div class="p-3">No users exist. How are you seeing this?</div>
            {:else}
                {#each keysView as key}
                    <tr>
                        <th scope="row">
                            <Link
                                to={paths.minion.getPath(key.id)}
                                class="text-reset text-decoration-none"
                            >
                                {key.id}
                            </Link>
                        </th>
                        <td>
                            {#if key.status == "accepted"}
                                <Badge color="success">Accepted</Badge>
                            {:else if key.status == "pre"}
                                <Badge color="danger">Unaccepted</Badge>
                            {:else if key.status == "rejected"}
                                <Badge color="warning">Rejected</Badge>
                            {:else if key.status == "denied"}
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
                            {#if key.status == "accepted"}
                                <Button
                                    color="warning"
                                    size="sm"
                                    class="key-btn me-1"
                                    on:click={() => {
                                        onClickReject(key.finger);
                                    }}>Reject</Button
                                >{:else if key.status == "pre"}
                                <Button
                                    color="success"
                                    size="sm"
                                    class="key-btn me-1"
                                    on:click={() => {
                                        onClickAccept(key.finger);
                                    }}>Accept</Button
                                >{:else if key.status == "rejected"}
                                <Button
                                    color="success"
                                    size="sm"
                                    class="key-btn me-1"
                                    on:click={() => {
                                        onClickAccept(key.finger);
                                    }}>Accept</Button
                                >{:else if key.status == "denied"}
                                <Button
                                    color={null}
                                    size="sm"
                                    class="key-btn me-1 btn-orange"
                                    on:click={() => {
                                        onClickAccept(key.finger);
                                    }}>Force Accept</Button
                                >{/if}<Button
                                color="danger"
                                size="sm"
                                class="key-btn"
                                on:click={() => {
                                    onClickDelete(key.finger);
                                }}>Delete</Button
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
    last={$keys == null || $keys.length < paginationSize}
    updateData={fakePaginate}
/>

<style>
    :global(.key-btn) {
        width: 4.5rem;
    }
</style>
