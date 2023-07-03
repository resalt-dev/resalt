<script lang="ts">
	import Icon from '$component/Icon.svelte';
	import { theme, toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';

	export let name: string;
	export let value: unknown;
</script>

<button
	class="btn btn-{$theme.color} btn-sm ms-2"
	style="margin-bottom: -0.15rem;margin-top: -0.15rem;"
	on:click={() => {
		navigator.clipboard.writeText(value + '');
		toasts.add(
			MessageType.SUCCESS,
			'Copied to Clipboard',
			`Copied ${name} "${value}" to clipboard.`,
		);
	}}
>
	<Icon name="clipboard" size="1" align="top" style="padding-top: 0.15rem;" />
</button>
