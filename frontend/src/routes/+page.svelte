<script>
	import paths from '$lib/paths';
	import { auth } from '$lib/stores';
</script>

<svelte:head>
	<title>Resalt</title>
	<meta
		name="description"
		content="Resalt is a free, open-source, self-hosted, web-based administration and monitoring panel for SaltStack."
	/>
</svelte:head>

{#if $auth === null}
	<meta http-equiv="refresh" content="0;url={paths.login.getPath()}" />
{:else}
	<meta http-equiv="refresh" content="0;url={paths.dashboard.getPath()}" />
{/if}
