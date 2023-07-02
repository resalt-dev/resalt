<script lang="ts">
	import Tabs from '../../../components/Tabs.svelte';
	import paths from '$lib/paths';
</script>

<svelte:head>
	<title>Minions</title>
</svelte:head>

<Tabs tabs={[paths.minions, paths.minions_presets, paths.minions_grains]}>
	<slot />
</Tabs>
