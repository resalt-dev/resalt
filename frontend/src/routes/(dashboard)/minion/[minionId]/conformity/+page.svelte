<script lang="ts">
	import { page } from '$app/stores';
	import FloatingRightButton from '$component/FloatingRightButton.svelte';
	import JsonViewer from '$component/JsonViewer.svelte';
	import { getMinionById } from '$lib/api';
	import { theme, toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';
	import type Minion from '$model/Minion';
	import { onMount } from 'svelte';
	import { writable, type Writable } from 'svelte/store';
	import ConformityResultBox from '../../ConformityResultBox.svelte';
	import ConformityTreeView from '../../ConformityTreeView.svelte';
	import type {
		Conform,
		ConformData,
		ConformDataOptional,
		ConformTreeNode,
	} from '../../ConformityTypes';

	$: minionId = $page.params.minionId;
	const minion: Writable<Minion | null> = writable(null);

	onMount(() => {
		getMinionById(minionId)
			.then((data) => {
				minion.set(data);
			})
			.catch((err) => {
				toasts.add(MessageType.ERROR, 'Failed fetching minion: ' + minionId, err);
			});
	});

	let rawData = false;

	enum ConformSortOption {
		Incremental = 'Incremental order',
		Decremental = 'Decremental order',
		LongestRuntime = 'Longest runtime',
		BestResult = 'Success first',
		WorstResult = 'Errors first',
	}

	let sortOrder: ConformSortOption = ConformSortOption.Incremental;
	let showSuccess = true;
	let showIncorrect = true;
	let showError = true;
	let showCollapsed = true;
	let filterNamespace = '';
	let collapseList: string[] = [];

	$: conformity = Object.entries(JSON.parse($minion?.conformity ?? '[]') ?? [])
		.map(([key, v]) => {
			let value = v as ConformDataOptional;

			let parts = key.split('_|-');
			let conform: Conform = {
				title: key,
				fun: parts[0] + '.' + parts[parts.length - 1],
				// color should be success/warning/danger based on true/null/false
				color:
					value.result === true
						? 'success'
						: value.result === false
						? 'danger'
						: 'warning',
				data: {
					__id__: value.__id__ ?? parts[1] ?? 'UKNOWN ID',
					// eslint-disable-next-line camelcase
					__run_num__: value.__run_num__,
					__sls__: value.__sls__,
					changes: value.changes ?? {},
					comment: value.comment,
					duration: value.duration,
					name: value.name ?? parts[2] ?? 'UKNOWN NAME',
					result: value.result,
					// eslint-disable-next-line camelcase
					start_time: value.start_time,
				} as ConformData,
			};
			return conform;
		})
		.sort((a, b) => {
			switch (sortOrder) {
				case ConformSortOption.Incremental:
					return a.data.__run_num__ - b.data.__run_num__;
				case ConformSortOption.Decremental:
					return b.data.__run_num__ - a.data.__run_num__;
				case ConformSortOption.LongestRuntime:
					return b.data.duration - a.data.duration;
				case ConformSortOption.BestResult:
					return (
						(a.data.result === true ? 1 : a.data.result === false ? 3 : 2) -
						(b.data.result === true ? 1 : b.data.result === false ? 3 : 2)
					);
				case ConformSortOption.WorstResult:
					return (
						(b.data.result === true ? 1 : b.data.result === false ? 3 : 2) -
						(a.data.result === true ? 1 : a.data.result === false ? 3 : 2)
					);
				default:
					return 0;
			}
		});

	// Reduce above Conformity states to a tree of SLS files
	// - a (1)
	//   - aa (1)
	//   - ab (1)
	// - common
	//   - init
	//     - test (2)
	// - editor (1)
	//   - vim (7)
	function sortSubtreeRecursively(subtree: ConformTreeNode[]) {
		subtree.sort((a, b) => a.name.localeCompare(b.name));
		subtree.forEach((node) => {
			sortSubtreeRecursively(node.subtree);
		});
	}

	$: conformityTree = conformity
		// .filter((c) => {
		//     if (!showSuccess && c.data.result === true) return false;
		//     if (!showIncorrect && c.data.result === null) return false;
		//     if (!showError && c.data.result === false) return false;
		//     return true;
		// })
		.reduce(
			(acc, c) => {
				let parts = c.data.__sls__.split('.');
				let current = acc;
				for (let i = 0; i < parts.length; i++) {
					let part = parts[i];
					let existing = current.subtree.find((e) => e.name === part);
					if (!existing) {
						existing = {
							name: part,
							color: '',
							parent: current,
							subtree: [],
							items: [],
						};
						current.subtree.push(existing);
					}
					current = existing;
				}
				current.items.push(c);
				// Set min color for chain going up
				let parent: ConformTreeNode | null = current;
				while (parent !== null) {
					if (c.color === 'danger') {
						parent.color = 'danger';
					} else if (c.color === 'warning' && parent.color !== 'danger') {
						parent.color = 'warning';
					} else if (c.color === 'success' && parent.color === '') {
						parent.color = 'success';
					}
					parent = parent.parent;
				}
				return acc;
			},
			{
				name: '#',
				color: '',
				parent: null,
				subtree: [],
				items: [],
			} as ConformTreeNode,
		);
	// Recursively sort tree alphabetically
	$: sortSubtreeRecursively(conformityTree.subtree);

	$: console.log(conformityTree);
</script>

{#if !$minion?.conformity}
	<div class="p-3">No conformity data. Please refresh minion.</div>
{:else}
	<FloatingRightButton
		onclick={() => (rawData = !rawData)}
		label={rawData ? 'View List' : 'View JSON'}
	/>

	{#if rawData}
		<JsonViewer data={JSON.parse($minion?.conformity)} />
	{:else}
		<div class="row">
			<div class="col-3">
				<div class="card mb-3">
					<div class="card-header">Options</div>
					<div class="card-body">
						<h5 class="card-title">Sort method</h5>

						<!-- loop through sort orders-->
						{#each Object.entries(ConformSortOption) as [sortKey, sortValue]}
							<div class="form-check">
								<input
									class="form-check-input form-check-input-{$theme.color}"
									type="radio"
									name="sortMethod"
									id={`sortMethod-${sortKey}`}
									on:click={() => (sortOrder = sortValue)}
									checked={sortOrder === sortValue}
								/>
								<label class="form-check-label" for={`sortMethod-${sortKey}`}>
									{sortValue}
								</label>
							</div>
						{/each}

						<h5 class="card-title mt-3">Visibility</h5>

						<!-- showSuccess -->
						<div class="form-check">
							<input
								class="form-check-input form-check-input-success"
								type="checkbox"
								id="showSuccess"
								on:click={() => (showSuccess = !showSuccess)}
								checked={showSuccess}
							/>
							<label class="form-check-label" for="showSuccess">
								Show Succeeded ({conformity.filter((c) => c.data.result === true)
									.length})
							</label>
						</div>
						<!-- showIncorrect -->
						<div class="form-check">
							<input
								class="form-check-input form-check-input-warning"
								type="checkbox"
								id="showIncorrect"
								on:click={() => (showIncorrect = !showIncorrect)}
								checked={showIncorrect}
							/>
							<label class="form-check-label" for="showIncorrect">
								Show Incorrects ({conformity.filter((c) => c.data.result === null)
									.length})
							</label>
						</div>
						<!-- showError -->
						<div class="form-check">
							<input
								class="form-check-input form-check-input-danger"
								type="checkbox"
								id="showError"
								on:click={() => (showError = !showError)}
								checked={showError}
							/>
							<label class="form-check-label" for="showError">
								Show Errors ({conformity.filter((c) => c.data.result === false)
									.length})
							</label>
						</div>
						<br />
						<!-- showCollapsed -->
						<div class="form-check">
							<input
								class="form-check-input form-check-input-{$theme.color}"
								type="checkbox"
								id="showCollapsed"
								on:click={() => (showCollapsed = !showCollapsed)}
								checked={showCollapsed}
							/>
							<label class="form-check-label" for="showCollapsed">
								Show Collapsed
							</label>
						</div>
					</div>
				</div>

				<div class="card mb-3">
					<div class="card-header">States</div>
					<div class="card-body">
						<!-- Render Tree structure in a recursive fashion. -->
						<ConformityTreeView
							node={conformityTree}
							bind:filterNamespace
							bind:collapseList
						/>
					</div>
				</div>
			</div>
			<div class="col">
				<div class="d-grid">
					{#each conformity.filter( (c) => c.data.__sls__.startsWith(filterNamespace), ) as conform}
						<div
							class=" {!(
								(showSuccess && conform.data.result === true) ||
								(showIncorrect && conform.data.result === null) ||
								(showError && conform.data.result === false)
							) && !showCollapsed
								? 'd-none'
								: ''}"
						>
							<ConformityResultBox
								color={conform.color}
								num={conform.data.__run_num__}
								sls={conform.data.__sls__}
								stateName={conform.data.__id__}
								fun={conform.fun}
								name={conform.data.name}
								result={conform.data.result}
								comment={conform.data.comment}
								startTime={conform.data.start_time}
								duration={conform.data.duration}
								changes={conform.data.changes}
								collapsed={!(
									(showSuccess && conform.data.result === true) ||
									(showIncorrect && conform.data.result === null) ||
									(showError && conform.data.result === false)
								)}
								{showCollapsed}
							/>
						</div>
					{/each}
				</div>
			</div>
		</div>
	{/if}
{/if}
