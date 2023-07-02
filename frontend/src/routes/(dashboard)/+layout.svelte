<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { currentUser, toasts } from '$lib/stores';
	import { getCurrentUser, logout } from '$lib/api';
	import paths from '$lib/paths';
	import { MessageType } from '../../models/MessageType';
	import DashboardHeader from './DashboardHeader.svelte';
	import DashboardSidebar from './DashboardSidebar.svelte';
	import SSEConnector from '../../components/SSEConnector.svelte';
	import type User from '../../models/User';

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
				<!-- <Route path="dashboard/*subPage" component={Home} />
				<Route path="minion/:minionId/*subPage" component={Minion} />
				<Route path="minions/:subPage/*selected" component={Minions} />
				<Route path="minions" component={Minions} />
				<Route path="run/*subPage" component={Run} />
				<Route path="job/:jobId" component={Job} />
				<Route path="jobs" component={Jobs} />
				<Route path="keys" component={Keys} />
				<Route path="events" component={Events} />
				<Route path="user/:userId" component={User} />
				<Route path="users/*subPage" component={Users} />
				<Route path="preferences/*subPage" component={Preferences} />
				<Route path="settings/*subPage" component={Settings} />
				<Route path="*">
					<Redirect to={paths.dashboard.getPath()} />
				</Route> -->
			</div>
		</div>
	</div>
{/if}
