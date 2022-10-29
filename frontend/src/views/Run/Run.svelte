<script lang="ts">
    import { Card } from 'sveltestrap';
    import { writable, type Writable } from 'svelte/store';
    import ConsoleChangeBranch from '../../components/ConsoleChangeBranch.svelte';
    import paths from '../../paths';
    import Tabs from '../../components/Tabs.svelte';
    import type { NavigateFn } from 'svelte-navigator';
    import type TabPage from '../../models/TabPage';

    import RunTabLive from './RunTabLive.svelte';

    // svelte-ignore unused-export-let
    export let location: Location;
    // svelte-ignore unused-export-let
    export let navigate: NavigateFn;
    export let subPage: string = '';

    let returns: Writable<any[]> = writable([]);

    let tabs: TabPage[] = [];
    $: tabs = [
        {
            key: 'live',
            label: 'Live Run',
            path: paths.run.getPath(),
            component: RunTabLive,
            data: { returns },
        },
    ];
</script>

<h1>Run</h1>

<Tabs {tabs} current={subPage} />

{#each $returns as ret}
    <Card class="result-box mb-3">
        <div
            type="button"
            class="card-header"
            data-bs-toggle="collapse"
            data-bs-target="#conformityCollapse{ret.num}"
        >
            <span>Result : </span>
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
                        <ConsoleChangeBranch data={ret.data} />
                    {/if}
                </div>
            </div>
        </div>
    </Card>
{/each}
