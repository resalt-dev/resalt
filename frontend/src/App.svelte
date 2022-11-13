<script lang="ts">
    import type RawLocation from 'svelte-navigator/types/RawLocation';
    import DashboardLayout from './layouts/Dashboard/DashboardLayout.svelte';
    import PortalLayout from './layouts/Portal/PortalLayout.svelte';
    import { getConfig } from './api';
    import { onMount } from 'svelte';
    import { config, theme } from './stores';
    import type Config from './models/Config';
    import { globalHistory, type NavigatorHistory } from 'svelte-navigator';
    import { get, writable } from 'svelte/store';

    const isPortalView = writable<boolean>(
        window.location.pathname.startsWith('/auth'),
    );

    function onUrlChange() {
        let result = window.location.pathname.startsWith('/auth');
        console.log(
            'onUrlChange',
            window.location.pathname,
            result,
            get(config) === null,
            get(theme).color === null,
        );
        if (result !== $isPortalView) {
            isPortalView.set(result);
        }
    }

    // Wrap globalHistory, pass forward all function calls, except also call onUrlChange() on every change.
    function wrapFunction<A extends any[], R>(someFunction: (...a: A) => R) {
        const wrappedFunction = function (...args: A) {
            let result = someFunction(...args);
            onUrlChange();
            return result;
        };
        return { execute: wrappedFunction };
    }
    class WrapperGlobalHistory implements NavigatorHistory {
        readonly location: RawLocation = globalHistory.location;
        listen = globalHistory.listen;
        navigate: any = wrapFunction(globalHistory.navigate).execute;
    }
    const wrapperGlobalHistory: NavigatorHistory = new WrapperGlobalHistory();

    onMount(() => {
        getConfig()
            .then((data: Config) => {
                config.set(data);
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
        <p>Loading....</p>
    {:else if $isPortalView}
        <PortalLayout history={wrapperGlobalHistory} />
    {:else}
        <DashboardLayout history={wrapperGlobalHistory} />
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
