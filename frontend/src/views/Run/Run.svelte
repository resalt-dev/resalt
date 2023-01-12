<script lang="ts">
	import { writable, type Writable } from 'svelte/store';
	import ConsoleChangeBranch from '../../components/ConsoleChangeBranch.svelte';
	import paths from '../../paths';
	import Tabs from '../../components/Tabs.svelte';
	import type { NavigateFn } from 'svelte-navigator';
	import type TabPage from '../../models/TabPage';

	import RunTabLive from './RunTabLive.svelte';
	import type RunResult from '../../models/RunResult';
	import { theme } from '../../stores';
	import TerminalBox from '../../components/TerminalBox.svelte';

	// svelte-ignore unused-export-let
	export let location: Location;
	// svelte-ignore unused-export-let
	export let navigate: NavigateFn;
	export let subPage: string = '';

	const returns: Writable<RunResult[]> = writable([]);
	const collapsed: Writable<number[]> = writable([]);

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
	<TerminalBox
		toggleCollapse={() => toggleCollapsedResult(ret.num)}
		collapsed={$collapsed.includes(ret.num)}
	>
		<div slot="header">
			<code class="fw-bold {$theme.dark ? '' : 'text-dark'}">
				{ret.command.toCommandLine({ forceWheel: true })}
			</code>
			<small class="float-end text-muted pt-1">
				# {ret.num + 1}
			</small>
		</div>
		<div slot="body">
			{#if Object.keys(ret.data).length != 0}
				<ConsoleChangeBranch data={ret.data} />
			{/if}
		</div>
	</TerminalBox>
{/each}
