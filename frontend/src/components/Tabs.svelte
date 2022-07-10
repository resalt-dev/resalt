<script lang="ts">
    import { Card, CardBody } from "sveltestrap";
    import { theme } from "../stores";

    export let children = [];
    let selected: number = 0;
</script>

<div class="nav bg-dark w-100 no-select">
    {#each children as tab, i}
        <div
            class="nav-link px-4 py-3 fw-bold mouse-pointer {i === selected
                ? 'bg-' + $theme.color
                : ''} {$theme.color === 'yellow' && i === selected
                ? 'text-dark'
                : 'text-white'}"
            on:click={() => {
                selected = i;
            }}
        >
            {tab.label}
        </div>
    {/each}
</div>
<Card
    class="mb-3 {$theme.dark ? 'bg-dark border-0' : ''}"
    style="border-radius: 0px !important"
>
    <CardBody>
        <svelte:component
            this={children[selected].component}
            label={children[selected].label}
        />
    </CardBody>
</Card>
