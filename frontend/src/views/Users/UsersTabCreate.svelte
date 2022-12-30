<script lang="ts">
	import type { NavigateFn } from 'svelte-navigator';
	import {
		Button,
		Card,
		CardBody,
		CardHeader,
		Col,
		FormGroup,
		Input,
		Label,
		Row,
	} from 'sveltestrap';
	import { createUser } from '../../api';
	import { MessageType } from '../../models/MessageType';
	import paths from '../../paths';
	import { theme, toasts } from '../../stores';

	export let navigate: NavigateFn;

	// Depending on if creating LOCAL or LDAP user, different fields are shown.
	// Email is only shown if creating LOCAL user, as otherwise synced from LDAP.
	// LDAP Sync is only shown if creating LDAP user.

	let userLDAPFieldValue: boolean = false;
	let userUsernameFieldValue: string = '';
	let userUsernameFieldError: boolean = false;
	let userEmailFieldValue: string = '';
	let userEmailFieldError: boolean = false;
	let userLDAPSyncFieldValue: string = '';
	let userLDAPSyncFieldError: boolean = false;

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
				navigate(paths.user.getPath(user.id));
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

<Card class="mb-3">
	<CardHeader>Create User</CardHeader>
	<CardBody>
		<Row>
			<Col class="mb-0" md="5" lg="2">
				<FormGroup floating={true}>
					<Input
						id="userUsername"
						type="text"
						invalid={userUsernameFieldError}
						bind:value={userUsernameFieldValue}
						on:blur={validateUserUsernameField}
					/>
					<Label for="userUsername">Username</Label>
				</FormGroup>
			</Col>
			<Col class="mb-0" md="2" lg="1">
				<div class="d-flex justify-content-center">
					<Label for="userLDAP" class="mb-0 me-2">LDAP</Label>
				</div>
				<div class="clearfix" />
				<div class="d-flex justify-content-center">
					<FormGroup floating={true} class="form-switch ps-0">
						<Input
							id="userLDAP"
							type="switch"
							class="fs-3 mt-0"
							bind:checked={userLDAPFieldValue}
							on:blur={validateUserLDAPField}
						/>
					</FormGroup>
				</div>
			</Col>
			{#if userLDAPFieldValue}
				<Col class="mb-0" md="5" lg="5">
					<FormGroup floating={true}>
						<Input
							id="userLDAPSync"
							type="text"
							invalid={userLDAPSyncFieldError}
							bind:value={userLDAPSyncFieldValue}
							on:blur={validateUserLDAPSyncField}
						/>
						<Label for="userLDAPSync">LDAP Sync DN</Label>
					</FormGroup>
				</Col>
			{:else}
				<Col class="mb-0" md="5" lg="3">
					<FormGroup floating={true}>
						<Input
							id="userEmail"
							type="email"
							invalid={userEmailFieldError}
							bind:value={userEmailFieldValue}
							on:blur={validateUserEmailField}
						/>
						<Label for="userEmail">Email (optional)</Label>
					</FormGroup>
				</Col>
			{/if}
		</Row>

		<Button color={null} class="btn-{$theme.color}" on:click={_create}>Create User</Button>
	</CardBody>
</Card>
