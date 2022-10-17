<script lang="ts">
    import {
        showToast,
        getUserById,
        updateUserPassword,
    } from '../../controller';
    import { theme, currentUser, config } from '../../stores';
    import { writable, type Writable } from 'svelte/store';

    import { onMount } from 'svelte';
    import {
        Badge,
        Card,
        CardBody,
        CardHeader,
        CardTitle,
        Col,
        FormGroup,
        Input,
        Label,
        Row,
    } from 'sveltestrap';
    import { AlertType } from '../../models/MessageType';
    import JsonViewer from '../../components/JsonViewer.svelte';
    import type User from '../../models/User';
    import {
        hasResaltPermission,
        P_ADMIN_USER,
        P_USER_PASSWORD,
    } from '../../perms';

    const PASSWORD_MIN_LENGTH: number = 8;

    // export let navigate;
    // export let location;
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
                showToast(
                    AlertType.ERROR,
                    'Failed fetching user: ' + userId,
                    err,
                );
            });
    }

    function updatePassword() {
        validatePasswordField();
        validateRepeatPasswordField();
        if (passwordFieldError || repeatPasswordFieldError) {
            return;
        }

        updateUserPassword($user.id, passwordFieldValue)
            .then(() => {
                // OK!
                passwordFieldValue = '';
                passwordFieldError = false;
                repeatPasswordFieldValue = '';
                repeatPasswordFieldError = false;
                updateData();
            })
            .catch((err) => {
                showToast(
                    AlertType.ERROR,
                    'Failed updating password for user: ' + userId,
                    err,
                );
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

{#if !$user}
    <h1>Loading...</h1>
{:else}
    <h1>
        User {$user.username}
        {#if $user.id === $currentUser.id}
            <span class="text-{$theme.color}"> (You)</span>
        {/if}
    </h1>

    <Row>
        <Col xs="12" xxl="4" class="pb-3">
            <Card class="h-100 {$theme.dark ? 'bg-dark' : ''}">
                <CardHeader>
                    <CardTitle class="mb-0">General</CardTitle>
                </CardHeader>
                <ul class="list-group list-group-flush">
                    <li
                        class="list-group-item {$theme.dark
                            ? 'bg-dark text-light'
                            : ''}"
                    >
                        <strong>ID</strong>
                        <span class="float-end">{$user.id}</span>
                    </li>
                    <li
                        class="list-group-item {$theme.dark
                            ? 'bg-dark text-light'
                            : ''}"
                    >
                        <strong>Username</strong>
                        <span class="float-end">{$user.username}</span>
                    </li>
                    <li
                        class="list-group-item {$theme.dark
                            ? 'bg-dark text-light'
                            : ''}"
                    >
                        <strong>Last Login</strong>
                        <span class="float-end">
                            {#if $user.lastLogin}
                                {$user.lastLogin}
                            {:else}
                                <em>Never</em>
                            {/if}
                        </span>
                    </li>
                    <li
                        class="list-group-item {$theme.dark
                            ? 'bg-dark text-light'
                            : ''}"
                    >
                        <strong>Email</strong>
                        <span class="float-end">
                            {#if $user.email}
                                {$user.email}
                            {:else}
                                <em>Not set</em>
                            {/if}
                        </span>
                    </li>
                    <li
                        class="list-group-item {$theme.dark
                            ? 'bg-dark text-light'
                            : ''}"
                    >
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
            </Card>
        </Col>
        {#if hasResaltPermission($currentUser.perms, P_ADMIN_USER) || ($currentUser.id === $user.id && hasResaltPermission($currentUser.perms, P_USER_PASSWORD))}
            <Col xs="12" xxl="4" class="pb-3">
                <Card class="h-100 {$theme.dark ? 'bg-dark' : ''}">
                    <CardHeader>
                        <CardTitle class="mb-0">Password</CardTitle>
                    </CardHeader>
                    <CardBody>
                        <FormGroup floating={true}>
                            <Input
                                type="password"
                                disabled={$user.ldapSync !== null}
                                invalid={passwordFieldError}
                                bind:value={passwordFieldValue}
                                on:blur={validatePasswordField}
                            />
                            <Label>New password</Label>
                        </FormGroup>
                        <FormGroup floating={true}>
                            <Input
                                type="password"
                                disabled={$user.ldapSync !== null}
                                invalid={repeatPasswordFieldError &&
                                    repeatPasswordFieldValue.length > 0}
                                bind:value={repeatPasswordFieldValue}
                                on:keyup={validateRepeatPasswordField}
                            />
                            <Label>Confirm password</Label>
                        </FormGroup>
                        {#if $user.ldapSync !== null}
                            <p class="text-muted mt-3">
                                This user is synced with LDAP. Passwords can
                                only be changed in LDAP.
                            </p>
                        {/if}
                        {#if passwordFieldError}
                            <p class="text-danger mt-3">
                                Password must be at least {PASSWORD_MIN_LENGTH} characters
                                long.
                            </p>
                        {/if}
                        {#if repeatPasswordFieldError}
                            <p class="text-danger mt-3">
                                Passwords do not match.
                            </p>
                        {/if}
                        <button
                            disabled={$user.ldapSync !== null}
                            class="btn btn-{$theme.color}"
                            on:click={updatePassword}>Update</button
                        >
                    </CardBody>
                </Card>
            </Col>
        {/if}
        <Col xs="12" xxl="4" class="pb-3">
            <Card class="h-100 {$theme.dark ? 'bg-dark' : ''}">
                <CardHeader>
                    <CardTitle class="mb-0">Permissions</CardTitle>
                </CardHeader>
                <JsonViewer data={$user.perms} />
            </Card>
        </Col>
    </Row>
{/if}
