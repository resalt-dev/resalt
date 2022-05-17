<script>
    import { useLocation, Link } from "svelte-navigator";
    import paths from "../paths";
    import constants from "../constants";

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

<div class="row g-0 bg-light">
    <div class="col align-self-start">
        <div class="btn-group" role="group">
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
    <div class="col-2 align-self-end">Right</div>
</div>
