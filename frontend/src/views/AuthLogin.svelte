<script lang="ts">
    import { onMount } from "svelte";
    import {
        AlertType,
        loadCurrentUser,
        login,
        logout,
        showAlert,
    } from "../controller";
    import { theme } from "../stores";
    import paths from "../paths";
    import { useNavigate } from "svelte-navigator";
    const navigate = useNavigate();

    let username = "";
    let password = "";
    let usernameField;

    onMount(() => {
        usernameField.focus();
    });

    function handleClick() {
        login(username, password)
            .then(() => {
                loadCurrentUser()
                    .then(() => {
                        navigate(paths.home.path);
                    })
                    .catch((err) => {
                        showAlert(AlertType.ERROR, "Failed fetching user", err);
                        logout();
                    });
            })
            .catch((err) => {
                showAlert(AlertType.ERROR, "Login Error", err);
            });
    }
</script>

<p class="fw-bold">
    This is a restricted admin area. Please login to continue.
</p>

<br />

<form action="javascript:void(0);" autocomplete="false">
    <input
        bind:value={username}
        type="text"
        class="form-control mb-3 {$theme.dark &&
            'bg-secondary border-0 text-light'}"
        id="loginInputUsername"
        placeholder="Username"
        bind:this={usernameField}
    />
    <input
        bind:value={password}
        type="password"
        class="form-control mb-3 {$theme.dark &&
            'bg-secondary border-0 text-light'}"
        id="loginInputPassword"
        placeholder="Password"
    />

    <br />

    <button
        on:click={handleClick}
        class={`btn btn-${$theme.color} float-center px-5 fw-bold mb-3`}
        >Login</button
    >
</form>
