<script lang="ts">
	import { goto } from '$app/navigation';
	import ResaltProgress from '$component/ResaltProgress.svelte';
	import { login } from '$lib/api';
	import paths from '$lib/paths';
	import { config, theme, toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';
	import { onMount } from 'svelte';

	let usernameField: HTMLInputElement;
	let usernameFieldValue = '';
	let usernameFieldError = false;
	let passwordFieldValue = '';
	let passwordFieldError = false;

	onMount(() => {
		if ($config !== null && $config.authForwardEnabled) {
			_login();
		} else {
			usernameField.focus();
		}
	});

	function formLogin() {
		validateUsernameField();
		validatePasswordField();

		if (usernameFieldError || passwordFieldError) {
			return;
		}

		_login();
	}

	function _login() {
		let username = usernameFieldValue;
		let password = passwordFieldValue;
		login(username, password)
			.then(() => {
				goto(paths.dashboard.getPath());
			})
			.catch((err: unknown) => {
				// Check if error msg contains "Internal error"
				if (err instanceof Error && err.message.includes('Internal error')) {
					// If so, it's probably a 500 error
					toasts.add(
						MessageType.ERROR,
						'Login Error',
						'An internal error occurred. Retrying...',
					);
					// Try again after timeout
					setTimeout(() => {
						_login();
					}, 1000);
				} else {
					toasts.add(MessageType.ERROR, 'Login Error', err);
				}
			});
	}

	/*
    // VALIDATION
    */

	function validateUsernameField(): void {
		if (usernameFieldValue.length === 0) {
			usernameFieldError = true;
			return;
		}
		usernameFieldError = false;
	}

	function validatePasswordField(): void {
		if (passwordFieldValue.length < 8) {
			passwordFieldError = true;
			return;
		}
		passwordFieldError = false;
	}
</script>

<svelte:head>
	<title>Login</title>
</svelte:head>

{#if $config !== null && $config.authForwardEnabled}
	<p class="fw-bold">SSO (Single Sign On) enabled. Please wait while authenticating...</p>

	<ResaltProgress />
{:else}
	<p class="fw-bold">This is a restricted admin area. Please log in to continue.</p>
	<br />
	<form action="javascript:void(0);" autocomplete="false">
		<div class="form-floating mb-3">
			<input
				id="username"
				type="text"
				class="form-control {usernameFieldError ? 'is-invalid' : ''}"
				bind:value={usernameFieldValue}
				on:blur={validateUsernameField}
				bind:this={usernameField}
			/>
			<label class="form-label" for="username">Username</label>
		</div>
		<div class="form-floating mb-3">
			<input
				id="password"
				type="password"
				class="form-control {passwordFieldError ? 'is-invalid' : ''}"
				bind:value={passwordFieldValue}
				on:blur={validatePasswordField}
			/>
			<label class="form-label" for="password">Password</label>
		</div>

		<br />

		<button type="submit" on:click={formLogin} class="btn btn-{$theme.color} px-5 fw-bold mb-3">
			Sign in
		</button>
	</form>
{/if}
