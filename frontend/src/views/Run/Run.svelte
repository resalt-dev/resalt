<script lang="ts">
	import { Card, Collapse } from 'sveltestrap';
	import { writable, type Writable } from 'svelte/store';
	import ConsoleChangeBranch from '../../components/ConsoleChangeBranch.svelte';
	import paths from '../../paths';
	import Tabs from '../../components/Tabs.svelte';
	import type { NavigateFn } from 'svelte-navigator';
	import type TabPage from '../../models/TabPage';

	import RunTabLive from './RunTabLive.svelte';
	import type RunResult from '../../models/RunResult';
	import Clickable from '../../components/Clickable.svelte';

	// svelte-ignore unused-export-let
	export let location: Location;
	// svelte-ignore unused-export-let
	export let navigate: NavigateFn;
	export let subPage: string = '';

	let returns: Writable<RunResult[]> = writable([]);
	let collapsed: Writable<number[]> = writable([]);

	let tabs: TabPage[] = [];
	$: tabs = [
		{
			key: 'live',
			label: 'Live Run',
			path: paths.run.getPath(),
			component: RunTabLive,
			data: { returns },
		},
	];

	function toggleCollapsedResult(index: number) {
		collapsed.update((collapsed) => {
			if (collapsed.includes(index)) {
				return collapsed.filter((i) => i !== index);
			} else {
				return [...collapsed, index];
			}
		});
	}
</script>

<Tabs {tabs} current={subPage} />

{#each $returns as ret}
	<Card class="result-box mb-3">
		<Clickable event={() => toggleCollapsedResult(ret.num)} class="card-header">
			<span>Result : </span>
			({ret.command.targetType}) {ret.command.target}
			<small class="text-muted">
				({ret.command.toCommandLine({ forceWheel: true })})
			</small>
			<small class="float-end text-muted pt-1">
				# {ret.num + 1}
			</small>
		</Clickable>
		<Collapse isOpen={!$collapsed.includes(ret.num)}>
			<div class="card-body bg-dark text-light">
				<div class="card-text">
					{#if Object.keys(ret.data).length != 0}
						<ConsoleChangeBranch data={ret.data} />
					{/if}
				</div>
			</div>
		</Collapse>
	</Card>
{/each}
