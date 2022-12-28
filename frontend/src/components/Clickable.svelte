<!-- a11y <div> which duplicates on:click to on:keypress -->
<script lang="ts">
	type A11yType = 'div' | 'span' | 'th';
	let event: any,
		type: A11yType = 'div',
		inputProps: any;
	$: ({ event, type, ...inputProps } = $$props);
	// Check if props.class contain "mouse-pointer", if not add it
	$: if (!inputProps.class?.includes('mouse-pointer')) {
		inputProps.class = inputProps.class ? `${inputProps.class} mouse-pointer` : 'mouse-pointer';
	}
</script>

{#if type === 'div' || type === undefined}
	<div {...inputProps} on:click={event} on:keypress={event}>
		<slot />
	</div>
{:else if type === 'span'}
	<span {...inputProps} on:click={event} on:keypress={event}>
		<slot />
	</span>
{:else if type === 'th'}
	<th {...inputProps} on:click={event} on:keypress={event}>
		<slot />
	</th>
{:else}
	WARNING: A11y component type not supported: {type}.
{/if}
