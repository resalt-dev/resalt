<script lang="ts">
    import { useLocation, Link } from "svelte-navigator";
    import { currentUser, socket, theme } from "../../stores";
    import paths from "../../paths";
    import { Col } from "sveltestrap";
    import Icon from "../../components/Icon.svelte";

    const location = useLocation();

    $: navbar = $location.pathname
        .split("/")
        .filter(Boolean)
        .filter((path) => path !== "home")
        .map((str) => {
            return {
                title: str.charAt(0).toUpperCase() + str.slice(1),
                path: paths[
                    str === "dashboard" ? "home" : str.toLowerCase()
                ]?.getPath(),
            };
        });
</script>

<div
    class="row g-0 d-flex align-items-center {$theme.dark
        ? 'bg-secondary'
        : 'bg-light'}"
>
    <div class="col">
        <div class="btn-group me-3" role="group">
            {#each navbar as item}
                {#if item.path}
                    <Link
                        to={item.path}
                        class={`btn btn-${$theme.color} btn-arrow-right fw-bold`}
                        >{item.title}</Link
                    >
                {:else}
                    <div class="btn btn-dark btn-arrow-right fw-bold">
                        {item.title}
                    </div>
                {/if}
            {/each}
        </div>
    </div>
    <div class="col-auto pe-3 d-flex align-items-center">
        {#if $socket.connected}
            <!-- display last_ping as hh:mm:ss -->
            <span class="font-monospace pt-1"
                >Connected: {new Date($socket.last_ping)
                    .toLocaleTimeString("en-US", {
                        timeZone: "UTC",
                        timeZoneName: "short",
                        hour12: false,
                    })
                    .replace(/\./g, ":")}</span
            >
        {:else}
            <span class="font-monospace pt-1 text-danger">Disconnected</span>
        {/if}
    </div>
    <Col xs="auto">
        <div class="vr" style="height: 1.5rem;background: rgba(0, 0, 0, 0.2)" />
    </Col>
    <Col xs="auto" class="px-3 text-reset text-decoration-none">
        <Icon name="user" size="1.5" type="solid" class="pe-1" />
        {$currentUser.username}
        <!-- <ul
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
        </ul> -->
    </Col>
    <Col xs="auto">
        <div class="vr" style="height: 1.5rem;background: rgba(0, 0, 0, 0.2)" />
    </Col>
    <Col xs="auto" class="px-3 text-reset text-decoration-none">
        <Icon name="bell" size="1.5" />
    </Col>
    <Col xs="auto">
        <div class="vr" style="height: 1.5rem;background: rgba(0, 0, 0, 0.2)" />
    </Col>
    <Link
        to={paths.logout.path}
        class="col-auto px-3 text-reset text-decoration-none"
    >
        <Icon name="log-out" size="1.5" class="pe-1" />
        Logout
    </Link>
</div>

<style>
    * {
        /* outline: 1px solid red; */
    }
</style>
