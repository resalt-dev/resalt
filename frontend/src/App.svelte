<script lang="ts">
    import { Router, Route } from "svelte-navigator";
    import constants from "./constants";
    import paths from "./paths";
    import DashboardLayout from "./layouts/Dashboard/DashboardLayout.svelte";
    import PortalLayout from "./layouts/Portal/PortalLayout.svelte";
    import Redirect from "./components/Redirect.svelte";
    import SSEConnector from "./components/SSEConnector.svelte";
    import { loadConfig } from "./controller";
    import { onMount } from "svelte";
    import { config } from "./stores";

    // check if URL starts with basePath, if not then redirect
    const basePath = constants.basePath;
    const path = window.location.pathname;
    if (!path.startsWith(basePath)) {
        window.location.href = basePath;
    }

    onMount(() => {
        loadConfig()
            .then((data) => {})
            .catch((err) => {
                console.error(err);
                alert("Critical API error");
            });
    });
</script>

<main>
    {#if $config === null}
        <p>Loading...</p>
    {:else}
        <Router basepath={constants.basePath} primary={false}>
            <SSEConnector />
            <Route path="auth/*">
                <PortalLayout />
            </Route>
            <Route path="dashboard/*">
                <DashboardLayout />
            </Route>
            <Route path="*">
                <Redirect to={paths.home.path} />
            </Route>
        </Router>
    {/if}
</main>

<style>
    main {
        width: 100vw;
        height: 100vh;
        max-width: 100vw;
        max-height: 100vh;
        overflow-x: hidden;
        overflow-y: hidden;
    }
</style>
