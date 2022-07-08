<script lang="ts">
    import { onMount } from "svelte";
    import { Table } from "sveltestrap";
    import Icon from "../components/Icon.svelte";
    import { get_events } from "../controller";
    import { theme } from "../stores";
    import { useNavigate } from "svelte-navigator";
    import TablePaginate from "../components/TablePaginate.svelte";
    const navigate = useNavigate();

    let pagination_size: number = 20;
    let pagination_page: number = 1;
    let events = [];
    let expanded_events = [];

    $: mapped_events = events.map((event) => {
        const data = JSON.parse(event.data ?? "{data: {}}").data;
        return {
            ...event,
            jid: data.jid ?? "",
            target: data.id ?? "",
            fun: data.fun ?? "",
            data_parsed: data,
            data_formatted: JSON.stringify(data, null, 2),
            unique_index: (
                (event.tag ?? "") +
                "_" +
                (event.timestamp ?? "")
            ).replace(/ /g, "_"),
        };
    });
    $: filtered_events = mapped_events.filter((event) => true);
    $: paginated_events = filtered_events.slice(
        (pagination_page - 1) * pagination_size,
        pagination_page * pagination_size
    );

    function toggle_event_expand(index: string) {
        console.log(index);
        if (expanded_events.includes(index)) {
            expanded_events = expanded_events.filter((i) => i !== index);
        } else {
            expanded_events = [...expanded_events, index];
        }
        console.log(expanded_events);
    }

    onMount(() => {
        get_events(navigate).then((data) => {
            events = data;
        });
    });
</script>

<h1>Events</h1>

<div class="table-responsive card {$theme.dark ? 'bg-dark' : ''}">
    <Table
        dark={$theme.dark}
        hover
        id="eventListTable"
        class="b-0 mb-0 {$theme.dark ? 'text-light border-secondary' : ''}"
    >
        <thead class="bg-dark text-light border-0">
            <tr>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center ps-2">Tag</div>
                        <div class="col-auto align-self-center d-grid">
                            <Icon
                                size="1.125"
                                name="chevron-up"
                                class="sort-icon mouse-pointer"
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer"
                            />
                        </div>
                        <div class="col-auto align-self-center">
                            <input type="text" class="ms-1 lh-1" size="15" />
                        </div>
                    </div>
                </th>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Function</div>
                        <div class="col-auto align-self-center d-grid">
                            <Icon
                                size="1.125"
                                name="chevron-up"
                                class="sort-icon mouse-pointer"
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer"
                            />
                        </div>
                        <div class="col-auto align-self-center">
                            <input type="text" class="ms-1 lh-1" size="12" />
                        </div>
                    </div>
                </th>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Target</div>
                        <div class="col-auto align-self-center d-grid">
                            <Icon
                                size="1.125"
                                name="chevron-up"
                                class="sort-icon mouse-pointer"
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer"
                            />
                        </div>
                        <div class="col-auto align-self-center">
                            <input type="text" class="ms-1 lh-1" size="12" />
                        </div>
                    </div>
                </th>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Job ID</div>
                        <div class="col-auto align-self-center d-grid">
                            <Icon
                                size="1.125"
                                name="chevron-up"
                                class="sort-icon mouse-pointer"
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer"
                            />
                        </div>
                        <div class="col-auto align-self-center">
                            <input type="text" class="ms-1 lh-1" size="15" />
                        </div>
                    </div>
                </th>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Date</div>
                        <div class="col-auto align-self-center d-grid">
                            <Icon
                                size="1.125"
                                name="chevron-up"
                                class="sort-icon mouse-pointer"
                            />
                            <Icon
                                size="1.125"
                                name="chevron-down"
                                class="sort-icon mouse-pointer"
                            />
                        </div>
                    </div>
                </th>
            </tr>
        </thead>
        <tbody class="align-middle">
            {#each paginated_events as event}
                <tr>
                    <!-- <th scope="row">{event.id}</th> -->
                    <td
                        on:click={() => toggle_event_expand(event.unique_index)}
                        class="mouse-pointer"
                    >
                        <Icon
                            size="1.125"
                            name={expanded_events.includes(event.unique_index)
                                ? "chevron-down"
                                : "chevron-up"}
                        />
                        {event.tag}
                    </td>
                    <td>{event.fun}</td>
                    <td>{event.target}</td>
                    <td>{event.jid}</td>
                    <td><small>{event.timestamp}</small></td>
                </tr>
                {#if expanded_events.includes(event.unique_index)}
                    <tr>
                        <td
                            class={$theme.dark ? "bg-secondary" : "bg-light"}
                            colspan="5"
                        >
                            <pre class="text-left">{event.data_formatted}</pre>
                        </td>
                    </tr>
                {/if}
            {/each}
        </tbody>
    </Table>
    <TablePaginate
        data={filtered_events}
        bind:size={pagination_size}
        bind:page={pagination_page}
    />
</div>

{#if events.length === 0}
    <div class="p-3">Loading events...</div>
{:else}
    <div class="p-3 text-muted">
        Note: Only a maximum of 2,000 most recent events (100 pages) can be
        displayed and filtered.
    </div>
{/if}
