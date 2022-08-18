<script lang="ts">
    import { Link, useLocation } from "svelte-navigator";
    import { theme } from "../../stores";
    import Icon from "../../components/Icon.svelte";

    export let route;
    export let collapsed;

    const location = useLocation();
    // $: isActive = $location.pathname === route.path;
    $: isActiveOrSub = $location.pathname.startsWith(route.path);
</script>

<li class="nav-item">
    <Link
        to={route.path}
        class="nav-link {$theme.color === 'yellow' && isActiveOrSub
            ? 'text-dark'
            : $theme.dark && !isActiveOrSub
            ? 'text-light'
            : 'text-white'} py-3 fw-light d-flex align-items-center {isActiveOrSub
            ? `bg-${$theme.color}`
            : ''}"
    >
        <Icon name={route.icon} class="ps-1 {collapsed ? '' : 'me-3'}" />
        {#if !collapsed}
            <span>{route.label}</span>
        {/if}
    </Link>
</li>
