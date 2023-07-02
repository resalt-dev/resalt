<script lang="ts">
	import { config } from '$lib/stores';
	import JsonViewer from '../../../components/JsonViewer.svelte';
</script>

<JsonViewer data={$config} />
