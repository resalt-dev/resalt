<script lang="ts">
    import { onMount } from 'svelte';
    import { login } from '../../api';
    import { auth, config, theme, toasts } from '../../stores';
    import paths from '../../paths';
    import { MessageType } from '../../models/MessageType';
    import { FormGroup, Input, Label } from 'sveltestrap';
    import type { NavigateFn } from 'svelte-navigator';
    import ResaltProgress from '../../components/ResaltProgress.svelte';

    // svelte-ignore unused-export-let
    export let location: Location;
    export let navigate: NavigateFn;

    let usernameField: HTMLInputElement;
    let usernameFieldValue: string = '';
    let usernameFieldError: boolean = false;
    let passwordFieldValue: string = '';
    let passwordFieldError: boolean = false;

    onMount(() => {
        if ($config.authForwardEnabled) {
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
                // Redirect is done automatic by router, once auth is set.
            })
            .catch((err) => {
                toasts.add(MessageType.ERROR, 'Login Error', err);
            });
    }

    /*
    // VALIDATION
    */

    function validateUsernameField(): void {
        usernameFieldError = false;
    }

    function validatePasswordField(): void {
        passwordFieldError = false;
    }
</script>

{#if $config.authForwardEnabled}
    <p class="fw-bold">
        SSO (Single Sign On) enabled. Please wait while authenticating...
    </p>

    <ResaltProgress />
{:else}
    <p class="fw-bold">
        This is a restricted admin area. Please log in to continue.
    </p>
    <br />
    <form action="javascript:void(0);" autocomplete="false">
        <FormGroup floating={true}>
            <Input
                type="text"
                invalid={usernameFieldError}
                bind:value={usernameFieldValue}
                on:blur={validateUsernameField}
                bind:inner={usernameField}
            />
            <Label>Username</Label>
        </FormGroup>
        <FormGroup floating={true}>
            <Input
                type="password"
                invalid={passwordFieldError}
                bind:value={passwordFieldValue}
                on:blur={validatePasswordField}
            />
            <Label>Password</Label>
        </FormGroup>

        <br />

        <button
            on:click={formLogin}
            class="btn btn-{$theme.color} px-5 fw-bold mb-3"
        >
            Sign in
        </button>
    </form>
{/if}
