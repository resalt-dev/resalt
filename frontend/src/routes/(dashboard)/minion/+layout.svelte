<script lang="ts">
	import paths from '$lib/paths';
	import Tabs from '../../../components/Tabs.svelte';
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
