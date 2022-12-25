<!-- a11y <div> which duplicates on:click to on:keypress -->
<script lang="ts">
    type A11yType = 'div' | 'span';
    let event: any, type: A11yType = 'div', inputProps: any;
    $: ({ event, ...inputProps } = $$props);
</script>

{#if type === 'div'}
<div {...inputProps} on:click={event} on:keypress={event}>
    <slot></slot>
</div>
{:else if type === 'span'}
<span {...inputProps} on:click={event} on:keypress={event}>
    <slot></slot>
</span>
{:else}
    WARNING: A11y component type not supported: {type}.
{/if}



