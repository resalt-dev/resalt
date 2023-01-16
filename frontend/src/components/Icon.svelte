<script lang="ts">
	export let type = 'regular';
	export let name = 'rocket';
	export let size = '2';
	export let align: 'top' | 'middle' | 'bottom' = 'middle';
	let cssClass = '';
	export { cssClass as class };
	export let htmlElement = null;
	export let style = '';
	export let tooltip = '';

	$: iconName = (type === 'solid' ? 'bxs-' : type === 'logo' ? 'bxl-' : 'bx-') + name;
	$: tooltipAttr =
		tooltip && tooltip.length > 0
			? {
					'data-bs-toggle': 'tooltip',
					'data-bs-placement': 'top',
					title: tooltip,
			  }
			: {};
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<i
	bind:this={htmlElement}
	on:click
	class="bx {iconName} {cssClass.indexOf('align-') === -1 ? 'align-' + align : ''} {cssClass}"
	style="font-size: {size}rem;{style}"
	{...tooltipAttr}
/>
