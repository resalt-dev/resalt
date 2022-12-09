<script lang="ts">
    import { Link, useLocation } from 'svelte-navigator';
    import { theme } from '../../stores';
    import Icon from '../../components/Icon.svelte';
    import type { Path } from '../../paths';

    export let path: Path;
    export let collapsed: boolean;

    const location = useLocation();
    $: slugPath = path.getPath().split('/')[1] ?? '';
    $: slugLocation = $location.pathname.split('/')[1];
    //$: console.log('path', slugPath, slugLocation);
    $: isActive = slugPath.startsWith(slugLocation);
</script>

<li class="nav-item" style="height: 4.5rem;">
    <Link
        to={path.getPath()}
        class="nav-link {$theme.color === 'yellow' && isActive
            ? 'text-dark'
            : 'text-white'} {isActive
            ? ''
            : 'fw-light'} d-flex align-items-center {isActive
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
