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
    class="nav-link text-white py-2 fw-light {isActive ? 'active' : ''}"
  >
    <Icon name={route.icon} class="mb-1 {collapsed ? '' : 'me-3'}" />
    {#if !collapsed}
      <span class="">{route.label}</span>
    {/if}
  </Link>
</li>