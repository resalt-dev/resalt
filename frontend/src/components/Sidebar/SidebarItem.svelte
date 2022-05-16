<script>
    import { Link, useLocation } from "svelte-navigator";
    import Icon from "../Icon.svelte";

    export let route;
    export let collapsed;

    const location = useLocation();
    $: isActive = $location.pathname === route.path;
</script>

<li class="nav-item">
    <Link
        to={route.path}
        class="nav-link text-white py-3 fw-light {isActive
            ? 'active fw-bold'
            : ''}"
    >
        <Icon name={route.icon} class={collapsed ? "" : "me-3"} />
        {#if !collapsed}
            <span class="sidebar-item-text">{route.label}</span>
        {/if}
    </Link>
</li>

<style>
    .sidebar-item-text {
        top: 0.15rem;
        position: relative;
    }
</style>
