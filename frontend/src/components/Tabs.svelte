<script lang="ts">
	import { page } from '$app/stores';
	import type { Path } from '$lib/paths';
	import { currentUser, theme } from '$lib/stores';

	export let tabs: Path[] = [];

	$: pathname = $page.url.pathname;
	$: params = $page.params;
	// $: console.log('= TABS', pathname, params);
	// $: tabs.forEach((tab) => console.log(tab.name, tab.getPath(params)));
	$: currentTab = tabs
		.filter((tab) => pathname.startsWith(tab.getPath(params)))
		.sort((a, b) => b.getPath(params).length - a.getPath(params).length)
		.shift();
</script>

<!-- Tabs Header -->
<div class="nav bg-dark w-100 no-select">
	{#each tabs.filter((tab) => tab.hasPermission($currentUser)) as tab}
		<a
			href={tab.getBarePath()}
			class="nav-link px-4 py-3 fw-bold mouse-pointer {tab === currentTab
				? 'bg-' + $theme.color
				: ''} {$theme.color === 'yellow' && tab === currentTab
				? 'text-dark'
				: 'text-white'}"
		>
			{tab.label}
		</a>
	{/each}
</div>

<!-- Tabs Content -->
{#if currentTab !== undefined}
	<div class="card mb-3 border border-4 border-{$theme.color} rounded-none bg-none">
		<div class="card-body">
			{#if !currentTab.hasPermission($currentUser)}
				<div class="alert alert-warning mb-0" role="alert">
					<h4 class="alert-heading">Unavailable</h4>
					You do not have permission to view this page, or it has been disabled.
					<br />
					<br />
					<a href="/" class="link-dark">Return to home.</a>
				</div>
			{:else}
				<slot />
			{/if}
		</div>
	</div>
{/if}
