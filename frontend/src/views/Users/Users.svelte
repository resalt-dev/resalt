<script lang="ts">
    import { onMount } from "svelte";
    import { theme, currentUser } from "../../stores";
    import { getUsers, showAlert } from "../../controller";
    import { Badge, Card, Table } from "sveltestrap";
    import { writable } from "svelte/store";
    import TablePaginate from "../../components/TablePaginate.svelte";
    import paths from "../../paths";
    import { Link } from "svelte-navigator";
    import { AlertType } from "../../models/AlertType";

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
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center ps-2">User</div>
                    </div>
                </th>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">ID</div>
                    </div>
                </th>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Is Local</div>
                    </div>
                </th>
                <th scope="col" class="border-secondary" />
            </tr>
        </thead>
        <tbody class="align-middle">
            {#if $users === null}
                <p>Loading</p>
            {:else if $users.length === 0 && paginationPage === 1}
                <div class="p-3">No users exist. How are you seeing this?</div>
            {:else}
                {#each $users as user}
                    <tr>
                        <th scope="row">
                            <Link
                                to={paths.user.getPath(user.id)}
                                class="text-reset text-decoration-none"
                            >
                                {user.username}
                                {#if user.id === $currentUser.id}
                                    <span class="text-{$theme.color}">
                                        (You)</span
                                    >
                                {/if}
                            </Link>
                        </th>
                        <td>{user.id}</td>
                        <td>
                            {#if user.isLocal}
                                <Badge
                                    color={$theme.dark ? "secondary" : "dark"}
                                    >Yes</Badge
                                >
                            {:else}
                                <Badge color={null} class="bg-{$theme.color}"
                                    >No</Badge
                                >
                            {/if}
                        </td>
                        <td>
                            <Link
                                to={paths.user.getPath(user.id)}
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
    last={$users === null || $users.length < paginationSize}
    {updateData}
/>
