<script lang="ts">
	import JsonViewer from '$component/JsonViewer.svelte';
	import { config } from '$lib/stores';
</script>

<svelte:head>
	<title>Settings</title>
</svelte:head>

<JsonViewer data={$config} />
