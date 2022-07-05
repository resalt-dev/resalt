<script>
    import { onMount } from "svelte";
    import { login } from "../controller";
    import { theme } from "../stores";

    import { useNavigate } from "svelte-navigator";
    const navigate = useNavigate();

    let username = "";
    let password = "";
    let usernameField;

    onMount(() => {
        usernameField.focus();
    });

    function handleClick() {
        login(navigate, username, password);
    }
</script>

<p class="fw-bold">
    This is a restricted admin area. Please login to continue.
</p>

<br />

<form onsubmit="return false;" autocomplete="false">
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
