<script lang="ts">
    import { SortOrder } from '../models/SortOrder';
    import Icon from './Icon.svelte';

    const SORT_COLOR = 'text-orange sort-active';

    export let active: string;
    export let field: string;
    export let order: SortOrder;
    export let click: (field: string, order: SortOrder) => void;
    export let color: string = '';
</script>

<Icon
    size="1.25"
    name={order === SortOrder.Down ? 'chevron-up' : 'chevron-down'}
    class="sort-icon-up {active === `${field}:${order}`
        ? SORT_COLOR
        : `text-${color}`}"
    on:click={() => {
        click(field, order);
    }}
/>
