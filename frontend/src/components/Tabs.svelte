<script lang="ts">
	import { Alert, Card, CardBody } from 'sveltestrap';
	import { Link } from 'svelte-navigator';
	import { theme } from '../stores';
	import type TabPage from '../models/TabPage';

	export let tabs: TabPage[] = [];
	export let current: string;
	$: _current = current.length > 0 ? current : tabs[0].key;
	$: _currentTab = tabs.findIndex(
		(tab) => current.length === 0 || tab.key.toLowerCase() === current,
	);
</script>

<div class="nav bg-dark w-100 no-select">
	{#each tabs.filter((tab) => !tab.hidden) as tab}
		<Link
			to={tab.path}
			class="nav-link px-4 py-3 fw-bold mouse-pointer {tab.key === _current
				? 'bg-' + $theme.color
				: ''} {$theme.color === 'yellow' && tab.key === _current
				? 'text-dark'
				: 'text-white'}"
		>
			{tab.label}
		</Link>
	{/each}
</div>

<Card class="mb-3 border border-4 border-{$theme.color} rounded-none bg-none">
	<CardBody>
		{#if _currentTab === -1 || tabs[_currentTab].hidden}
			<!-- Either disled or no permissions. -->
			<Alert color="warning" class="mb-0" fade={false}>
				<h4 class="alert-heading">Unavailable</h4>
				You do not have permission to view this page, or it has been disabled.
				<br />
				<br />
				<Link to="/" class="link-dark">Return to home.</Link>
			</Alert>
		{:else}
			<svelte:component this={tabs[_currentTab].component} {...tabs[_currentTab].data} />
		{/if}
	</CardBody>
</Card>
