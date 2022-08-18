<script lang="ts">
    import { Link } from "svelte-navigator";
    import paths from "../../paths";
    import {
        sidebarCollapsed as collapsed,
        theme,
        currentUser,
    } from "../../stores";
    import Icon from "../../components/../components/Icon.svelte";
    import Logo from "../../components/../components/Logo.svelte";
    import SidebarItem from "./DashboardSidebarItem.svelte";

    function handleClickCollapse() {
        collapsed.update((n) => !n);
    }
</script>

<div
    class="d-flex flex-column flex-shrink-0 bg-dark h-100 no-select"
    style="min-height: 100vh; {$collapsed
        ? 'width: 4.5rem;'
        : 'width: 17.5rem;'}"
>
    <div
        on:click={handleClickCollapse}
        class="d-flex text-decoration-none mouse-pointer"
    >
        <div
            class="d-flex align-items-center py-4 {$collapsed
                ? 'mx-auto'
                : 'w-100'}"
            style="height: 80px"
        >
            <div class="px-5 py-3 {$collapsed ? 'd-none' : 'w-100'}">
                <Logo color={$theme.color} />
            </div>
            <Icon
                name="menu"
                class="mb-0 h3 {$theme.dark
                    ? 'text-light'
                    : 'text-white'} {!$collapsed && 'd-none'}"
            />
        </div>
    </div>

    <hr class="mt-0 mb-3" />

    <ul
        class="nav nav-pills flex-column mb-auto fs-5 {$collapsed
            ? 'nav-flush text-center'
            : 'mx-2'}"
    >
        {#each paths as route}
            {#if route.showInNav}
                {#if route.name === "users"}
                    <li><hr /></li>
                {/if}
                <SidebarItem {route} collapsed={$collapsed} />
            {/if}
        {/each}
    </ul>

    <hr class="mb-0" />

    <div
        on:click={handleClickCollapse}
        class="{$theme.dark
            ? 'text-light'
            : 'text-white'} btn-dark bg-dark border-0 pt-3 pb-3 px-3 fw-light mouse-pointer d-flex align-items-center"
        style="padding-top: 2px;"
        aria-current="page"
    >
        <Icon
            name={$collapsed ? "right-arrow-alt" : "left-arrow-alt"}
            class={$collapsed ? "" : "me-3"}
            size="2.5"
        />
        {#if !$collapsed}
            <span class="fs-5">Collapse</span>
        {/if}
    </div>

    <hr class="mt-0 mb-0" />

    {#if $collapsed}
        <div class="text-center text-secondary">0.0.x</div>
    {:else}
        <span class="text-center text-secondary">Resalt - 0.0.x</span>
    {/if}
</div>
