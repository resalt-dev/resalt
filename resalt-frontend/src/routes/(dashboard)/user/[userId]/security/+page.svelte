<script lang="ts">
	import { page } from '$app/stores';
	import { getUserById, updateUserPassword } from '$lib/api';
	import { P_ADMIN_USER, P_USER_PASSWORD, hasResaltPermission } from '$lib/perms';
	import { currentUser, replacementParams, theme, toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';
	import type User from '$model/User';
	import { onMount } from 'svelte';
	import { writable, type Writable } from 'svelte/store';

	const PASSWORD_MIN_LENGTH = 8;
	const user: Writable<User | null> = writable(null);

	$: userId = $page.params.userId;

	let passwordFieldValue = '';
	let passwordFieldError = false;
	let repeatPasswordFieldValue = '';
	let repeatPasswordFieldError = false;

	function updateData(): void {
		getUserById(userId)
			.then((data) => {
				user.set(data);
				replacementParams.set({ ...$replacementParams, userId: data.username });
			})
			.catch((err) => {
				toasts.add(MessageType.ERROR, 'Failed fetching user: ' + userId, err);
			});
	}

	function updatePassword() {
		validatePasswordField();
		validateRepeatPasswordField();
		if (passwordFieldError || repeatPasswordFieldError) {
			return;
		}

		updateUserPassword(userId, passwordFieldValue)
			.then(() => {
				// OK!
				passwordFieldValue = '';
				passwordFieldError = false;
				repeatPasswordFieldValue = '';
				repeatPasswordFieldError = false;
				updateData();
			})
			.catch((err) => {
				toasts.add(MessageType.ERROR, 'Failed updating password for user: ' + userId, err);
			});
	}

	/*
    // VALIDATION
    */

	function validatePasswordField(): void {
		validateRepeatPasswordField();

		passwordFieldError = false;

		if (passwordFieldValue.length < PASSWORD_MIN_LENGTH) {
			passwordFieldError = true;
			return;
		}
	}

	function validateRepeatPasswordField(): void {
		repeatPasswordFieldError = false;

		if (repeatPasswordFieldValue.length < PASSWORD_MIN_LENGTH) {
			repeatPasswordFieldError = true;
			return;
		}
		if (passwordFieldValue !== repeatPasswordFieldValue) {
			repeatPasswordFieldError = true;
			return;
		}
	}

	onMount(() => {
		updateData();
	});

	function getTitle(user: User | null): string {
		return 'Security :: ' + (user ? user.username : userId);
	}
</script>

<svelte:head>
	<title>{getTitle($user)}</title>
</svelte:head>

{#if !$user}
	<h1>Loading...</h1>
{:else}
	<div class="row">
		{#if hasResaltPermission($currentUser, P_ADMIN_USER) || ($currentUser?.id === $user.id && hasResaltPermission($currentUser, P_USER_PASSWORD))}
			<div class="col-12 col-xxl-4 pb-3">
				<div class="card h-100">
					<div class="card-header">Password</div>
					<div class="card-body">
						<div class="form-floating mb-3">
							<input
								id="password1"
								type="password"
								class="form-control {passwordFieldError ? 'is-invalid' : ''}"
								bind:value={passwordFieldValue}
								on:blur={validatePasswordField}
							/>
							<label class="form-label" for="password1">New password</label>
						</div>
						<div class="form-floating mb-3">
							<input
								id="password2"
								type="password"
								class="form-control {repeatPasswordFieldError &&
								repeatPasswordFieldValue.length > 0
									? 'is-invalid'
									: ''}"
								bind:value={repeatPasswordFieldValue}
								on:keyup={validateRepeatPasswordField}
							/>
							<label class="form-label" for="password2">Confirm password</label>
						</div>
						{#if passwordFieldError}
							<p class="text-danger mt-3">
								Password must be at least {PASSWORD_MIN_LENGTH} characters long.
							</p>
						{/if}
						{#if repeatPasswordFieldError}
							<p class="text-danger mt-3">Passwords do not match.</p>
						{/if}
						<button class="btn btn-{$theme.color}" on:click={updatePassword}
							>Update</button
						>
					</div>
				</div>
			</div>
		{/if}
	</div>
{/if}
