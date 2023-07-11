<script lang="ts">
	import { page } from '$app/stores';
	import Icon from '$component/Icon.svelte';
	import paths from '$lib/paths';
	import { currentUser, replacementParams, socket, theme } from '$lib/stores';
	import type { Page } from '@sveltejs/kit';

	type NavbarItem = {
		title: string;
		path: string | null;
	};
	function generateNavbar(page: Page, replacements: Record<string, string>): NavbarItem[] {
		let navbar: NavbarItem[] = [];
		let route = (page.route.id ?? '').replace(/\/\(dashboard\)/, '');

		// If route = /minions/presets/[[presetId]], then split it so
		// 0: /minions
		// 1: /minions/presets
		// 2: /minions/presets/[[presetId]]
		let routeParts = route.split('/');
		let routePath = '';

		// Override-map for special cases where singular goes to plural
		let overrides: Record<string, string> = {
			'/minion': '/minions',
			'/user': '/users',
		};

		A: for (let i = 1; i < routeParts.length; i++) {
			let part = routeParts[i];
			routePath += '/' + part;
			let localRoutePath = routePath in overrides ? overrides[routePath] : routePath;

			// console.log(localRoutePath);

			let found = false;
			// Loop over all paths
			for (let path of Object.values(paths)) {
				// If the path matches the localRoutePath, then add it to the navbar
				let url = path.getRawPath();
				if (!part.includes('[')) {
					// Remove optional variables
					url = url.replace(/\/\[\[.+?\]\]/g, '');
				}
				// console.log('   .: ', url, localRoutePath);
				if (url === localRoutePath) {
					// If part contains [, then find the value inside the brackets and get it from page.params, otherwise use path.label
					let name;
					if (part.includes('[')) {
						let match = part
							.replaceAll('[[', '[')
							.replaceAll(']]', ']')
							.match(/\[(.+?)\]/);
						if (match) {
							let variable = match?.[1];
							if (variable in replacements) name = replacements[variable];
							else if (variable in page.params) name = page.params[variable];
							else continue A;
						} else {
							name = path.label;
						}
					} else {
						name = path.label;
					}

					// console.log('          ', url, '\t', localRoutePath, name);
					navbar.push({
						title: name,
						path: path.getPath(page.params),
					});
					found = true;
					break;
				}
			}
			if (!found) {
				navbar.push({
					title: part,
					path: null,
				});
			}
		}

		return navbar;
	}

	$: navbar = generateNavbar($page, $replacementParams);
</script>

<div id="dashboard-header" class="g-0 d-flex align-items-center bg-light">
	<div class="col">
		<div class="btn-group me-3" role="group">
			{#each navbar as item}
				{#if item.path}
					<a
						href={item.path}
						class={`btn btn-${$theme.color} ${
							$theme.color === 'yellow' ? '' : 'text-white'
						} btn-arrow-right fw-bold`}
					>
						{item.title}
					</a>
				{:else}
					<div class="btn btn-dark text-white btn-arrow-right fw-bold">
						{item.title}
					</div>
				{/if}
			{/each}
		</div>
	</div>
	<div class="col-auto pe-3 d-flex align-items-center">
		{#if $socket.connected}
			<!-- display lastPing as hh:mm:ss -->
			<span class="font-monospace pt-1 ps-3"
				>Connected: {new Date($socket.lastPing ?? new Date(0))
					.toLocaleTimeString('en-US', {
						timeZone: 'UTC',
						timeZoneName: 'short',
						hourCycle: 'h23',
					})
					.replace(/\./g, ':')}</span
			>
		{:else}
			<span class="font-monospace pt-1 text-danger">Disconnected</span>
		{/if}
	</div>
	<div class="col-auto">
		<div class="vr sep" />
	</div>
	<div class="col-auto px-3 text-reset text-decoration-none">
		<a
			href={paths.user_info.getPath($currentUser?.id ?? '')}
			class="text-decoration-none text-reset"
		>
			<Icon name="user" size="1.5" type="solid" class="pe-1" />
			{$currentUser?.username}
		</a>
	</div>
	<!-- <div class="col-auto">
		<div class="vr sep" />
	</div>
	<div class="col-auto px-3 text-reset text-decoration-none">
		<Icon name="bell" size="1.5" />
	</div> -->
	<div class="col-auto">
		<div class="vr sep" />
	</div>
	<a
		href={paths.logout.getPath()}
		class="col-auto px-3 text-reset text-decoration-none mouse-pointer"
	>
		<Icon name="log-out" size="1.5" class="pe-1" />
		Logout
	</a>
</div>

<style>
	.sep {
		margin-top: 0.4rem;
		height: 1.5rem;
	}
</style>
