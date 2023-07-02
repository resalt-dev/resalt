<script lang="ts">
	import paths from '$lib/paths';
	import Tabs from '../../../components/Tabs.svelte';
</script>

<svelte:head>
	<title>Dashboard</title>
</svelte:head>

<Tabs tabs={[paths.dashboard]}>
	<slot />
</Tabs>
