<!-- a11y <div> which duplicates on:click to on:keypress -->
<script lang="ts">
	type A11yType = 'div' | 'span' | 'tr' | 'td' | 'th' | 'button';
	export let type: A11yType;
	export let disabled: boolean = false;
	let event: any, inputProps: any;
	$: ({ event, ...inputProps } = $$props);
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
	<div role="button" tabindex="0" {...inputProps} on:click={wr(event)} on:keypress={wr(event)}>
		<slot />
	</div>
{:else if type === 'span'}
	<span role="button" tabindex="0" {...inputProps} on:click={wr(event)} on:keypress={wr(event)}>
		<slot />
	</span>
{:else if type === 'tr'}
	<tr role="button" tabindex="0" {...inputProps} on:click={wr(event)} on:keypress={wr(event)}>
		<slot />
	</tr>
{:else if type === 'td'}
	<td role="button" tabindex="0" {...inputProps} on:click={wr(event)} on:keypress={wr(event)}>
		<slot />
	</td>
{:else if type === 'th'}
	<th role="button" tabindex="0" {...inputProps} on:click={wr(event)} on:keypress={wr(event)}>
		<slot />
	</th>
{:else if type === 'button'}
	<button role="button" tabindex="0" {...inputProps} on:click={wr(event)} on:keypress={wr(event)}>
		<slot />
	</button>
{:else}
	WARNING: A11y component type not supported: {type}.
{/if}
