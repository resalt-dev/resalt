<!-- a11y <div> which duplicates on:click to on:keypress -->
<script lang="ts">
	type A11yType = 'div' | 'span' | 'th';
	let event: any,
		type: A11yType = 'div',
		disabled: boolean = false,
		inputProps: any;
	$: ({ event, type, disabled, ...inputProps } = $$props);
	// Check if props.class contain "mouse-pointer", if not add it
	$: if (disabled) {
		if (!inputProps.class?.includes('no-select')) {
			inputProps.class = inputProps.class ? `${inputProps.class} no-select` : 'no-select';
		}
	} else {
		if (!inputProps.class?.includes('mouse-pointer')) {
			inputProps.class = inputProps.class
				? `${inputProps.class} mouse-pointer`
				: 'mouse-pointer';
		}
	}

	function wr(event: any): () => any {
		return () => {
			if (disabled) {
				return;
			}
			event();
		};
	}
</script>

{#if type === 'div' || type === undefined}
	<div {...inputProps} on:click={wr(event)} on:keypress={wr(event)}>
		<slot />
	</div>
{:else if type === 'span'}
	<span {...inputProps} on:click={wr(event)} on:keypress={wr(event)}>
		<slot />
	</span>
{:else if type === 'th'}
	<th {...inputProps} on:click={wr(event)} on:keypress={wr(event)}>
		<slot />
	</th>
{:else}
	WARNING: A11y component type not supported: {type}.
{/if}
