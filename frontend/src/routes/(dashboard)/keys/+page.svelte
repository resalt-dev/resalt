<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { acceptKey, deleteKey, getKeys, rejectKey } from '$lib/api';
	import { currentUser, toasts } from '$lib/stores';
	import { writable, type Writable } from 'svelte/store';
	import TablePaginate from '../../../components/TablePaginate.svelte';
	import paths from '$lib/paths';
	import type Key from '../../../models/Key';
	import { MessageType } from '../../../models/MessageType';
	import {
		hasResaltPermission,
		P_SALTKEY_ACCEPT,
		P_SALTKEY_DELETE,
		P_SALTKEY_REJECT,
	} from '$lib/perms';

	let paginationSize = 20;
	let paginationPage = 1;
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
		if ($keys === null) {
			return;
		}
		keysView = $keys.slice(
			paginationSize * (paginationPage - 1),
			paginationSize * paginationPage,
		);
	}

	function onClickAccept(key: Key) {
		acceptKey(key)
			.then(() => {
				updateData();
				toasts.add(MessageType.SUCCESS, 'Key accepted', `Key ${key.id} accepted`);
			})
			.catch((err) => {
				toasts.add(MessageType.ERROR, 'Failed accepting key', err);
			});
	}

	function onClickReject(key: Key) {
		rejectKey(key)
			.then(() => {
				updateData();
				toasts.add(MessageType.SUCCESS, 'Key rejected', `Key ${key.id} rejected`);
			})
			.catch((err) => {
				toasts.add(MessageType.ERROR, 'Failed rejecting key', err);
			});
	}

	function onClickDelete(key: Key) {
		deleteKey(key)
			.then(() => {
				updateData();
				toasts.add(MessageType.SUCCESS, 'Key deleted', `Key ${key.id} deleted`);
			})
			.catch((err) => {
				toasts.add(MessageType.ERROR, 'Failed deleting key', err);
			});
	}

	onMount(() => {
		updateData();
	});
</script>

<svelte:head>
	<title>Keys</title>
</svelte:head>

<div class="card table-responsive border-bottom-0">
	<table class="table table-hover b-0 mb-0">
		<thead class="border-0">
			<tr>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center ps-2">ID</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center">Status</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center">Public Key</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white" />
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
							on:click={() => goto(paths.minion.getPath(key.id))}
						>
							{key.id}
						</th>
						<td>
							{#if key.state === 'minions'}
								<span class="badge bg-success">Accepted</span>
							{:else if key.state === 'minions_pre'}
								<span class="badge bg-danger">Unaccepted</span>
							{:else if key.state === 'minions_rejected'}
								<span class="badge bg-warning">Rejected</span>
							{:else if key.state === 'minions_denied'}
								<span class="badge bg-purple">Denied</span>
							{:else}
								<span class="badge bg-secondary">{key.state}</span>
							{/if}
						</td>
						<td>{key.finger}</td>
						<td>
							{#if key.state === 'minions'}
								{#if hasResaltPermission($currentUser, P_SALTKEY_REJECT)}
									<button
										type="button"
										class="btn btn-warning btn-sm key-btn me-1"
										on:click={() => {
											onClickReject(key);
										}}
									>
										Reject
									</button>
								{/if}
							{:else if key.state === 'minions_pre'}
								{#if hasResaltPermission($currentUser, P_SALTKEY_ACCEPT)}
									<button
										type="button"
										class="btn btn-success btn-sm key-btn me-1"
										on:click={() => {
											onClickAccept(key);
										}}
									>
										Accept
									</button>
								{/if}
							{:else if key.state === 'minions_rejected'}
								{#if hasResaltPermission($currentUser, P_SALTKEY_ACCEPT)}
									<button
										type="button"
										class="btn btn-success btn-sm key-btn me-1"
										on:click={() => {
											onClickAccept(key);
										}}
									>
										Accept
									</button>
								{/if}
							{:else if key.state === 'minions_denied'}
								{#if hasResaltPermission($currentUser, P_SALTKEY_ACCEPT)}
									<button
										type="button"
										class="btn btn-success btn-sm key-btn me-1"
										on:click={() => {
											onClickAccept(key);
										}}
									>
										Accept
									</button>
								{/if}
							{/if}
							{#if hasResaltPermission($currentUser, P_SALTKEY_DELETE)}
								<button
									type="button"
									class="btn btn-danger btn-sm key-btn"
									on:click={() => {
										onClickDelete(key);
									}}
								>
									Delete
								</button>
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
	last={$keys === null || $keys.length < paginationSize}
	updateData={fakePaginate}
/>

<style>
	:global(.key-btn) {
		width: 4.5rem;
	}
</style>
