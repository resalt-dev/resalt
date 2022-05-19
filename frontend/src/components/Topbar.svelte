<script>
    import { useLocation, Link } from "svelte-navigator";
    import { socket } from "../stores";
    import paths from "../paths";
    import constants from "../constants";
    import Icon from "../components/Icon.svelte";

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

<div class="row g-0 d-flex align-items-center bg-light">
    <div class="col">
        <div class="btn-group me-3" role="group">
            {#each navbar as item}
                {#if item.path}
                    <Link
                        to={item.path}
                        class={`btn btn-${constants.mainColor} btn-arrow-right fw-bold`}
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
        <div class="btn-group me-2" role="group" aria-label="Basic example">
            <button
                type="button"
                class="btn btn-dark btn-sm d-flex align-items-center"
            >
                <Icon size="1.3" name="palette" />
            </button>
            <button
                type="button"
                class="btn btn-dark btn-sm d-flex align-items-center"
            >
                <Icon size="1.3" name="bell" />
            </button>
            <button
                type="button"
                class="btn btn-dark btn-sm d-flex align-items-center"
            >
                <Icon size="1.3" name="wrench" />
            </button>
        </div>
        {#if $socket.connected}
            <!-- display last_ping as hh:mm:ss -->
            <span class="badge rounded-pill bg-success"
                >Connected: {new Date($socket.last_ping)
                    .toLocaleTimeString()
                    .replace(/\./g, ":")}</span
            >
        {:else}
            <span class="badge rounded-pill bg-danger">Disconnected</span>
        {/if}
    </div>
</div>
