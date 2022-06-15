<script>
    import { theme } from "../stores";
    import Icon from "./Icon.svelte";

    export let data;
    export let size;
    export let page;

    $: last_page = Math.ceil(data.length / size);
    $: current_page_size = data.slice((page - 1) * size, page * size).length;

    // Create number array with always 5 elements, with the current page in the middle if possible.
    // page = 1 => [1, 2, 3, 4, 5]
    // page = 2 => [1, 2, 3, 4, 5]
    // page = 3 => [1, 2, 3, 4, 5]
    // page = 4 => [2, 3, 4, 5, 6]
    // page = 5 => [3, 4, 5, 6, 7]
    // if there are a maximum of 7 pages, then:
    // page = 6 => [3, 4, 5, 6, 7]
    // page = 7 => [3, 4, 5, 6, 7]
    $: visible_pagination_indexes = [
        page > last_page - 2 ? page - 4 : -1,
        page > last_page - 1 ? page - 3 : -1,
        page - 2,
        page - 1,
        page,
        page + 1,
        page + 2,
        page < 2 ? page + 3 : -1,
        page < 3 ? page + 4 : -1,
    ].filter((page) => page > 0 && page <= last_page);
</script>

<div class="nav bg-dark w-100 justify-content-start no-select">
    <div
        class="nav-link fw-bold mouse-pointer {page === 1
            ? 'text-secondary'
            : 'text-light'}"
        on:click={() => (page = 1)}
    >
        &lt;&lt;
    </div>
    <div
        class="nav-link fw-bold mouse-pointer {page === 1
            ? 'text-secondary'
            : 'text-light'}"
        on:click={() => page > 1 && (page = page - 1)}
    >
        &lt;
    </div>
    {#each visible_pagination_indexes as index}
        <div
            class="nav-link fw-bold mouse-pointer {page === index
                ? 'bg-' +
                  $theme.color +
                  ($theme.color === 'yellow' ? ' text-dark' : ' text-light')
                : 'text-light'}"
            on:click={() => (page = index)}
        >
            {index}
        </div>
    {/each}
    {#if page < last_page - 3}
        <div class="nav-link text-light fw-bold">...</div>
    {/if}
    {#if page < last_page - 2}
        <div
            class="nav-link text-light fw-bold mouse-pointer"
            on:click={() => (page = last_page)}
        >
            {last_page}
        </div>
    {/if}
    <div
        class="nav-link fw-bold mouse-pointer {page === last_page
            ? 'text-secondary'
            : 'text-light'}"
        on:click={() => page < last_page && (page = page + 1)}
    >
        &gt;
    </div>
    <div
        class="nav-link fw-bold mouse-pointer {page === last_page
            ? 'text-secondary'
            : 'text-light'}"
        on:click={() => (page = last_page)}
    >
        &gt;&gt;
    </div>
    <div class="nav-item dropdown ms-3">
        <span
            class="nav-link text-light mouse-pointer"
            id="dropdownPaginatePageSize"
            role="button"
            data-bs-toggle="dropdown"
            aria-expanded="false"
        >
            Page size
            <Icon name="caret-down" size="1.125" />
        </span>
        <ul
            class="dropdown-menu dropdown-menu-dark bg-darker ms-4"
            aria-labelledby="dropdownPaginatePageSize"
        >
            {#each [10, 20, 50, 100, 250] as s}
                <li>
                    <span
                        class="dropdown-item mouse-pointer {size === s
                            ? 'fw-bold text-light'
                            : 'text-light'}"
                        on:click={() => (size = s)}>{s}</span
                    >
                </li>
            {/each}
        </ul>
    </div>
    <div class="nav-link text-muted">
        <small
            >Showing {size * (page - 1) + 1}-{size * (page - 1) +
                current_page_size} / {data.length}</small
        >
    </div>
</div>

<style>
    .dropdown-menu {
        min-width: auto;
    }
</style>
