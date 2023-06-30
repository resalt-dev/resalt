<script lang="ts">
	import Clickable from './Clickable.svelte';
	import { Collapse } from '../../assets/js/bootstrap.esm-5.3.0.min';
	import { onMount } from 'svelte';
	import { v4 as uuidv4 } from 'uuid';

	let topClass = '';
	export { topClass as class };
	export let collapsed: boolean = false;
	export let toggleCollapse: () => void = undefined;

	const randomId = uuidv4();

	function logic(collapsed: boolean) {
		if (collapsed) {
			hide();
		} else {
			show();
		}
	}

	$: logic(collapsed);
	onMount(() => {
		logic(collapsed);
	});

	function show(): void {
		const element = document.getElementById(`terminalBoxCollapse${randomId}`);
		if (element !== null) {
			new Collapse(element).show();
		}
	}

	function hide(): void {
		const element = document.getElementById(`terminalBoxCollapse${randomId}`);
		if (element !== null) {
			new Collapse(element).hide();
		}
	}
</script>

<div class="card terminal-box {topClass.indexOf('mb-') == -1 ? 'mb-3' : ''} {topClass}">
	{#if toggleCollapse !== undefined}
		<Clickable type="div" event={toggleCollapse} class="card-header">
			<slot name="header" />
		</Clickable>
		<div class="collapse" style="overflow: hidden" id="terminalBoxCollapse{randomId}">
			<div class="card-body bg-dark text-light">
				<div class="card-text">
					<slot name="body" />
				</div>
			</div>
		</div>
	{:else}
		<!-- Without header, and with no Collapse around the Card body -->
		<div class="card-body bg-dark text-light">
			<div class="card-text">
				<slot name="body" />
			</div>
		</div>
	{/if}
</div>
