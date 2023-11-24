<script lang="ts">
	import { page } from '$app/stores';
	import CopyButton from '$component/CopyButton.svelte';
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
		return 'User :: ' + (user ? user.username : userId);
	}
</script>

<svelte:head>
	<title>{getTitle($user)}</title>
</svelte:head>

{#if !$user}
	<h1>Loading...</h1>
{:else}
	<div class="row">
		<div class="col-12 col-xxl-4 pb-3">
			<div class="card h-100">
				<div class="card-header">General</div>
				<ul class="list-group list-group-flush">
					<li class="list-group-item">
						<strong>ID</strong>
						<span class="float-end">
							{$user.id}
							<CopyButton name="User ID" value={$user.id} />
						</span>
					</li>
					<li class="list-group-item">
						<strong>Username</strong>
						<span class="float-end">{$user.username}</span>
					</li>
					<li class="list-group-item">
						<strong>Last Login</strong>
						<span class="float-end">
							{#if $user.lastLogin}
								{$user.lastLogin}
							{:else}
								<em>Never</em>
							{/if}
						</span>
					</li>
					<li class="list-group-item">
						<strong>Email</strong>
						<span class="float-end">
							{#if $user.email}
								{$user.email}
							{:else}
								<em>Not set</em>
							{/if}
						</span>
					</li>
				</ul>
			</div>
		</div>
	</div>
{/if}
