<script lang="ts">
    import { writable } from "svelte/store";
    import { theme } from "../../stores";
    import Tabs from "../../components/Tabs.svelte";
    import ConsoleChangeBranch from "../Minion/ConsoleChangeBranch.svelte";
    import RunTabLive from "./RunTabLive.svelte";

    const SHIFT = 0;

    let returns = writable([]);
</script>

<h1>Run</h1>

<Tabs
    children={[
        {
            label: "Live Run",
            component: RunTabLive,
        },
    ]}
    tabData={{
        returns: returns,
    }}
/>

{#each $returns as ret}
    <div class="card mb-3 {$theme.dark ? 'bg-secondary' : ''}">
        <div
            type="button"
            class="card-header"
            data-bs-toggle="collapse"
            data-bs-target="#conformityCollapse{ret.num}"
        >
            <span class="fw-bold">Result : </span>
            ({ret.command.targetType}) {ret.command.target}
            <small class="text-muted">({ret.command.fun})</small>
            <small class="float-end text-muted pt-1">
                # {ret.num + 1}
            </small>
        </div>
        <div class="collapse show" id="conformityCollapse{ret.num}">
            <div class="card-body bg-dark text-light">
                <div class="card-text">
                    {#if Object.keys(ret.data).length != 0}
                        <ConsoleChangeBranch data={ret.data} shift={SHIFT} />
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/each}
