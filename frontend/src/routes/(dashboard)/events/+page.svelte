<script lang="ts">
	import { onMount } from 'svelte';
	import Icon from '../../../components/Icon.svelte';
	import { getEvents } from '$lib/api';
	import { toasts } from '$lib/stores';
	import TablePaginate from '../../../components/TablePaginate.svelte';
	import { MessageType } from '../../../models/MessageType';
	import { writable, type Writable } from 'svelte/store';
	import EventsWriteableData from '../../../models/EventsWriteableData';
	import Clickable from '../../../components/Clickable.svelte';

	let paginationSize = 20;
	let paginationPage = 1;

	const events: Writable<EventsWriteableData[] | null> = writable(null);
	const expandedEvents: Writable<string[]> = writable([]);

	function toggleExpandEvent(index: string) {
		console.log(index);
		if ($expandedEvents.includes(index)) {
			expandedEvents.update((array) => array.filter((i) => i !== index));
		} else {
			expandedEvents.update((array) => [...array, index]);
		}
		console.log($expandedEvents); // TODO: remove
	}

	function updateData() {
		getEvents(paginationSize, (paginationPage - 1) * paginationSize)
			.then((data) => {
				events.set(
					data.map((event) => {
						const data: { [fun: string]: unknown } = JSON.parse(
							event.data ?? '{data: {}}',
						).data;
						return new EventsWriteableData(
							event.id,
							event.timestamp,
							event.tag,
							event.data,
							(data.jid as string) ?? '',
							(data.id as string) ?? '',
							(data.fun as string) ?? '',
							data,
							JSON.stringify(data, null, 2),
							((event.tag ?? '') + '_' + (event.timestamp ?? '')).replace(/ /g, '_'),
						);
					}),
				);
			})
			.catch((err) => {
				toasts.add(MessageType.ERROR, 'Failed fetching events', err);
			});
	}

	onMount(() => {
		updateData();
	});
</script>

<svelte:head>
	<title>Events</title>
</svelte:head>

<div class="card table-responsive border-bottom-0">
	<table class="table table-hover b-0 mb-0">
		<thead class="border-0">
			<tr>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center ps-2">Tag</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center">Function</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center">Target</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center">Job ID</div>
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
			{#if $events === null}
				<p>Loading</p>
			{:else if $events.length === 0 && paginationPage === 1}
				<div class="p-3">No events exist. Very unusal.</div>
			{:else}
				{#each $events as event}
					<tr>
						<!-- <th>{event.id}</th> -->
						<Clickable type="td" event={() => toggleExpandEvent(event.uniqueIndex)}>
							<Icon
								size="1.25"
								name={$expandedEvents.includes(event.uniqueIndex)
									? 'chevron-up'
									: 'chevron-down'}
							/>
							{event.tag}
						</Clickable>
						<td>{event.fun}</td>
						<td>{event.target}</td>
						<td>{event.jid}</td>
						<td><small>{event.timestamp}</small></td>
					</tr>
					{#if $expandedEvents.includes(event.uniqueIndex)}
						<tr>
							<td class="bg-light" colspan="5">
								<pre class="text-left">{event.dataFormatted}</pre>
							</td>
						</tr>
					{/if}
				{/each}
			{/if}
		</tbody>
	</table>
</div>

<TablePaginate
	bind:size={paginationSize}
	bind:page={paginationPage}
	last={$events === null || $events.length < paginationSize}
	{updateData}
/>
