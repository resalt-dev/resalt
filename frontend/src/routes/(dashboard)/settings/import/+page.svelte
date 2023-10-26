<script lang="ts">
	import ResaltProgress from '$component/ResaltProgress.svelte';
	import { importData } from '$lib/api';
	import { toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';

	let inputValue = '';
	let data: any = undefined;
	let loading: boolean = false;

	function reviewImport() {
		try {
			data = JSON.parse(inputValue);
			if (data === undefined || data === null || typeof data !== 'object') {
				toasts.add(MessageType.ERROR, 'Invalid JSON', 'The JSON you provided is invalid.');
				return;
			}
			// Rewrite the above to use a for loop and check for the existence of all required fields
			for (const f of ['users', 'groups', 'minions']) {
				if (data[f].length < 1) {
					throw new Error('Invalid JSON');
				}
			}
		} catch (e) {
			data = undefined;
			console.error(e);
			toasts.add(MessageType.ERROR, 'Invalid JSON', 'The JSON you provided is invalid.');
		} finally {
			inputValue = '';
		}
	}

	function submitImport() {
		if (!confirm('Are you REALLY sure you want to import this data?')) {
			return;
		}
		loading = true;
		importData(data)
			.then(() => {
				toasts.add(MessageType.SUCCESS, 'Import Successful', 'The import was successful.');
				data = undefined;
			})
			.catch((e) => {
				toasts.add(MessageType.ERROR, 'Import Failed', 'Please see server logs.');
			})
			.finally(() => {
				loading = false;
			});
	}
</script>

<svelte:head>
	<title>Import</title>
</svelte:head>

{#if data === undefined}
	<button type="button" class="btn btn-warning" on:click={reviewImport}>Review Data</button>

	<hr class="text-light" />

	<textarea
		bind:value={inputValue}
		class="form-control"
		rows="10"
		placeholder="Paste your data here..."
	></textarea>
{:else}
	<button type="button" class="btn btn-danger" on:click={submitImport}
		>Ready to IMPORT?<br />Warning: This is DESTRUCTIVE!</button
	>

	{#if loading}
		<ResaltProgress />
	{/if}

	<hr class="text-light" />

	<strong>Users:</strong>
	{data.users.length}<br />
	<strong>Groups:</strong>
	{data.groups.length}<br />
	<strong>Minions:</strong>
	{data.minions.length}
{/if}
