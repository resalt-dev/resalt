<script lang="ts">
	import { goto } from '$app/navigation';
	import SSEConnector from '$component/SSEConnector.svelte';
	import { getCurrentUser, logout } from '$lib/api';
	import paths from '$lib/paths';
	import { currentUser, toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';
	import type User from '$model/User';
	import { onMount } from 'svelte';
	import DashboardHeader from './DashboardHeader.svelte';
	import DashboardSidebar from './DashboardSidebar.svelte';

	onMount(() => {
		getCurrentUser()
			.then((data: User) => {
				currentUser.set(data);
			})
			.catch((err: unknown) => {
				console.error(err);
				logout()
					.then(() => {
						toasts.add(
							MessageType.WARNING,
							'Logged out',
							'You have been logged out due to the token being invalid.',
						);
						goto(paths.login.getPath());
					})
					.catch((err: unknown) => {
						toasts.add(MessageType.ERROR, 'Logout Error', err);
					});
			});
	});
</script>

{#if $currentUser === null}
	<p>Loading...</p>
{:else}
	<SSEConnector />
	<div class="d-flex flex-row h-100">
		<div class="">
			<DashboardSidebar />
		</div>
		<div class="w-100 overflow-auto bg-white">
			<DashboardHeader />
			<div class="px-4 py-3">
				<slot />
			</div>
		</div>
	</div>
{/if}
