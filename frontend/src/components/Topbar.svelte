<script lang="ts">
    import { useLocation, Link } from "svelte-navigator";
    import { socket, theme } from "../stores";
    import paths from "../paths";

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
            <span class="badge rounded-pill bg-success"
                >Connected: {new Date($socket.last_ping)
                    .toLocaleTimeString("en-US", {
                        timeZone: "UTC",
                        timeZoneName: "short",
                        hour12: false,
                    })
                    .replace(/\./g, ":")}</span
            >
        {:else}
            <span class="badge rounded-pill bg-danger">Disconnected</span>
        {/if}
    </div>
</div>
