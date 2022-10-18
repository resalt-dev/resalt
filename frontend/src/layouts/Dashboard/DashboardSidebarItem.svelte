<script lang="ts">
    import { Link, useLocation } from 'svelte-navigator';
    import { theme } from '../../stores';
    import Icon from '../../components/Icon.svelte';
    import type { Path } from '../../paths';

    export let path: Path;
    export let collapsed: boolean;

    const location = useLocation();
    // $: isActive = $location.pathname === route.path;
    $: isActiveOrSub = $location.pathname.startsWith(path.path);
</script>

<li class="nav-item" style="height: 4.5rem;">
    <Link
        to={path.path}
        class="nav-link {$theme.color === 'yellow' && isActiveOrSub
            ? 'text-dark'
            : 'text-white'} {isActiveOrSub
            ? ''
            : 'fw-light'} d-flex align-items-center {isActiveOrSub
            ? `bg-${$theme.color}`
            : ''}"
        style="height: inherit;"
    >
        <Icon name={path.icon} class="ps-1 {collapsed ? '' : 'me-3'}" />
        {#if !collapsed}
            <span>{path.label}</span>
        {/if}
    </Link>
</li>
