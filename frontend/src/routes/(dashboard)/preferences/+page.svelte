<script lang="ts">
	import paths from '$lib/paths';
	import Tabs from '../../../components/Tabs.svelte';
</script>

<svelte:head>
	<title>Preferences</title>
</svelte:head>

<Tabs tabs={[paths.preferences]}>
	<slot />
</Tabs>
