<script lang="ts">
	import { goto } from '$app/navigation';
	import { createUser } from '$lib/api';
	import { MessageType } from '../../../../models/MessageType';
	import paths from '$lib/paths';
	import { theme, toasts } from '$lib/stores';

	// Depending on if creating LOCAL or LDAP user, different fields are shown.
	// Email is only shown if creating LOCAL user, as otherwise synced from LDAP.
	// LDAP Sync is only shown if creating LDAP user.

	let userLDAPFieldValue = false;
	let userUsernameFieldValue = '';
	let userUsernameFieldError = false;
	let userEmailFieldValue = '';
	let userEmailFieldError = false;
	let userLDAPSyncFieldValue = '';
	let userLDAPSyncFieldError = false;

	function _create(): void {
		if (!_validate()) {
			return;
		}

		createUser(
			userUsernameFieldValue,
			userEmailFieldValue.length === 0 ? null : userEmailFieldValue,
			userLDAPSyncFieldValue.length === 0 ? null : userLDAPSyncFieldValue,
		)
			.then((user) => {
				goto(paths.user.getPath(user.id));
				toasts.add(MessageType.SUCCESS, 'User created', `User ${user.username} created`);
			})
			.catch((err) => {
				toasts.add(MessageType.ERROR, 'Failed creating user', err);
			});
	}

	/*
    // VALIDATION
    */

	function _validate(): boolean {
		validateUserUsernameField();
		validateUserEmailField();
		validateUserLDAPSyncField();

		if (userLDAPFieldValue) {
			return !userUsernameFieldError && !userLDAPSyncFieldError;
		} else {
			return !userUsernameFieldError && !userEmailFieldError;
		}
	}

	function validateUserLDAPField(): void {
		//
	}

	function validateUserUsernameField(): void {
		userUsernameFieldError = false;
	}

	function validateUserEmailField(): void {
		userEmailFieldError = false;
	}

	function validateUserLDAPSyncField(): void {
		userLDAPSyncFieldError = false;
	}
</script>

<svelte:head>
	<title>Add user</title>
</svelte:head>

<div class="card mb-3">
	<div class="card-header">Create User</div>
	<div class="card-body">
		<div class="row">
			<div class="col col-md-5 col-lg-2 mb-0">
				<div class="form-floating mb-3">
					<input
						id="userUsername"
						type="text"
						class="form-control {userUsernameFieldError ? 'is-invalid' : ''}"
						bind:value={userUsernameFieldValue}
						on:blur={validateUserUsernameField}
					/>
					<label class="form-label" for="userUsername">Username</label>
				</div>
			</div>
			<div class="col col-md-2 col-lg-1 mb-0">
				<div class="d-flex justify-content-center">
					<label class="form-label mb-0 me-2" for="userLDAP">LDAP</label>
				</div>
				<div class="clearfix" />
				<div class="d-flex justify-content-center">
					<div class="form-floating mb-3 ps-0">
						<div class="fs-3 mt-0 form-check form-switch">
							<input
								id="userLDAP"
								type="checkbox"
								class="form-check-input"
								bind:checked={userLDAPFieldValue}
								on:blur={validateUserLDAPField}
							/>
						</div>
					</div>
				</div>
			</div>
			{#if userLDAPFieldValue}
				<div class="col col-md-5 col-lg-5 mb-0">
					<div class="form-floating mb-3">
						<input
							id="userLDAPSync"
							type="text"
							class="form-control {userLDAPSyncFieldError ? 'is-invalid' : ''}"
							bind:value={userLDAPSyncFieldValue}
							on:blur={validateUserLDAPSyncField}
						/>
						<label class="form-label" for="userLDAPSync">LDAP Sync DN</label>
					</div>
				</div>
			{:else}
				<div class="col col-md-5 col-lg-3 mb-0">
					<div class="form-floating mb-3">
						<input
							id="userEmail"
							type="email"
							class="form-control {userEmailFieldError ? 'is-invalid' : ''}"
							bind:value={userEmailFieldValue}
							on:blur={validateUserEmailField}
						/>
						<label class="form-label" for="userEmail">Email (optional)</label>
					</div>
				</div>
			{/if}
		</div>

		<button type="button" class="btn btn-{$theme.color}" on:click={_create}>Create User</button>
	</div>
</div>
