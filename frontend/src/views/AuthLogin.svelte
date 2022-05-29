<script>
    import { onMount } from "svelte";
    import { login } from "../controller";
    import { theme, alerts } from "../stores";

    import { useNavigate } from "svelte-navigator";
    const navigate = useNavigate();

    let username = "";
    let password = "";
    let usernameField;

    let localAlerts = [];

    // Clone "alerts" to "localAlerts" then empty it on every Svelte update
    $: {
        localAlerts = $alerts.slice();
        alerts.update(() => []);
    }

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
            >Username</label
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
    <button
        on:click={handleClick}
        class={`btn btn-${$theme.color} float-end px-5 fw-bold mb-3`}
        >Login</button
    >
    <div class="clearfix" />

    {#each localAlerts as alert}
        <div class="card text-white bg-{alert.type} mb-3">
            <div class="card-body">
                <h5 class="card-title">
                    {alert.title}
                </h5>
                <p class="card-text">
                    {alert.message}
                </p>
            </div>
        </div>
    {/each}
</form>
