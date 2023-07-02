<!-- a11y <div> which duplicates on:click to on:keypress -->
<script lang="ts">
	type A11yType = 'div' | 'span' | 'tr' | 'td' | 'th' | 'button';
	export let type: A11yType;
	export let disabled = false;
	let event: () => void;
	let inputProps: {
		class?: string;
		[key: string]: unknown;
	};
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

	function wr(event: () => void): () => void {
		return () => {
			if (disabled) {
				return;
			}
			event();
		};
	}
</script>

{#if type === 'div' || type === undefined}
	<div {...inputProps} on:click={wr(event)} on:keypress={wr(event)} role="button" tabindex="0">
		<slot />
	</div>
{:else if type === 'span'}
	<span {...inputProps} on:click={wr(event)} on:keypress={wr(event)} role="button" tabindex="0">
		<slot />
	</span>
{:else if type === 'tr'}
	<tr {...inputProps} on:click={wr(event)} on:keypress={wr(event)}>
		<slot />
	</tr>
{:else if type === 'td'}
	<td {...inputProps} on:click={wr(event)} on:keypress={wr(event)}>
		<slot />
	</td>
{:else if type === 'th'}
	<th {...inputProps} on:click={wr(event)} on:keypress={wr(event)}>
		<slot />
	</th>
{:else if type === 'button'}
	<button {...inputProps} on:click={wr(event)} on:keypress={wr(event)}>
		<slot />
	</button>
{:else}
	WARNING: A11y component type not supported: {type}.
{/if}
