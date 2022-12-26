<script lang="ts">
    import DashboardLayout from './layouts/Dashboard/DashboardLayout.svelte';
    import PortalLayout from './layouts/Portal/PortalLayout.svelte';
    import { getConfig } from './api';
    import { onMount } from 'svelte';
    import { config, theme, toasts } from './stores';
    import type Config from './models/Config';
    import { globalHistory, type NavigatorHistory } from 'svelte-navigator';
    import { writable } from 'svelte/store';
    import { Toast, ToastBody, ToastHeader } from 'sveltestrap';
	import WrapperGlobalHistory from './models/WrapperGlobalHistory';

    const isPortalView = writable<boolean>(
        window.location.pathname.startsWith('/auth'),
    );

    function onUrlChange() {
        let result = window.location.pathname.startsWith('/auth');
        // console.log(
        //     'onUrlChange',
        //     window.location.pathname,
        //     result,
        //     get(config) === null,
        //     get(theme).color === null,
        // );
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

    const wrapperGlobalHistory: NavigatorHistory = new WrapperGlobalHistory(
        globalHistory.location,
        globalHistory.listen,
        wrapFunction(globalHistory.navigate).execute as any,
    );

    onMount(() => {
        getConfig()
            .then((data: Config) => {
                config.set(data);
                // set default color if theme.color is null
                if (!$theme.color) {
                    $theme.color = data.defaultThemeColor;
                    $theme.dark = data.defaultThemeDark;
                }
                // reset color if switching is disabled
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

    <!-- Toast / Alerts -->
    <div class="position-fixed top-0 end-0 mt-5 me-5" style="z-index: 11">
        {#each $toasts as toast}
            <Toast class="{'toast-' + toast.type} mb-2">
                <ToastHeader>{toast.title}</ToastHeader>
                <ToastBody>{toast.message}</ToastBody>
            </Toast>
        {/each}
    </div>
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
