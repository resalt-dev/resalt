<script lang="ts">
	import { Tooltip } from 'bootstrap';
	import { onMount } from 'svelte';

	export let type = 'regular';
	export let name = 'rocket';
	export let size = '2';
	export let align: 'top' | 'middle' | 'bottom' = 'middle';
	let cssClass = '';
	export { cssClass as class };
	export let htmlElement: HTMLElement | null = null;
	export let style = '';
	export let tooltip = '';

	$: iconName = (type === 'solid' ? 'bxs-' : type === 'logo' ? 'bxl-' : 'bx-') + name;
	$: tooltipAttr =
		tooltip && tooltip.length > 0
			? {
					'data-bs-toggle': 'tooltip',
					'data-bs-placement': 'top',
					'data-bs-title': tooltip,
			  }
			: {};

	onMount(() => {
		if (htmlElement) {
			const tooltip = new Tooltip(htmlElement);
			tooltip.enable();
		}
	});
</script>

<i
	bind:this={htmlElement}
	on:click
	role="button"
	tabindex="0"
	class="bx {iconName} {cssClass.indexOf('align-') === -1 ? 'align-' + align : ''} {cssClass}"
	style="font-size: {size}rem;{style}"
	{...tooltipAttr}
/>
