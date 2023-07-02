<script lang="ts">
	import { onMount } from 'svelte';
	import { writable, type Writable } from 'svelte/store';
	import { getSystemStatus } from '$lib/api';
	import Icon from '../../../components/Icon.svelte';
	import ResaltProgress from '../../../components/ResaltProgress.svelte';
	import { MessageType } from '../../../models/MessageType';
	import type SystemStatus from '../../../models/SystemStatus';
	import { config, theme, toasts } from '$lib/stores';

	const status: Writable<SystemStatus | null> = writable(null);

	function updateData() {
		getSystemStatus()
			.then((data) => {
				status.set(data);
			})
			.catch((err) => {
				toasts.add(MessageType.ERROR, 'Failed fetching system status', err);
			});
	}

	onMount(() => {
		updateData();
	});
</script>

<div class="row">
	<div class="col-12 pb-3">
		<div class="card {$theme.dark ? 'bg-dark' : ''}">
			<div class="card-body">
				<!-- welcome title -->
				<h1 class="display-4">Welcome to Resalt</h1>
				<!-- subtitle -->
				<p class="lead mb-0">
					This is the control panel for your SaltStack infrastructure.
				</p>
				<!-- <br />
				<br />
				<div class="row">
					<div class="col-12 col-xl-4">
						<h5 class="card-title">Get Started</h5>
					</div>
					<div class="col-12 col-xl-4">
						<h5 class="card-title">Next Steps</h5>
						Hello!
					</div>
					<div class="col-12 col-xl-4">
						<h5 class="card-title">More Actions</h5>
						Hello!
					</div>
				</div> -->
			</div>
		</div>
	</div>
	<div class="col-12 col-xl-4 pb-3">
		<div class="card {$theme.dark ? 'bg-dark' : ''}">
			<div class="card-header">
				<h5 class="card-title mb-0">System Summary</h5>
			</div>
			{#if $status === null}
				<ResaltProgress />
			{:else}
				<div class="card-body">
					<h5 class="card-title">Salt Event Listener</h5>
					<div class="card-text mb-3">
						{#if $status.salt}
							<Icon name="check-circle" class="text-success m-2" /> Connected
						{:else}
							<Icon name="x-circle" class="text-danger m-2" /> Disconnected
						{/if}
					</div>
					<h5 class="card-title">Database</h5>
					<div class="card-text mb-3">
						{#if $status.db}
							<Icon name="check-circle" class="text-success m-2" /> Connected
						{:else}
							<Icon name="x-circle" class="text-danger m-2" /> Disconnected
						{/if}
					</div>
					<div class="card-text">
						<table class="table table-hover b-0 mb-0">
							<thead class="border-0">
								<tr>
									<th class="border-secondary bg-dark text-white">Table</th>
									<th class="border-secondary bg-dark text-white">Count</th>
								</tr>
							</thead>
							<tbody>
								<tr>
									<td>auth_token_total</td>
									<td>{$status.dbAuthTokensTotal}</td>
								</tr>
								<tr>
									<td>auth_token_active</td>
									<td>{$status.dbAuthTokensActive}</td>
								</tr>
								<tr>
									<td>events_total</td>
									<td>{$status.dbEventsTotal}</td>
								</tr>
								<tr>
									<td>job_returns_total</td>
									<td>{$status.dbJobReturnsTotal}</td>
								</tr>
								<tr>
									<td>jobs_total</td>
									<td>{$status.dbJobsTotal}</td>
								</tr>
								<tr>
									<td>minions_total</td>
									<td>{$status.dbMinionsTotal}</td>
								</tr>
								<tr>
									<td>permission_group_users_total</td>
									<td>{$status.dbPermissionGroupUsersTotal}</td>
								</tr>
								<tr>
									<td>permission_groups_total</td>
									<td>{$status.dbPermissionGroupsTotal}</td>
								</tr>
								<tr>
									<td>users_total</td>
									<td>{$status.dbUsersTotal}</td>
								</tr>
							</tbody>
						</table>
					</div>
				</div>
			{/if}
		</div>
	</div>
	<div class="col-12 col-xl-4 pb-3">
		<div class="card {$theme.dark ? 'bg-dark' : ''}">
			<div class="card-header">
				<h5 class="card-title mb-0">Quick Links</h5>
			</div>
			<div class="card-body text-center">
				<a
					href="https://resalt.dev/"
					class="text-reset text-decoration-none"
					target="_blank"
					rel="noopener noreferrer"
				>
					<Icon name="buildings" size="7" class="my-5" />
					<h5 class="card-title">Go to Documentation</h5>
					<p class="card-text">Find documentation, guides, and more</p>
				</a>

				<hr class="text-light mt-5 mb-0" />

				<a
					href="https://resalt.dev/contribute#Feedback"
					class="text-reset text-decoration-none"
					target="_blank"
					rel="noopener noreferrer"
				>
					<Icon name="comment-dots" size="7" class="mt-5 mb-4" />
					<h5 class="card-title">Give Feedback</h5>
					<p class="card-text">Give feedback or report a bug</p>
					<br />
				</a>
			</div>
		</div>
	</div>
	<div class="col-12 col-xl-4 pb-3">
		<div class="card {$theme.dark ? 'bg-dark' : ''}">
			<div class="card-header">
				<h5 class="card-title mb-0">Latest News</h5>
			</div>
			<div class="card-body" style="max-height: 700px; overflow-y: auto;">
				{#each $config?.latestNews ?? [] as news, i}
					{#if i !== 0}
						<hr class="text-light" />
					{/if}
					<h5 class="card-title">{news.split('§')[0]}</h5>
					{#if news.split('§')[1]}
						<p class="card-text">{news.split('§')[1]}</p>
					{/if}
				{/each}
			</div>
		</div>
	</div>
</div>
