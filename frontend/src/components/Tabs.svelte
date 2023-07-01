<script lang="ts">
	import { Link } from 'svelte-navigator';
	import { theme } from '../stores';
	import type TabPage from '../models/TabPage';

	export let tabs: TabPage[] = [];
	export let current: string;
	$: _currentTabKey = current.length > 0 ? current.split('/')[0] : tabs[0].key;
	$: _currentTabIndex = tabs.findIndex(
		(tab) => _currentTabKey.length === 0 || tab.key.toLowerCase() === _currentTabKey,
	);
</script>

<div class="nav bg-dark w-100 no-select">
	{#each tabs.filter((tab) => !tab.hidden) as tab}
		<Link
			to={tab.path}
			class="nav-link px-4 py-3 fw-bold mouse-pointer {tab.key === _currentTabKey
				? 'bg-' + $theme.color
				: ''} {$theme.color === 'yellow' && tab.key === _currentTabKey
				? 'text-dark'
				: 'text-white'}"
		>
			{tab.label}
		</Link>
	{/each}
</div>

<div class="card mb-3 border border-4 border-{$theme.color} rounded-none bg-none">
	<div class="card-body">
		{#if _currentTabIndex === -1 || tabs[_currentTabIndex].hidden}
			<!-- Either disled or no permissions. -->
			<div class="alert alert-warning mb-0" role="alert">
				<h4 class="alert-heading">Unavailable</h4>
				You do not have permission to view this page, or it has been disabled.
				<br />
				<br />
				<Link to="/" class="link-dark">Return to home.</Link>
			</div>
		{:else}
			<svelte:component
				this={tabs[_currentTabIndex].component}
				{...tabs[_currentTabIndex].data}
			/>
		{/if}
	</div>
</div>
