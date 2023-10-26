<script lang="ts">
	import CopyButton from '$component/CopyButton.svelte';
	import JsonViewer from '$component/JsonViewer.svelte';
	import { getExport } from '$lib/api';
	import { toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';
	import { writable, type Writable } from 'svelte/store';

	const data: Writable<unknown> = writable(undefined);

	function getData() {
		getExport().then((res) => {
			try {
				data.set(res);
			} catch (e) {
				toasts.add(MessageType.ERROR, 'Failed parsing Export', e);
			}
		});
	}
</script>

<svelte:head>
	<title>Export</title>
</svelte:head>

<button type="button" class="btn btn-warning" on:click={getData}>Generate Export</button>

{#if $data !== undefined}
	<CopyButton name="Export" value={JSON.stringify($data)} size="md" class="btn-dark" />
{/if}

<hr class="text-light" />

<JsonViewer data={$data} />
