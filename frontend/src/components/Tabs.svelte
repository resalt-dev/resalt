<script lang="ts">
    import { Card, CardBody } from 'sveltestrap';
    import { theme } from '../stores';
    import type { NavSubPage } from '../utils';

    export let children: NavSubPage[] = [];
    export let selected: number = 0;
    export let onSelect: (index: number) => void = () => {};
</script>

<div class="nav bg-black w-100 no-select">
    {#each children as tab, i}
        <div
            class="nav-link px-4 py-3 fw-bold mouse-pointer {i === selected
                ? 'bg-' + $theme.color
                : ''} {$theme.color === 'yellow' && i === selected
                ? 'text-dark'
                : 'text-white'}"
            on:click={() => {
                selected = i;
                onSelect(i);
            }}
        >
            {tab.label}
        </div>
    {/each}
</div>
{#if selected < children.length}
    <Card
        class="mb-3 border-4 border-{$theme.color} rounded-none {$theme.dark
            ? 'bg-dark'
            : ''}"
    >
        <CardBody class={children[selected].class || ''}>
            <svelte:component
                this={children[selected].component}
                label={children[selected].label}
                tabData={children[selected].data || undefined}
            />
        </CardBody>
    </Card>
{/if}
