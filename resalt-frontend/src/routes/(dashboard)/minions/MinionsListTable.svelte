<script lang="ts">
	import { goto } from '$app/navigation';
	import Clickable from '$component/Clickable.svelte';
	import Icon from '$component/Icon.svelte';
	import ResaltProgress from '$component/ResaltProgress.svelte';
	import SortIcon from '$component/SortIcon.svelte';
	import TablePaginate from '$component/TablePaginate.svelte';
	import { getMinions, refreshMinion } from '$lib/api';
	import paths from '$lib/paths';
	import { hasResaltPermission, P_MINION_CONFORMITY, P_MINION_REFRESH } from '$lib/perms';
	import { currentUser, theme, toasts } from '$lib/stores';
	import type Filter from '$model/Filter';
	import { MessageType } from '$model/MessageType';
	import type Minion from '$model/Minion';
	import { SortOrder } from '$model/SortOrder';
	import { writable, type Writable } from 'svelte/store';

	export let filters: Filter[];

	let lastFilters = '[]';

	let loading = true;
	let minions: Minion[] | null = null;
	const refreshing: Writable<string[]> = writable([]);

	let sortField: string | null = null;
	let sortOrder: SortOrder = SortOrder.Down;
	let paginationSize = 20;
	let paginationPage = 1;
	$: active = sortField + ':' + sortOrder;
	$: updateData(filters, false);

	function updateData(filters: Filter[], force: boolean): void {
		const parsedFilters = JSON.stringify(filters);
		if (lastFilters === parsedFilters && !force) {
			return;
		}
		lastFilters = parsedFilters;

		loading = true;
		getMinions(
			filters,
			sortField === null ? null : sortField + '.' + sortOrder,
			paginationSize,
			(paginationPage - 1) * paginationSize,
		)
			.then((data) => {
				minions = data;
				loading = false;
			})
			.catch((err) => {
				toasts.add(MessageType.ERROR, 'Failed fetching minions', err);
			});
	}

	function toggleSort(field: string, order: SortOrder): void {
		if (sortField === null) {
			sortField = field;
			sortOrder = order;
		} else if (sortField === field) {
			if (order !== sortOrder) {
				sortOrder = order;
			} else {
				sortField = null;
				sortOrder = SortOrder.Down;
			}
		} else {
			sortField = field;
			sortOrder = order;
		}

		console.log('toggleSort', field, order, sortField, sortOrder);

		updateData(filters, true);
	}

	function resync(minionId: string) {
		refreshing.update((ids) => [...ids, minionId]);
		refreshMinion(minionId)
			.then(() => {
				refreshing.update((ids) => ids.filter((id) => id !== minionId));
				updateData(filters, true);
			})
			.catch((err) => {
				toasts.add(MessageType.ERROR, 'Failed resyncing minion', err);
			});
	}
</script>

<div class="card table-responsive border-bottom-0">
	<table class=" table table-hover b-0 mb-0">
		<thead class="border-0">
			<tr>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center ps-2">ID</div>
						<div class="col-auto">
							<SortIcon
								{active}
								field="id"
								order={SortOrder.Down}
								click={toggleSort}
							/>
							<SortIcon {active} field="id" order={SortOrder.Up} click={toggleSort} />
						</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center">OS</div>
						<div class="col-auto">
							<SortIcon
								{active}
								field="osType"
								order={SortOrder.Down}
								click={toggleSort}
							/>
							<SortIcon
								{active}
								field="osType"
								order={SortOrder.Up}
								click={toggleSort}
							/>
						</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center">Last seen</div>
						<div class="col-auto">
							<SortIcon
								{active}
								field="lastSeen"
								order={SortOrder.Down}
								click={toggleSort}
							/>
							<SortIcon
								{active}
								field="lastSeen"
								order={SortOrder.Up}
								click={toggleSort}
							/>
						</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center">Conformity</div>
						<div class="col-auto">
							<SortIcon
								{active}
								field="conformitySuccess"
								order={SortOrder.Down}
								click={toggleSort}
								color="success"
							/>
							<SortIcon
								{active}
								field="conformitySuccess"
								order={SortOrder.Up}
								click={toggleSort}
								color="success"
							/>
						</div>
						<div class="col-auto">
							<SortIcon
								{active}
								field="conformityIncorrect"
								order={SortOrder.Down}
								click={toggleSort}
								color="warning"
							/>
							<SortIcon
								{active}
								field="conformityIncorrect"
								order={SortOrder.Up}
								click={toggleSort}
								color="warning"
							/>
						</div>
						<div class="col-auto">
							<SortIcon
								{active}
								field="conformityError"
								order={SortOrder.Down}
								click={toggleSort}
								color="danger"
							/>
							<SortIcon
								{active}
								field="conformityError"
								order={SortOrder.Up}
								click={toggleSort}
								color="danger"
							/>
						</div>
					</div>
				</th>
				<th class="border-secondary bg-dark text-white">
					<div class="row g-1">
						<div class="col-auto align-self-center">Actions</div>
						<div class="col align-self-bottom canRotate">
							<Icon
								size="1.5"
								name="refresh"
								class="float-end hover-icon mouse-pointer"
								on:click={() => {
									updateData(filters, true);
								}}
							/>
						</div>
					</div>
				</th>
			</tr>
		</thead>
		<tbody class="align-middle">
			{#if minions === null}
				<p>Loading</p>
			{:else if minions.length === 0 && paginationPage === 1}
				<div class="p-3">No minions returned.</div>
			{:else}
				{#each minions as minion}
					<tr>
						<Clickable
							event={() => goto(paths.minion.getPath({ minionId: minion.id }))}
							type="th"
						>
							<a
								href={paths.minion.getPath({ minionId: minion.id })}
								class="text-decoration-none text-reset"
							>
								{minion.id}
							</a>
						</Clickable>
						<td>{minion.osType ?? 'Unknown'}</td>
						<td>{minion.lastSeen}</td>
						<td>
							{#if minion.lastUpdatedConformity === null}
								<span class="badge bg-purple"> Unknown </span>
							{:else}
								<svelte:element
									this={hasResaltPermission($currentUser, P_MINION_CONFORMITY)
										? 'a'
										: 'span'}
									href={paths.minion_conformity.getPath({
										minionId: minion.id,
									})}
									class="badge bg-success text-decoration-none"
								>
									{minion.conformitySuccess ?? '?'}
								</svelte:element>
								/
								<svelte:element
									this={hasResaltPermission($currentUser, P_MINION_CONFORMITY)
										? 'a'
										: 'span'}
									href={paths.minion_conformity.getPath({
										minionId: minion.id,
									})}
									class="badge bg-warning text-decoration-none"
								>
									{minion.conformityIncorrect ?? '?'}
								</svelte:element>
								/
								<svelte:element
									this={hasResaltPermission($currentUser, P_MINION_CONFORMITY)
										? 'a'
										: 'span'}
									href={paths.minion_conformity.getPath({
										minionId: minion.id,
									})}
									class="badge bg-danger text-decoration-none"
								>
									{minion.conformityError ?? '?'}
								</svelte:element>
							{/if}
						</td>
						<td>
							<a
								href={paths.minion.getPath({ minionId: minion.id })}
								class="btn btn-{$theme.color} btn-sm px-3 me-2"
							>
								View
							</a>
							{#if hasResaltPermission($currentUser, P_MINION_REFRESH)}
								<button
									type="button"
									class="btn btn-secondary btn-sm me-2"
									style="width: 65px;"
									on:click={() => resync(minion.id)}
									disabled={$refreshing.indexOf(minion.id) !== -1}
								>
									{#if $refreshing.indexOf(minion.id) !== -1}
										<div role="status" class="spinner-border-sm spinner-border">
											<span class="visually-hidden">Loading...</span>
										</div>
									{:else}
										Resync
									{/if}
								</button>
							{/if}
						</td>
					</tr>
				{/each}
			{/if}
		</tbody>
	</table>
</div>

<TablePaginate
	bind:size={paginationSize}
	bind:page={paginationPage}
	last={minions === null || minions.length < paginationSize}
	updateData={() => updateData(filters, true)}
/>

{#if loading}
	<ResaltProgress />
{/if}
