<script lang="ts">
	import { goto } from '$app/navigation';
	import { createUser } from '$lib/api';
	import paths from '$lib/paths';
	import { theme, toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';

	let userUsernameFieldValue = '';
	let userUsernameFieldError = false;
	let userEmailFieldValue = '';
	let userEmailFieldError = false;

	function _create(): void {
		if (!_validate()) {
			return;
		}

		createUser(
			userUsernameFieldValue,
			userEmailFieldValue.length === 0 ? null : userEmailFieldValue,
		)
			.then((user) => {
				goto(paths.user_info.getPath({ userId: user.id }));
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

		return !userUsernameFieldError && !userEmailFieldError;
	}

	function validateUserUsernameField(): void {
		userUsernameFieldError = false;
	}

	function validateUserEmailField(): void {
		userEmailFieldError = false;
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
		</div>

		<button type="button" class="btn btn-{$theme.color}" on:click={_create}>Create User</button>
	</div>
</div>
