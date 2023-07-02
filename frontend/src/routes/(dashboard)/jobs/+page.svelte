<script lang="ts">
	import { onMount } from 'svelte';
	import { getJobs } from '$lib/api';
	import { toasts } from '$lib/stores';
	import Icon from '../../../components/Icon.svelte';
	import { writable, type Writable } from 'svelte/store';
	import TablePaginate from '../../../components/TablePaginate.svelte';
	import { MessageType } from '../../../models/MessageType';
	import type Job from '../../../models/Job';
	import { SortOrder } from '../../../models/SortOrder';

	let sortField: string | null = null;
	let sortOrder: SortOrder = SortOrder.Up;
	let paginationSize = 20;
	let paginationPage = 1;

	const jobs: Writable<Job[] | null> = writable(null);

	function updateData() {
		getJobs(
			sortField === null ? null : sortField + '.' + sortOrder,
			paginationSize,
			(paginationPage - 1) * paginationSize,
		)
			.then((data) => {
				jobs.set(data);
			})
			.catch((err) => {
				toasts.add(MessageType.ERROR, 'Failed fetching jobs', err);
			});
	}

	onMount(() => {
		updateData();
	});
</script>

<svelte:head>
	<title>Jobs</title>
</svelte:head>

<div class="card table-responsive border-bottom-0">
	<table class="table table-hover b-0 mb-0">
		<thead class="border-0">
			<tr>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center ps-2">
							JID
							<Icon
								size="0.95"
								name="help-circle"
								class="mb-0 h3 text-muted align-top"
								tooltip="Job ID"
							/>
						</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center">User</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center">Target</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center">Date</div>
					</div>
				</th>
			</tr>
		</thead>
		<tbody class="align-middle">
			{#if $jobs === null}
				<p>Loading</p>
			{:else if $jobs.length === 0 && paginationPage === 1}
				<div class="p-3">No jobs exist.</div>
			{:else}
				{#each $jobs as job}
					<tr>
						<th>{job.jid}</th>
						<td>{job.user}</td>
						<td>-</td>
						<td><small>{job.timestamp}</small></td>
					</tr>
				{/each}
			{/if}
		</tbody>
	</table>
</div>

<TablePaginate
	bind:size={paginationSize}
	bind:page={paginationPage}
	last={$jobs === null || $jobs.length < paginationSize}
	{updateData}
/>
