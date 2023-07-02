<script lang="ts">
	import { getUserById, updateUserPassword } from '$lib/api';
	import { theme, currentUser, toasts } from '$lib/stores';
	import { writable, type Writable } from 'svelte/store';
	import { onMount } from 'svelte';
	import { MessageType } from '../../../models/MessageType';
	import JsonViewer from '../../../components/JsonViewer.svelte';
	import type User from '../../../models/User';
	import { hasResaltPermission, P_USER_ADMIN, P_USER_PASSWORD } from '$lib/perms';
	import CopyButton from '../../../components/CopyButton.svelte';

	const PASSWORD_MIN_LENGTH: number = 8;

	export let userId: string;

	const user: Writable<User | null> = writable(null);

	let passwordFieldValue: string = '';
	let passwordFieldError: boolean = false;
	let repeatPasswordFieldValue: string = '';
	let repeatPasswordFieldError: boolean = false;

	function updateData(): void {
		getUserById(userId)
			.then((data) => {
				user.set(data);
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
</script>

<svelte:head>
	{#if !$user}
		<title>User {userId}</title>
	{:else}
		<title>User {$user.username}</title>
	{/if}
</svelte:head>

{#if !$user}
	<h1>Loading...</h1>
{:else}
	<h1>
		User {$user.username}
		{#if $user.id === $currentUser?.id}
			<span class="text-{$theme.color}"> (You)</span>
		{/if}
	</h1>

	<div class="row">
		<div class="col-12 col-xxl-4 pb-3">
			<div class="card h-100 {$theme.dark ? 'bg-dark' : ''}">
				<div class="card-header">
					<h5 class="card-title mb-0">General</h5>
				</div>
				<ul class="list-group list-group-flush">
					<li class="list-group-item {$theme.dark ? 'bg-dark text-light' : ''}">
						<strong>ID</strong>
						<span class="float-end">
							{$user.id}
							<CopyButton name="User ID" value={$user.id} />
						</span>
					</li>
					<li class="list-group-item {$theme.dark ? 'bg-dark text-light' : ''}">
						<strong>Username</strong>
						<span class="float-end">{$user.username}</span>
					</li>
					<li class="list-group-item {$theme.dark ? 'bg-dark text-light' : ''}">
						<strong>Last Login</strong>
						<span class="float-end">
							{#if $user.lastLogin}
								{$user.lastLogin}
							{:else}
								<em>Never</em>
							{/if}
						</span>
					</li>
					<li class="list-group-item {$theme.dark ? 'bg-dark text-light' : ''}">
						<strong>Email</strong>
						<span class="float-end">
							{#if $user.email}
								{$user.email}
							{:else}
								<em>Not set</em>
							{/if}
						</span>
					</li>
					<li class="list-group-item {$theme.dark ? 'bg-dark text-light' : ''}">
						<strong>LDAP Sync DN</strong>
						<span class="float-end">
							{#if $user.ldapSync}
								{$user.ldapSync}
							{:else}
								<em>Not set</em>
							{/if}
						</span>
					</li>
				</ul>
			</div>
		</div>
		{#if hasResaltPermission($currentUser, P_USER_ADMIN) || ($currentUser?.id === $user.id && hasResaltPermission($currentUser, P_USER_PASSWORD))}
			<div class="col-12 col-xxl-4 pb-3">
				<div class="card h-100 {$theme.dark ? 'bg-dark' : ''}">
					<div class="card-header">
						<h5 class="card-title mb-0">Password</h5>
					</div>
					<div class="card-body">
						<div class="form-floating mb-3">
							<input
								id="password1"
								type="password"
								class="form-control {passwordFieldError ? 'is-invalid' : ''}"
								disabled={$user.ldapSync !== null}
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
								disabled={$user.ldapSync !== null}
								bind:value={repeatPasswordFieldValue}
								on:keyup={validateRepeatPasswordField}
							/>
							<label class="form-label" for="password2">Confirm password</label>
						</div>
						{#if $user.ldapSync !== null}
							<p class="text-muted mt-3">
								This user is synced with LDAP. Passwords can only be changed in
								LDAP.
							</p>
						{/if}
						{#if passwordFieldError}
							<p class="text-danger mt-3">
								Password must be at least {PASSWORD_MIN_LENGTH} characters long.
							</p>
						{/if}
						{#if repeatPasswordFieldError}
							<p class="text-danger mt-3">Passwords do not match.</p>
						{/if}
						<button
							disabled={$user.ldapSync !== null}
							class="btn btn-{$theme.color}"
							on:click={updatePassword}>Update</button
						>
					</div>
				</div>
			</div>
		{/if}
		<div class="col-12 col-xxl-4 pb-3">
			<div class="card h-100 {$theme.dark ? 'bg-dark' : ''}">
				<div class="card-header">
					<h5 class="card-title mb-0">Permissions</h5>
				</div>
				<JsonViewer data={$user.perms} sort={false} />
			</div>
		</div>
	</div>
{/if}
