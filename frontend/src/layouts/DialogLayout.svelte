<script>
    import { Router, Route } from "svelte-navigator";
    import Logo from "../components/Logo.svelte";
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
    style="background-image: url('{constants.basePath}/assets/images/0da7530ac9cd4c88850c62138da12e66.jpg');"
>
    <div class="background2 h-100 w-100">
        <div class="row g-0 h-100">
            <!-- Right side -->
            <div
                class="offset-8 col-4 h-100 {$theme.dark
                    ? 'bg-dark'
                    : 'bg-white'}"
            >
                <div
                    class="row h-100 g-0 justify-content-center align-items-center"
                >
                    <div
                        class="col-12 {$theme.dark
                            ? 'bg-dark text-light'
                            : 'bg-white'}"
                    >
                        <!-- Title -->
                        <div class="m-3 px-5 py-3">
                            <Logo color={$theme.color} />
                        </div>

                        <hr class="mx-3 my-3" />

                        <!-- Content -->
                        <div class="px-5 py-4" style="max-width: 54rem;">
                            <Router primary={false}>
                                <Route path="login" component={AuthLogin} />
                                <Route path="logout" component={AuthLogout} />
                            </Router>

                            <div class="clearfix" />

                            {#each localAlerts as alert}
                                <div
                                    class="card text-light bg-{alert.type} mb-3"
                                >
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
