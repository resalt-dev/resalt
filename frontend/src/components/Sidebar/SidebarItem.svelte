<script>
    import { Link, useLocation } from "svelte-navigator";
    import { theme } from "../../stores";
    import Icon from "../Icon.svelte";

    export let route;
    export let collapsed;

    const location = useLocation();
    // $: isActive = $location.pathname === route.path;
    $: isActiveOrSub = $location.pathname.startsWith(route.path);
</script>

<li class="nav-item">
    <Link
        to={route.path}
        class="nav-link text-white py-3 fw-light d-flex align-items-center {isActiveOrSub
            ? `bg-${$theme.color} fw-bold`
            : ''}"
    >
        <Icon name={route.icon} class="ps-1 {collapsed ? '' : 'me-3'}" />
        {#if !collapsed}
            <span>{route.label}</span>
        {/if}
    </Link>
</li>
