<script lang="ts">
    import { Router, Route } from 'svelte-navigator';
    import constants from './constants';
    import paths from './paths';
    import DashboardLayout from './layouts/Dashboard/DashboardLayout.svelte';
    import PortalLayout from './layouts/Portal/PortalLayout.svelte';
    import Redirect from './components/Redirect.svelte';
    import { loadConfig } from './controller';
    import { onMount } from 'svelte';
    import { config, theme } from './stores';
    import type Config from './models/Config';

    // check if URL starts with basePath, if not then redirect
    const basePath = constants.basePath;
    const path = window.location.pathname;
    if (!path.startsWith(basePath)) {
        window.location.href = basePath;
    }

    onMount(() => {
        loadConfig()
            .then((data: Config) => {
                // set default color if theme.color is null
                if (!$theme.color) {
                    $theme.color = data.defaultThemeColor;
                }
                // reset theme is switching is disabled
                if (!data.enableThemeSwitching) {
                    $theme.color = data.defaultThemeColor;
                }
            })
            .catch((err) => {
                console.error(err);
                alert('Critical API error');
            });
    });
</script>

<main class={$theme.dark ? 'theme-dark' : ''}>
    {#if $config === null || $theme.color === null}
        <p>Loading...</p>
    {:else}
        <Router basepath={constants.basePath} primary={false}>
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

<!-- svelte-ignore css-unused-selector -->
<style lang="scss" global>
    @use './styles/global/_global.scss';

    main {
        width: 100vw;
        height: 100vh;
        max-width: 100vw;
        max-height: 100vh;
        overflow-x: hidden;
        overflow-y: hidden;
    }
</style>
