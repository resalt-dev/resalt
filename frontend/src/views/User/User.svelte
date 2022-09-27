<script lang="ts">
    import { showAlert, getUserById } from '../../controller';
    import { theme, currentUser, config } from '../../stores';
    import { writable, type Writable } from 'svelte/store';

    import { onMount } from 'svelte';
    import {
        Badge,
        Card,
        CardBody,
        CardHeader,
        CardSubtitle,
        CardTitle,
        Col,
        Input,
        Row,
    } from 'sveltestrap';
    import { AlertType } from '../../models/AlertType';
    import JsonViewer from '../../components/JsonViewer.svelte';
    import type User from '../../models/User';

    // export let navigate;
    // export let location;
    export let userId: string;

    let password: string = '';
    let passwordRepeat: string = '';

    const user: Writable<User | null> = writable(null);

    function handleUpdatePassword() {
        if (password !== passwordRepeat) {
            showAlert(
                AlertType.ERROR,
                'Password mismatch',
                'Passwords do not match, please verify and try again.',
            );
            return;
        }

        // TODO: Update password
    }

    onMount(() => {
        getUserById(userId)
            .then((data) => {
                user.set(data);
            })
            .catch((err) => {
                showAlert(
                    AlertType.ERROR,
                    'Failed fetching user: ' + userId,
                    err,
                );
            });
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
                        <strong>Has Password</strong>
                        <span class="float-end">
                            {#if $user.hasPassword}
                                <Badge
                                    color={$theme.dark ? 'secondary' : 'dark'}
                                    >Yes</Badge
                                >
                            {:else}
                                <Badge color={null} class="bg-{$theme.color}"
                                    >No</Badge
                                >
                            {/if}
                        </span>
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
        <Col xs="12" xxl="4" class="pb-3">
            <Card class="h-100 {$theme.dark ? 'bg-dark' : ''}">
                <CardHeader>
                    <CardTitle class="mb-0">Password</CardTitle>
                </CardHeader>
                <CardBody>
                    <CardSubtitle class="mb-3">New password:</CardSubtitle>
                    <Input
                        bind:value={password}
                        id="userNewPassword"
                        type="password"
                        placeholder="New password"
                        class="form-control mb-3 d-inline"
                        style="width: 20rem;"
                        disabled
                    />
                    <CardSubtitle class="mb-3">Confirm password:</CardSubtitle>
                    <Input
                        bind:value={passwordRepeat}
                        id="userNewPasswordConfirm"
                        type="password"
                        placeholder="Confirm password"
                        class="form-control mb-3 d-inline"
                        style="width: 20rem;"
                        disabled
                    />
                    <br />
                    <button
                        disabled
                        class="btn btn-{$theme.color}"
                        on:click={handleUpdatePassword}>Update</button
                    > Not yet implemented.
                </CardBody>
            </Card>
        </Col>
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
