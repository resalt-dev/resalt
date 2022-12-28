<script lang="ts">
	import { config } from '../../stores';
	import JsonViewer from '../../components/JsonViewer.svelte';
</script>

<JsonViewer data={$config} />
