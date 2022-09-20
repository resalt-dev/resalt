<script lang="ts">
    import { Link, useLocation } from "svelte-navigator";
    import { theme } from "../../stores";
    import Icon from "../../components/Icon.svelte";
    import type { Path } from "../../paths";

    export let route: Path;
    export let collapsed: boolean;

    const location = useLocation();
    // $: isActive = $location.pathname === route.path;
    $: isActiveOrSub = $location.pathname.startsWith(route.path);
</script>

<li class="nav-item" style="height: 4.5rem;">
    <Link
        to={route.path}
        class="nav-link {$theme.color === 'yellow' && isActiveOrSub
            ? 'text-dark'
            : $theme.dark && !isActiveOrSub
            ? 'text-light'
            : 'text-white'} fw-light d-flex align-items-center {isActiveOrSub
            ? `bg-${$theme.color}`
            : ''}"
        style="height: inherit;"
    >
        <Icon name={route.icon} class="ps-1 {collapsed ? '' : 'me-3'}" />
        {#if !collapsed}
            <span>{route.label}</span>
        {/if}
    </Link>
</li>
