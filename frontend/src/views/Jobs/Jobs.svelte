<script lang="ts">
	import { onMount } from 'svelte';
	import { getJobs } from '../../api';
	import { toasts } from '../../stores';
	import { Tooltip } from 'sveltestrap';
	import Icon from '../../components/Icon.svelte';
	import { writable, type Writable } from 'svelte/store';
	import TablePaginate from '../../components/TablePaginate.svelte';
	import { MessageType } from '../../models/MessageType';
	import type Job from '../../models/Job';
	import { SortOrder } from '../../models/SortOrder';
	import type { NavigateFn } from 'svelte-navigator';

	// svelte-ignore unused-export-let
	export let location: Location;
	// svelte-ignore unused-export-let
	export let navigate: NavigateFn;

	let sortField: string | null = null;
	let sortOrder: SortOrder = SortOrder.Up;
	let paginationSize: number = 20;
	let paginationPage: number = 1;

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

	let jobIdTooltipElement: HTMLElement;
</script>

<div class="card table-responsive border-bottom-0">
	<table class="table table-hover b-0 mb-0">
		<thead class="border-0">
			<tr>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center ps-2">
							JID<Icon
								size="0.95"
								name="help-circle"
								class="mb-0 h3 text-muted align-top"
								bind:htmlElement={jobIdTooltipElement}
							/>
							<Tooltip target={jobIdTooltipElement}>Job ID</Tooltip>
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
