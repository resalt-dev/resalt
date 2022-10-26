<script lang="ts">
    import { writable, type Writable } from 'svelte/store';
    import { theme } from '../../stores';
    import Tabs from '../../components/Tabs.svelte';
    import ConsoleChangeBranch from '../../components/ConsoleChangeBranch.svelte';
    import RunTabLive from './RunTabLive.svelte';
    import { Card } from 'sveltestrap';

    const SHIFT = 0;

    let returns: Writable<any[]> = writable([]);
</script>

<h1>Run</h1>

<Tabs
    children={[
        {
            label: 'Live Run',
            component: RunTabLive,
            data: { returns },
        },
    ]}
/>

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
                        <ConsoleChangeBranch data={ret.data} shift={SHIFT} />
                    {/if}
                </div>
            </div>
        </div>
    </Card>
{/each}
