<script lang="ts">
	import paths from '$lib/paths';
	import Tabs from '../../../components/Tabs.svelte';
</script>

<svelte:head>
	<title>Settings</title>
</svelte:head>

<Tabs tabs={[paths.settings_config, paths.settings_groups]}>
	<slot />
</Tabs>
