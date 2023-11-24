<script lang="ts">
	import Icon from '$component/Icon.svelte';
	import { theme, toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';

	export let name: string;
	export let value: unknown;
	let cssClass = '';
	export { cssClass as class };
	export let size: 'sm' | 'md' | 'lg' = 'sm';

	$: iconPad = {
		sm: '0.15rem',
		md: '0.15rem',
		lg: '0.2rem',
	}[size];
	$: iconSize = {
		sm: '1',
		md: '1.25',
		lg: '1.5',
	}[size];
</script>

<button
	type="button"
	class="btn btn-{$theme.color} btn-{size} ms-2 {cssClass}"
	style="margin-bottom: -0.15rem;margin-top: -0.15rem;"
	on:click={() => {
		if (!value) {
			toasts.add(MessageType.ERROR, 'Nothing to Copy', `No ${name} to copy.`);
			return;
		}
		navigator.clipboard.writeText(value + '');
		toasts.add(
			MessageType.SUCCESS,
			'Copied to Clipboard',
			`Copied ${name} "${value}" to clipboard.`,
		);
	}}
>
	<Icon
		name="clipboard"
		size={iconSize}
		align="top"
		style="padding-top: {iconPad};padding-bottom: {iconPad};"
	/>
</button>
