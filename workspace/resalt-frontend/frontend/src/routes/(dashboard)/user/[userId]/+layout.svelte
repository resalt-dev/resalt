<script lang="ts">
	import Tabs from '$component/Tabs.svelte';
	import paths from '$lib/paths';
</script>

<Tabs tabs={[paths.user_info, paths.user_security, paths.user_permissions]}>
	<slot />
</Tabs>
