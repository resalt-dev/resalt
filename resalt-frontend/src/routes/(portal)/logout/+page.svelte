<script lang="ts">
	import { goto } from '$app/navigation';
	import { logout } from '$lib/api';
	import paths from '$lib/paths';
	import { currentUser, toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';
	import { onMount } from 'svelte';

	onMount(() => {
		if ($currentUser === null) {
			goto(paths.login.getPath());
		} else {
			_logout();
		}
	});

	function _logout() {
		logout()
			.then(() => {
				console.log('Logged out');
				toasts.add(MessageType.SUCCESS, 'Logout Success', 'You have now been logged out.');
				setTimeout(() => {
					console.log('Redirecting to login page');
					goto(paths.login.getPath());
				}, 2000);
			})
			.catch((err: unknown) => {
				toasts.add(MessageType.ERROR, 'Logout Error', err);
			});
	}
</script>

<svelte:head>
	<title>Logout</title>
</svelte:head>

{#if $currentUser === null}
	<p class="fw-bold">Logged out.</p>
{:else}
	<p class="fw-bold">Logging out...</p>
{/if}
