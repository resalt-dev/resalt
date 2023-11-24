<script lang="ts">
	import { page } from '$app/stores';
	import JsonViewer from '$component/JsonViewer.svelte';
	import { getUserById } from '$lib/api';
	import { replacementParams, toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';
	import type User from '$model/User';
	import { onMount } from 'svelte';
	import { writable, type Writable } from 'svelte/store';

	const user: Writable<User | null> = writable(null);

	$: userId = $page.params.userId;

	function updateData(): void {
		getUserById(userId)
			.then((data) => {
				user.set(data);
				replacementParams.set({ ...$replacementParams, userId: data.username });
			})
			.catch((err) => {
				toasts.add(MessageType.ERROR, 'Failed fetching user: ' + userId, err);
			});
	}

	onMount(() => {
		updateData();
	});

	function getTitle(user: User | null): string {
		return 'Permission :: ' + (user ? user.username : userId);
	}
</script>

<svelte:head>
	<title>{getTitle($user)}</title>
</svelte:head>

{#if !$user}
	<h1>Loading...</h1>
{:else}
	<JsonViewer data={$user.perms} sort={false} />
{/if}
