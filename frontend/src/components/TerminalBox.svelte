<script lang="ts">
	import { Card, Collapse } from 'sveltestrap';
	import Clickable from './Clickable.svelte';

	let topClass = '';
	export { topClass as class };
	export let collapsed: boolean = false;
	export let toggleCollapse: (id: string) => void = undefined;
</script>

<Card class="terminal-box {topClass.indexOf('mb-') == -1 ? 'mb-3' : ''} {topClass}">
	{#if toggleCollapse !== undefined}
		<Clickable type="div" event={toggleCollapse} class="card-header">
			<slot name="header" />
		</Clickable>
		<Collapse isOpen={!collapsed}>
			<div class="card-body bg-dark text-light">
				<div class="card-text">
					<slot name="body" />
				</div>
			</div>
		</Collapse>
	{:else}
		<!-- Without header, and with no Collapse around the Card body -->
		<div class="card-body bg-dark text-light">
			<div class="card-text">
				<slot name="body" />
			</div>
		</div>
	{/if}
</Card>
