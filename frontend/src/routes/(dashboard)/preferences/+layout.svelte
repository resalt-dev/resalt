<script lang="ts">
	import paths from '$lib/paths';
	import Tabs from '../../../components/Tabs.svelte';
</script>

<Tabs tabs={[paths.preferences]}>
	<slot />
</Tabs>
