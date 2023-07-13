<script lang="ts">
	import { goto } from '$app/navigation';
	import SSEConnector from '$component/SSEConnector.svelte';
	import { getCurrentUser } from '$lib/api';
	import paths from '$lib/paths';
	import { auth, currentUser, toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';
	import type User from '$model/User';
	import { onMount } from 'svelte';
	import DashboardHeader from './DashboardHeader.svelte';
	import DashboardSidebar from './DashboardSidebar.svelte';

	onMount(() => {
		if ($auth == null) {
			goto(paths.login.getPath());
			return;
		}
		getCurrentUser()
			.then((data: User) => {
				currentUser.set(data);
			})
			.catch((err: unknown) => {
				console.error(err);
				toasts.add(
					MessageType.WARNING,
					'Logged out',
					'You have been logged out due to the token being invalid.',
				);
				goto(paths.logout.getPath());
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
