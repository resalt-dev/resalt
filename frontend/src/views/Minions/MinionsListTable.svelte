<script lang="ts">
	import { Link, type NavigateFn } from 'svelte-navigator';
	import { writable, type Writable } from 'svelte/store';
	import { Button, Card, Spinner, Table } from 'sveltestrap';
	import { getMinions, refreshMinion } from '../../api';
	import Clickable from '../../components/Clickable.svelte';
	import Icon from '../../components/Icon.svelte';
	import ResaltProgress from '../../components/ResaltProgress.svelte';
	import SortIcon from '../../components/SortIcon.svelte';
	import TablePaginate from '../../components/TablePaginate.svelte';
	import type Filter from '../../models/Filter';
	import { MessageType } from '../../models/MessageType';
	import type Minion from '../../models/Minion';
	import { SortOrder } from '../../models/SortOrder';
	import paths from '../../paths';
	import { hasResaltPermission, P_MINION_CONFORMITY, P_MINION_REFRESH } from '../../perms';
	import { currentUser, theme, toasts } from '../../stores';

	export let navigate: NavigateFn;
	export let filters: Filter[];

	let lastFilters: string = '[]';

	let loading: boolean = true;
	let minions: Minion[] | null = null;
	const refreshing: Writable<string[]> = writable([]);

	let sortField: string | null = null;
	let sortOrder: SortOrder = SortOrder.Down;
	let paginationSize: number = 20;
	let paginationPage: number = 1;
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

<Card class="table-responsive border-bottom-0">
	<Table hover class="b-0 mb-0">
		<thead class="bg-dark border-0 text-white">
			<tr>
				<th class="border-secondary">
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
				<th class="border-secondary">
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
				<th class="border-secondary">
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
				<th class="border-secondary">
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
				<th class="border-secondary">
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
							event={() => navigate(paths.minion.getPath(minion.id))}
							type="th"
						>
							<Link
								to={paths.minion.getPath(minion.id)}
								class="text-decoration-none text-reset"
							>
								{minion.id}
							</Link>
						</Clickable>
						<td>{minion.osType ?? 'Unknown'}</td>
						<td>{minion.lastSeen}</td>
						<td>
							{#if minion.lastUpdatedConformity === null}
								<span class="badge bg-purple"> Unknown </span>
							{:else}
								<Clickable
									type="span"
									event={() =>
										navigate(paths.minion.getPath(minion.id, 'conformity'))}
									disabled={!hasResaltPermission(
										$currentUser,
										P_MINION_CONFORMITY,
									)}
									class="badge bg-success"
								>
									{minion.conformitySuccess ?? '?'}
								</Clickable>
								/
								<Clickable
									type="span"
									event={() =>
										navigate(paths.minion.getPath(minion.id, 'conformity'))}
									disabled={!hasResaltPermission(
										$currentUser,
										P_MINION_CONFORMITY,
									)}
									class="badge bg-warning"
								>
									{minion.conformityIncorrect ?? '?'}
								</Clickable>
								/
								<Clickable
									type="span"
									event={() =>
										navigate(paths.minion.getPath(minion.id, 'conformity'))}
									disabled={!hasResaltPermission(
										$currentUser,
										P_MINION_CONFORMITY,
									)}
									class="badge bg-danger"
								>
									{minion.conformityError ?? '?'}
								</Clickable>
							{/if}
						</td>
						<td>
							<Link
								to={paths.minion.getPath(minion.id)}
								class="btn btn-{$theme.color} btn-sm px-3 me-2"
							>
								View
							</Link>
							{#if hasResaltPermission($currentUser, P_MINION_REFRESH)}
								<Button
									color="secondary"
									size="sm"
									style="width: 65px;"
									class="me-2"
									on:click={() => resync(minion.id)}
									disabled={$refreshing.indexOf(minion.id) !== -1}
								>
									{#if $refreshing.indexOf(minion.id) !== -1}
										<Spinner size="sm" />
									{:else}
										Resync
									{/if}
								</Button>
							{/if}
						</td>
					</tr>
				{/each}
			{/if}
		</tbody>
	</Table>
</Card>

<TablePaginate
	bind:size={paginationSize}
	bind:page={paginationPage}
	last={minions === null || minions.length < paginationSize}
	updateData={() => updateData(filters, true)}
/>

{#if loading}
	<ResaltProgress />
{/if}
