<script>
    import { Link } from "svelte-navigator";
    import { load_minions } from "../../controller";
    import { minions } from "../../stores";
    import paths from "../../paths";
    import Redirect from "../../components/Redirect.svelte";

    import MinionInfo from "./MinionInfo.svelte";
    import MinionGrains from "./MinionGrains.svelte";
    import MinionPillars from "./MinionPillars.svelte";
    import MinionPackages from "./MinionPackages.svelte";
    import MinionConformity from "./MinionConformity.svelte";

    export let navigate;
    export let location;

    export let minionId;

    $: minion = ($minions ?? []).filter((minion) => minion.id === minionId)[0];
    $: minion === undefined && load_minions(navigate);

    $: subPage = location.pathname.split("/")[4];
    $: console.log("location", location, subPage);

    $: subPagesNav = [
        {
            name: "General",
            path: paths.minion.getPath(minionId),
        },
        {
            name: "Conformity",
            path: paths.minion.getPath(minionId, "conformity"),
        },
        {
            name: "Grains",
            path: paths.minion.getPath(minionId, "grains"),
        },
        {
            name: "Pillars",
            path: paths.minion.getPath(minionId, "pillars"),
        },
        {
            name: "Packages",
            path: paths.minion.getPath(minionId, "packages"),
        },
    ];
</script>

<h1>Minion {minion.id}</h1>

<div class="nav bg-dark w-100">
    {#each subPagesNav as item}
        <Link
            to={item.path}
            class="nav-link text-white px-4 py-3 fw-bold {(item.name ===
                'General' &&
                subPage === undefined) ||
            subPage === item.name.toLowerCase()
                ? 'bg-gold'
                : ''}"
        >
            {item.name}
        </Link>
    {/each}
</div>

{#if !minion}
    <h1>Loading...</h1>
{:else}
    <div class="card border-4 border-gold rounded-none">
        <div class="card-body p-0">
            {#if subPage === undefined}
                <MinionInfo {minion} />
            {:else if subPage === "grains"}
                <MinionGrains {minion} />
            {:else if subPage === "pillars"}
                <MinionPillars {minion} />
            {:else if subPage === "packages"}
                <MinionPackages {minion} />
            {:else if subPage === "conformity"}
                <MinionConformity {minion} />
            {:else}
                <Redirect to={paths.minion.getPath(minionId)} />
            {/if}
        </div>
    </div>
{/if}
