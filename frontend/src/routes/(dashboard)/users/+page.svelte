<script lang="ts">
	import paths from '$lib/paths';
	import Tabs from '../../../components/Tabs.svelte';
</script>

<svelte:head>
	<title>Users</title>
</svelte:head>

<Tabs tabs={[paths.users_list, paths.users_new]}>
	<slot />
</Tabs>
