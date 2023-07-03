<script lang="ts">
	import Tabs from '$component/Tabs.svelte';
	import paths from '$lib/paths';
</script>

<Tabs tabs={[paths.settings_config, paths.settings_groups]}>
	<slot />
</Tabs>
