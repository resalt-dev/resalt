<script lang="ts">
	import { goto } from '$app/navigation';
	import Clickable from '$component/Clickable.svelte';
	import TablePaginate from '$component/TablePaginate.svelte';
	import { deleteUser, getUsers } from '$lib/api';
	import paths from '$lib/paths';
	import { P_USER_ADMIN, hasResaltPermission } from '$lib/perms';
	import { currentUser, theme, toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';
	import type User from '$model/User';
	import { onMount } from 'svelte';
	import { writable, type Writable } from 'svelte/store';

	let paginationSize = 20;
	let paginationPage = 1;

	const users: Writable<User[] | null> = writable(null);

	function updateData() {
		getUsers(paginationSize, (paginationPage - 1) * paginationSize)
			.then((data: User[] | null) => {
				users.set(data);
			})
			.catch((err: unknown) => {
				toasts.add(MessageType.ERROR, 'Failed fetching users', err);
			});
	}

	function _deleteUser(userId: string): void {
		if (!confirm('Are you sure you want to delete this user?')) {
			return;
		}

		deleteUser(userId)
			.then(() => {
				toasts.add(MessageType.SUCCESS, 'User deleted', `User ${userId} deleted`);
				updateData();
			})
			.catch((err: unknown) => {
				toasts.add(MessageType.ERROR, 'Failed deleting user', err);
			});
	}

	onMount(() => {
		updateData();
	});
</script>

<svelte:head>
	<title>Users</title>
</svelte:head>

Search box here.

<hr class="text-light" />

<div class="card table-responsive border-bottom-0">
	<table class="table table-hover b-0 mb-0">
		<thead class="border-0">
			<tr>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center ps-2">User</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center">ID</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center">LDAP</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white" />
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
							event={() => goto(paths.user_info.getPath({ userId: user.id }))}
							type="th"
						>
							<a
								href={paths.user_info.getPath({ userId: user.id })}
								class="text-decoration-none text-reset"
							>
								{user.username}
								{#if user.id === $currentUser?.id}
									<span class="text-{$theme.color}"> (You)</span>
								{/if}
							</a>
						</Clickable>
						<td>{user.id}</td>
						<td>
							{#if user.ldapSync !== null}
								<span class="badge bg-{$theme.color}"> Yes </span>
							{:else}
								<span class="badge bg-{$theme.dark ? 'secondary' : 'dark'}">
									No
								</span>
							{/if}
						</td>
						<td>
							<a
								href={paths.user_info.getPath({ userId: user.id })}
								class="btn btn-{$theme.color} btn-sm px-3 me-2"
							>
								View
							</a>
							{#if hasResaltPermission($currentUser, P_USER_ADMIN)}
								<Clickable
									type="button"
									event={() => _deleteUser(user.id)}
									class="btn btn-danger btn-sm px-3 me-2"
									disabled={user.username === 'admin'}
								>
									Delete
								</Clickable>
							{/if}
						</td>
					</tr>
				{/each}
			{/if}
		</tbody>
	</table>
</div>

<TablePaginate
	bind:size={paginationSize}
	bind:page={paginationPage}
	last={$users === null || $users.length < paginationSize}
	{updateData}
/>
