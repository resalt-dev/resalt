<script lang="ts">
	import { writable, type Writable } from 'svelte/store';
	import ConsoleChangeBranch from '../../../components/ConsoleChangeBranch.svelte';
	import paths from '$lib/paths';
	import Tabs from '../../../components/Tabs.svelte';

	import { returns, theme } from '$lib/stores';
	import TerminalBox from '../../../components/TerminalBox.svelte';

	const collapsed: Writable<number[]> = writable([]);

	function toggleCollapsedResult(index: number) {
		collapsed.update((collapsed) => {
			if (collapsed.includes(index)) {
				return collapsed.filter((i) => i !== index);
			} else {
				return [...collapsed, index];
			}
		});
	}

	function dataToObject(data: unknown): object {
		if (typeof data === 'object' && data !== null) {
			return data as object;
		} else {
			return {};
		}
	}
</script>

<svelte:head>
	<title>Run</title>
</svelte:head>

<Tabs tabs={[paths.run]}>
	<slot />
</Tabs>

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
			{#if Object.keys(dataToObject(ret.data)).length != 0}
				<ConsoleChangeBranch data={ret.data} />
			{/if}
		</div>
	</TerminalBox>
{/each}
