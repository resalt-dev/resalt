<script lang="ts">
	import paths from '$lib/paths';
	import Tabs from '../../../components/Tabs.svelte';
</script>

<Tabs tabs={[paths.users_list, paths.users_add]}>
	<slot />
</Tabs>
