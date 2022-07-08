<script lang="ts">
    import { Link } from "svelte-navigator";
    import paths from "../../paths";
    import { sidebarCollapsed as collapsed, theme, user } from "../../stores";
    import Icon from "../Icon.svelte";
    import Logo from "../Logo.svelte";
    import SidebarItem from "./SidebarItem.svelte";

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

    <hr />

    <div class="dropdown">
        <!-- svelte-ignore a11y-invalid-attribute -->
        <a
            href="#"
            class="d-flex align-items-center {$theme.dark
                ? 'text-light'
                : 'text-white'} text-decoration-none dropdown-toggle px-3"
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
                style="margin-left: -4px;margin-right: 4px;"
            />
            {#if !$collapsed}
                <strong class="me-1">{$user.username}</strong>
            {/if}
        </a>
        <ul
            class="dropdown-menu dropdown-menu-dark bg-darker ms-5"
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
        class="{$theme.dark
            ? 'text-light'
            : 'text-white'} btn-dark bg-dark border-0 pt-3 pb-2 px-3 fw-light mouse-pointer d-flex align-items-center"
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
