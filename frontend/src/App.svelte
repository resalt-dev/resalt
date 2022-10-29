<script lang="ts">
    import type RawLocation from 'svelte-navigator/types/RawLocation';
    import DashboardLayout from './layouts/Dashboard/DashboardLayout.svelte';
    import PortalLayout from './layouts/Portal/PortalLayout.svelte';
    import { loadConfig } from './controller';
    import { onMount } from 'svelte';
    import { config, theme } from './stores';
    import type Config from './models/Config';
    import { globalHistory, type NavigatorHistory } from 'svelte-navigator';
    import { writable } from 'svelte/store';

    const isPortalView = writable<boolean>(
        window.location.pathname.startsWith('/auth'),
    );

    function onUrlChange() {
        let result = window.location.pathname.startsWith('/auth');
        if (result !== $isPortalView) {
            isPortalView.set(result);
        }
    }

    // Wrap globalHistory, pass forward all function calls, except also call onUrlChange() on every change.
    function wrapFunction<A extends any[], R>(someFunction: (...a: A) => R) {
        const wrappedFunction = function (...args: A) {
            let result = someFunction(...args);
            onUrlChange();
            console.log('foorack-wrapFunction', result);
            return result;
        };
        return { execute: wrappedFunction };
    }

    class WrapperGlobalHistory implements NavigatorHistory {
        readonly location: RawLocation = globalHistory.location;
        listen = globalHistory.listen;
        /*
        type NavigateFn<State extends AnyObject = AnyObject> = {
            (to: string, options?: NavigateOptions<State>): void;
            (delta: number): void;
        };
        Implement the above type as a function:
        */
        navigate: any = wrapFunction(globalHistory.navigate).execute;
    }
    const wrapperGlobalHistory: NavigatorHistory = new WrapperGlobalHistory();

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
