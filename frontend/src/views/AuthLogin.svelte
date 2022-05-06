<script>
    import { onMount } from "svelte";
    import { login } from "../controller";

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

<form onsubmit="return false;" autocomplete="false">
    <div class="row mb-3">
        <label for="loginInputUsername" class="col-sm-3 col-form-label fw-bold"
            >Username or email</label
        >
        <div class="col-sm-9">
            <input
                bind:value={username}
                type="text"
                class="form-control"
                id="loginInputUsername"
                bind:this={usernameField}
            />
        </div>
    </div>
    <div class="row mb-3">
        <label for="loginInputPassword" class="col-sm-3 col-form-label fw-bold">
            Password
        </label>
        <div class="col-sm-9">
            <input
                bind:value={password}
                type="password"
                class="form-control"
                id="loginInputPassword"
            />
        </div>
    </div>
    <button on:click={handleClick} class="btn btn-gold float-end px-5"
        >Login</button
    >
</form>
