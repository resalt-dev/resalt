<script lang="ts">
	import { toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';

	let inputValue = '';
	let data: any = undefined;

	function reviewImport() {
		try {
			data = JSON.parse(inputValue);
		} catch (e) {
			data = undefined;
			toasts.add(MessageType.ERROR, 'Invalid JSON', 'The JSON you provided is invalid.');
		}
	}

	function submitImport() {
		// TODO
	}
</script>

<svelte:head>
	<title>Import</title>
</svelte:head>

{#if data === undefined}
	<button type="button" class="btn btn-warning" on:click={reviewImport}>Pre-Review Data</button>

	<hr class="text-light" />

	<textarea
		bind:value={inputValue}
		class="form-control"
		rows="10"
		placeholder="Paste your data here..."
	></textarea>
{:else}
	<button type="button" class="btn btn-danger" on:click={submitImport}>Import Data</button>

	<hr class="text-light" />

	<pre>{JSON.stringify(data, null, 2)}</pre>
{/if}
