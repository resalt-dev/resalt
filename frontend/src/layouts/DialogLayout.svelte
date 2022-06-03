<script>
    import { Router, Route } from "svelte-navigator";
    import constants from "../constants";
    import { alerts, theme } from "../stores";
    import AuthLogin from "../views/AuthLogin.svelte";
    import AuthLogout from "../views/AuthLogout.svelte";

    let localAlerts = [];

    // Clone "alerts" to "localAlerts" then empty it on every Svelte update
    $: {
        localAlerts = $alerts.slice();
        alerts.update(() => []);
    }
</script>

<div
    class="background1 h-100"
    style="background-image: url('{constants.basePath}/assets/images/denisse-leon-OVEWbIgffDk-unsplash.jpg')"
>
    <div
        class="background2 h-100 w-100"
        style={$theme.dark
            ? "background: rgba(0, 0, 0, 0.25);"
            : "background: rgba(255, 255, 255, 0.25);"}
    >
        <!-- Title -->
        <div class="row g-0">
            <div class="col-9 offset-2 px-5" style="padding-top: 30rem;">
                <h1 class="text-uppercase text-white">
                    {constants.appName} Admin Console
                </h1>
            </div>
        </div>
        <!-- Content -->
        <div class="row g-0">
            <div class="col-2 p-5 {$theme.dark ? 'bg-darker' : 'bg-light'}" />
            <div
                class="col-6 p-5 {$theme.dark
                    ? 'bg-dark text-white'
                    : 'bg-white text-dark'}"
                style="max-width: 54rem;"
            >
                <Router primary={false}>
                    <Route path="login" component={AuthLogin} />
                    <Route path="logout" component={AuthLogout} />
                </Router>

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
            </div>
        </div>
    </div>
</div>

<style>
    .background1 {
        background: var(--dark);
        background-size: cover;
        background-position: center;
        background-repeat: no-repeat;
        background-attachment: fixed;
    }
    .background2 {
        /* background: rgba(255, 255, 255, 0.25); */
    }
</style>
