<script>
    import { Link } from "svelte-navigator";
    import constants from "../../constants";
    import paths from "../../paths";
    import { sidebarCollapsed as collapsed, theme } from "../../stores";
    import Icon from "../Icon.svelte";
    import SidebarItem from "./SidebarItem.svelte";

    function handleClickCollapse() {
        collapsed.update((n) => !n);
    }
</script>

<div
    class="d-flex flex-column flex-shrink-0 text-white bg-dark h-100 no-select"
    style="min-height: 100vh; {$collapsed
        ? 'width: 4.5rem;'
        : 'width: 17.5rem;'}"
>
    <div
        on:click={handleClickCollapse}
        class="d-flex text-decoration-none mouse-pointer"
    >
        <div class="d-flex align-items-center py-4 mx-auto text-white">
            <Icon name="menu" class="mb-0 h3 {$collapsed ? '' : 'me-3'}" />
            {#if !$collapsed}
                <span class="fs-4 fw-bold ln-0">{constants.appName}</span>
            {/if}
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
                    <hr />
                {/if}
                <SidebarItem {route} collapsed={$collapsed} />
            {/if}
        {/each}
    </ul>

    <hr />

    <div class="dropdown">
        <!-- svelte-ignore a11y-invalid-attribute -->
        <a
            href="#"
            class="d-flex align-items-center text-white text-decoration-none dropdown-toggle px-3"
            id="dropdownUser1"
            data-bs-toggle="dropdown"
            aria-expanded="false"
        >
            <img
                src="https://github.com/Foorack.png"
                alt=""
                class="rounded-circle {$collapsed ? '' : 'me-3'}"
                width="32"
                height="32"
            />
            {#if !$collapsed}
                <strong class="me-1">Max</strong>
            {/if}
        </a>
        <ul
            class="dropdown-menu dropdown-menu-dark bg-darker text-small shadow ms-5"
            aria-labelledby="dropdownUser1"
        >
            <li>
                <Link to={paths.preferences.path} class="dropdown-item"
                    >Preferences</Link
                >
            </li>
            <li><hr class="dropdown-divider" /></li>
            <li>
                <Link to={paths.logout.path} class="dropdown-item"
                    >Sign out</Link
                >
            </li>
        </ul>
    </div>

    <hr class="mb-0" />

    <div
        on:click={handleClickCollapse}
        class="text-white btn-dark bg-dark border-0 pt-3 pb-2 px-3 fw-light mouse-pointer d-flex align-items-center"
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
</div>

<style>
    .ln-0 {
        line-height: 0;
    }
</style>
