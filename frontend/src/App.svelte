<script lang="ts">
    import { Router, Route } from "svelte-navigator";
    import constants from "./constants";
    import paths from "./paths";
    import DashboardLayout from "./layouts/DashboardLayout.svelte";
    import DialogLayout from "./layouts/DialogLayout.svelte";
    import Redirect from "./components/Redirect.svelte";
    import SSEConnector from "./components/SSEConnector.svelte";

    // check if URL starts with basePath, if not then redirect
    const basePath = constants.basePath;
    const path = window.location.pathname;
    if (!path.startsWith(basePath)) {
        window.location.href = basePath;
    }
</script>

<main>
    <Router basepath={constants.basePath} primary={false}>
        <SSEConnector />
        <Route path="auth/*">
            <DialogLayout />
        </Route>
        <Route path="dashboard/*">
            <DashboardLayout />
        </Route>
        <Route path="*">
            <Redirect to={paths.home.path} />
        </Route>
    </Router>
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
