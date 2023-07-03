<script lang="ts">
	import Tabs from '$component/Tabs.svelte';
	import paths from '$lib/paths';
</script>

<Tabs tabs={[paths.users_list, paths.users_add]}>
	<slot />
</Tabs>
