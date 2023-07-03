<script lang="ts">
	import { config } from '$lib/stores';
	import JsonViewer from '../../../components/JsonViewer.svelte';
</script>

<svelte:head>
	<title>Settings</title>
</svelte:head>

<JsonViewer data={$config} />
