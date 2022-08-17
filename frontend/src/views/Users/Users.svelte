<script lang="ts">
    import { onMount } from "svelte";
    import { theme } from "../../stores";
    import { AlertType, getUsers, showAlert } from "../../controller";
    import { Table } from "sveltestrap";
    import Icon from "../../components/Icon.svelte";
    import { writable } from "svelte/store";
    import TablePaginate from "../../components/TablePaginate.svelte";

    let paginationSize: number = 20;
    let paginationPage: number = 1;

    const users = writable(null);

    function updateData() {
        getUsers(paginationSize, (paginationPage - 1) * paginationSize)
            .then((data) => {
                users.set(data);
            })
            .catch((err) => {
                showAlert(AlertType.ERROR, "Failed fetching users", err);
            });
    }

    onMount(() => {
        updateData();
    });
</script>

<h1>Users</h1>

<div class="table-responsive card {$theme.dark ? 'bg-dark' : ''}">
    <Table
        dark={$theme.dark}
        hover
        id="jobListTable"
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
                        <div class="col-auto align-self-center">User</div>
                    </div>
                </th>
            </tr>
        </thead>
        <tbody class="align-middle">
            {#if $users == null}
                <p>Loading</p>
            {:else if $users.length == 0}
                <div class="p-3">No users exist. How are you seeing this?</div>
            {:else}
                {#each $users as user}
                    <tr>
                        <th scope="row">{user.id}</th>
                        <td>{user.username}</td>
                        <!--<td><small>{user.timestamp}</small></td>-->
                    </tr>
                {/each}
            {/if}
        </tbody>
    </Table>
</div>

<TablePaginate
    bind:size={paginationSize}
    bind:page={paginationPage}
    last={$users == null || $users.length < paginationSize}
    {updateData}
/>
