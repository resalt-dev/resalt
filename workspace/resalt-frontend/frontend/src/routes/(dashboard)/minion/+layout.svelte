<script lang="ts">
	import Tabs from '$component/Tabs.svelte';
	import paths from '$lib/paths';
</script>

<Tabs
	tabs={[
		paths.minion,
		paths.minion_grains,
		paths.minion_conformity,
		paths.minion_pillars,
		paths.minion_packages,
	]}
>
	<slot />
</Tabs>
