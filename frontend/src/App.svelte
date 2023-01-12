<script lang="ts">
	import DashboardLayout from './layouts/Dashboard/DashboardLayout.svelte';
	import PortalLayout from './layouts/Portal/PortalLayout.svelte';
	import { ApiError, getConfig } from './api';
	import { onMount } from 'svelte';
	import { config, theme, toasts } from './stores';
	import type Config from './models/Config';
	import { globalHistory, type NavigatorHistory } from 'svelte-navigator';
	import { writable } from 'svelte/store';
	import { Toast, ToastBody, ToastHeader } from 'sveltestrap';
	import WrapperGlobalHistory from './models/WrapperGlobalHistory';
	import { MessageType } from './models/MessageType';

	const isPortalView = writable<boolean>(window.location.pathname.startsWith('/auth'));
	let errorLoadingConfig = false;

	function onUrlChange() {
		let result = window.location.pathname.startsWith('/auth');
		// console.log(
		//     'onUrlChange',
		//     window.location.pathname,
		//     result,
		//     get(config) === null,
		//     get(theme).color === null,
		// );
		if (result !== $isPortalView) {
			isPortalView.set(result);
		}
	}

	// Wrap globalHistory, pass forward all function calls, except also call onUrlChange() on every change.
	function wrapFunction<A extends any[], R>(someFunction: (...a: A) => R) {
		const wrappedFunction = function (...args: A) {
			let result = someFunction(...args);
			onUrlChange();
			return result;
		};
		return { execute: wrappedFunction };
	}

	const wrapperGlobalHistory: NavigatorHistory = new WrapperGlobalHistory(
		globalHistory.location,
		globalHistory.listen,
		wrapFunction(globalHistory.navigate).execute as any,
	);

	onMount(() => {
		getConfig()
			.then((data: Config) => {
				config.set(data);
				// set default color if theme.color is null
				if (!$theme.color) {
					$theme.color = data.themeDefaultColor;
					$theme.dark = data.themeDefaultDark;
				}
				// reset color if switching is disabled
				if (!data.themeEnableSwitching) {
					$theme.color = data.themeDefaultColor;
				}
			})
			.catch((err) => {
				console.error(err);

				config.set(null);
				errorLoadingConfig = true;

				toasts.add(MessageType.ERROR, 'Failed to load API Config', err);
			});
	});
</script>

<main class={$theme.dark ? 'theme-dark' : ''}>
	{#if $config === null || $theme.color === null}
		{#if errorLoadingConfig}
			<p>Failed to load config from server. Please try again later.</p>
		{:else}
			<p>Loading....</p>
		{/if}
	{:else if $isPortalView}
		<PortalLayout history={wrapperGlobalHistory} />
	{:else}
		<DashboardLayout history={wrapperGlobalHistory} />
	{/if}

	<!-- Toast / Alerts -->
	<div class="position-fixed top-0 end-0 mt-5 me-5" style="z-index: 11">
		{#each $toasts as toast}
			<Toast class="{'toast-' + toast.type} mb-2">
				<ToastHeader>{toast.title}</ToastHeader>
				{#if toast.message instanceof ApiError}
					<ToastBody>
						<strong>Code: </strong>{toast.message.code}<br />
						<strong>Data: </strong>{toast.message.message}
					</ToastBody>
				{:else}
					<ToastBody>{toast.message}</ToastBody>
				{/if}
			</Toast>
		{/each}
	</div>
</main>

<!-- svelte-ignore css-unused-selector -->
<style lang="scss" global>
	@use './styles/global/_global.scss';

	main {
		width: 100vw;
		height: 100vh;
		max-width: 100vw;
		max-height: 100vh;
		overflow-x: hidden;
		overflow-y: hidden;
	}
</style>
