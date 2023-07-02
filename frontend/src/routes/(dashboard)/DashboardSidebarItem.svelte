<script lang="ts">
	import { page } from '$app/stores';
	import { theme } from '$lib/stores';
	import Icon from '../../components/Icon.svelte';
	import type { Path } from '$lib/paths';

	export let path: Path;
	export let collapsed: boolean;

	$: slugPath = path.getPath().split('/')[1] ?? '';
	$: slugLocation = $page.url.pathname.split('/')[1];
	//$: console.log('path', slugPath, slugLocation);
	$: isActive = slugPath.startsWith(slugLocation);
</script>

<li class="nav-item" style="height: 4.5rem;">
	<a
		href={path.getPath()}
		class="nav-link {$theme.color === 'yellow' && isActive
			? 'text-dark'
			: 'text-white'} {isActive ? '' : 'fw-light'} d-flex align-items-center {isActive
			? `bg-${$theme.color}`
			: ''}"
		style="height: inherit;"
	>
		<Icon name={path.icon} class="ps-1 {collapsed ? '' : 'me-3'}" />
		{#if !collapsed}
			<span>{path.label}</span>
		{/if}
	</a>
</li>
