<script lang="ts">
    import { onMount } from 'svelte';
    import { loadCurrentUser, login, logout, showToast } from '../controller';
    import { theme } from '../stores';
    import paths from '../paths';
    import { useNavigate } from 'svelte-navigator';
    import { AlertType } from '../models/AlertType';
    import { FormGroup, Input, Label } from 'sveltestrap';
    const navigate = useNavigate();

    let usernameField: HTMLInputElement;
    let usernameFieldValue: string = '';
    let usernameFieldError: boolean = false;
    let passwordFieldValue: string = '';
    let passwordFieldError: boolean = false;

    onMount(() => {
        usernameField.focus();
    });

    function formLogin() {
        validateUsernameField();
        validatePasswordField();

        if (usernameFieldError || passwordFieldError) {
            return;
        }

        login(usernameFieldValue, passwordFieldValue)
            .then(() => {
                loadCurrentUser()
                    .then(() => {
                        navigate(paths.home.path);
                    })
                    .catch((err) => {
                        showToast(AlertType.ERROR, 'Failed fetching user', err);
                        logout();
                    });
            })
            .catch((err) => {
                showToast(AlertType.ERROR, 'Login Error', err);
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
        <Label class="text-muted">Username</Label>
    </FormGroup>
    <FormGroup floating={true}>
        <Input
            type="password"
            invalid={passwordFieldError}
            bind:value={passwordFieldValue}
            on:blur={validatePasswordField}
        />
        <Label class="text-muted">Password</Label>
    </FormGroup>

    <br />

    <button
        on:click={formLogin}
        class="btn btn-{$theme.color} px-5 fw-bold mb-3"
    >
        Sign in
    </button>
</form>
