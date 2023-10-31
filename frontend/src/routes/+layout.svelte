<script lang="ts">
	// Global CSS
	import '@eonasdan/tempus-dominus/dist/css/tempus-dominus.min.css';
	import 'bootstrap-directional-buttons/dist/bootstrap-directional-buttons.min.css';
	import 'bootstrap/dist/css/bootstrap.min.css';
	import 'boxicons/css/boxicons.min.css';

	import '$lib/css/base.css';
	import '$lib/css/resalt.css';
	import 'highlight.js/styles/github.css';

	import { ApiError, getConfig } from '$lib/api';
	import { config, theme, toasts } from '$lib/stores';
	import type Config from '$model/Config';
	import { MessageType } from '$model/MessageType';
	import { afterUpdate, onMount } from 'svelte';
	import { Toast } from 'bootstrap';
	import type Message from '$model/Message';

	let errorLoadingConfig = false;

	onMount(() => {
		getConfig()
			.then((data: Config) => {
				config.set(data);
				// set default color if theme.color is null
				if (!$theme.color) {
					$theme.color = data.themeDefaultColor;
				}
				// reset color if switching is disabled
				if (!data.themeEnableSwitching) {
					$theme.color = data.themeDefaultColor;
				}
			})
			.catch((err: unknown) => {
				console.error(err);

				errorLoadingConfig = true;

				toasts.add(MessageType.ERROR, 'Failed to load API Config', err);
			});
	});

	let alreadyShownToasts: string[] = [];

	function cleanupList(toasts: Message[]) {
		// remove old toasts
		alreadyShownToasts = alreadyShownToasts.filter((id) => {
			return toasts.find((toast) => toast.id === id);
		});
	}

	$: cleanupList($toasts);

	afterUpdate(() => {
		// init toasts
		$toasts.forEach((toast) => {
			if (alreadyShownToasts.includes(toast.id)) return;
			const htmlToast = document.getElementById('toast-' + toast.id);
			if (htmlToast) {
				const bsToast = new Toast(htmlToast, {
					autohide: true,
					delay: 5000,
				});
				bsToast.show();
				alreadyShownToasts.push(toast.id);
			}
		});
	});
</script>

<main class="app">
	{#if $config === null || $theme.color === null || errorLoadingConfig}
		{#if errorLoadingConfig}
			<p>Failed to load config from server. Please try again later.</p>
		{:else}
			<p>Loading....</p>
		{/if}
	{:else}
		<slot />
	{/if}

	<!-- Toast / Alerts -->
	<div class="position-fixed top-0 end-0 mt-5 me-5" style="z-index: 11">
		{#each $toasts as toast}
			<div
				id="toast-{toast.id}"
				class="toast {'toast-' + toast.type} mb-2"
				role="alert"
				aria-live="assertive"
				aria-atomic="true"
			>
				<div class="toast-header fw-bold">
					{toast.title}
				</div>
				{#if toast.message instanceof ApiError}
					<div class="toast-body">
						<strong>Code: </strong>{toast.message.code}<br />
						<strong>Data: </strong>{toast.message.message}
					</div>
				{:else}
					<div class="toast-body">
						{toast.message}
					</div>
				{/if}
			</div>
		{/each}
	</div>
</main>

<style>
	main {
		width: 100vw;
		height: 100vh;
		max-width: 100vw;
		max-height: 100vh;
		overflow-x: hidden;
		overflow-y: hidden;
	}
</style>
