<script lang="ts">
	import { onMount } from 'svelte';
	import { writable, type Writable } from 'svelte/store';
	import { CardBody, CardHeader, CardTitle, Col, Table } from 'sveltestrap';
	import { getSystemStatus } from '../../api';
	import Icon from '../../components/Icon.svelte';
	import ResaltProgress from '../../components/ResaltProgress.svelte';
	import { MessageType } from '../../models/MessageType';
	import type SystemStatus from '../../models/SystemStatus';
	import { config, theme, toasts } from '../../stores';

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
	<Col xs="12" class="pb-3">
		<div class="card {$theme.dark ? 'bg-dark' : ''}">
			<CardBody>
				<!-- welcome title -->
				<h1 class="display-4">Welcome to Resalt</h1>
				<!-- subtitle -->
				<p class="lead mb-0">
					This is the control panel for your SaltStack infrastructure.
				</p>
				<!-- <br />
				<br />
				<div class="row">
					<Col xs="12" lg="4">
						<h5 class="card-title">Get Started</h5>
					</Col>
					<Col xs="12" lg="4">
						<h5 class="card-title">Next Steps</h5>
						Hello!
					</Col>
					<Col xs="12" lg="4">
						<h5 class="card-title">More Actions</h5>
						Hello!
					</Col>
				</div> -->
			</CardBody>
		</div>
	</Col>
	<Col xs="12" xl="4" class="pb-3">
		<div class="card {$theme.dark ? 'bg-dark' : ''}">
			<CardHeader>
				<CardTitle class="mb-0">System Summary</CardTitle>
			</CardHeader>
			{#if $status === null}
				<ResaltProgress />
			{:else}
				<CardBody>
					<h5 class="card-title">Salt Event Listener</h5>
					<p class="card-text">
						{#if $status.salt}
							<Icon name="check-circle" class="text-success m-2" /> Connected
						{:else}
							<Icon name="x-circle" class="text-danger m-2" /> Disconnected
						{/if}
					</p>
					<h5 class="card-title">Database</h5>
					<p class="card-text">
						{#if $status.db}
							<Icon name="check-circle" class="text-success m-2" /> Connected
						{:else}
							<Icon name="x-circle" class="text-danger m-2" /> Disconnected
						{/if}
					</p>
					<p class="card-text">
						<Table hover class="b-0 mb-0">
							<thead class="bg-dark border-0 text-white">
								<tr>
									<th>Table</th>
									<th>Count</th>
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
						</Table>
					</p>
				</CardBody>
			{/if}
		</div>
	</Col>
	<Col xs="12" xl="4" class="pb-3">
		<div class="card {$theme.dark ? 'bg-dark' : ''}">
			<CardHeader>
				<CardTitle class="mb-0">Quick Links</CardTitle>
			</CardHeader>
			<CardBody class="text-center">
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
			</CardBody>
		</div>
	</Col>
	<Col xs="12" xl="4" class="pb-3">
		<div class="card {$theme.dark ? 'bg-dark' : ''}">
			<CardHeader>
				<CardTitle class="mb-0">Latest News</CardTitle>
			</CardHeader>
			<CardBody style="max-height: 700px; overflow-y: auto;">
				{#each $config.latestNews as news, i}
					{#if i !== 0}
						<hr class="text-light" />
					{/if}
					<h5 class="card-title">{news.split('§')[0]}</h5>
					{#if news.split('§')[1]}
						<p class="card-text">{news.split('§')[1]}</p>
					{/if}
				{/each}
			</CardBody>
		</div>
	</Col>
</div>
