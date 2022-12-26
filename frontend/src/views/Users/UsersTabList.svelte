<script lang="ts">
    import { onMount } from 'svelte';
    import { theme, currentUser, toasts } from '../../stores';
    import { getUsers } from '../../api';
    import { Badge, Card, Table } from 'sveltestrap';
    import { writable, type Writable } from 'svelte/store';
    import TablePaginate from '../../components/TablePaginate.svelte';
    import paths from '../../paths';
    import { Link, type NavigateFn } from 'svelte-navigator';
    import { MessageType } from '../../models/MessageType';
    import type User from '../../models/User';
	import Clickable from '../../components/Clickable.svelte';

    // svelte-ignore unused-export-let
    export let location: Location;
    export let navigate: NavigateFn;

    let paginationSize: number = 20;
    let paginationPage: number = 1;

    const users: Writable<User[] | null> = writable(null);

    function updateData() {
        getUsers(paginationSize, (paginationPage - 1) * paginationSize)
            .then((data) => {
                users.set(data);
            })
            .catch((err) => {
                toasts.add(MessageType.ERROR, 'Failed fetching users', err);
            });
    }

    onMount(() => {
        updateData();
    });
</script>

Search box here.

<hr class="bg-light" />

<Card class="table-responsive border-bottom-0">
    <Table hover class="b-0 mb-0">
        <thead class="bg-dark border-0 text-white">
            <tr>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center ps-2">User</div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">ID</div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">LDAP</div>
                    </div>
                </th>
                <th class="border-secondary" />
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
                        <Clickable
                            event={() => navigate(paths.user.getPath(user.id))}
                            type="th"
                        >
                            <Link to={paths.user.getPath(user.id)} class="text-decoration-none text-reset">
                                {user.username}
                                {#if user.id === $currentUser.id}
                                    <span class="text-{$theme.color}"> (You)</span>
                                {/if}
                            </Link>
                        </Clickable>
                        <td>{user.id}</td>
                        <td>
                            {#if user.ldapSync !== null}
                                <Badge color={null} class="bg-{$theme.color}"
                                    >Yes</Badge
                                >
                            {:else}
                                <Badge
                                    color={$theme.dark ? 'secondary' : 'dark'}
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
