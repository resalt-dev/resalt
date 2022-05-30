<script>
    import { onMount } from "svelte";
    import { Table } from "sveltestrap";
    import Icon from "../components/Icon.svelte";
    import { get_events } from "../controller";
    import { theme } from "../stores";

    let events = [];

    $: mapped_events = (events ?? []).map((event) => {
        const data = JSON.parse(event.data ?? "{data: {}}").data;
        return {
            ...event,
            jid: data.jid ?? "",
            target: data.id ?? "",
            fun: data.fun ?? "",
            data_parsed: data,
            data_formatted: JSON.stringify(data, null, 2),
        };
    });

    onMount(() => {
        get_events().then((data) => {
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
        class="b-0 mb-0 {$theme.dark ? 'text-white border-secondary' : ''}"
    >
        <thead class="bg-dark text-white border-0">
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
            {#each mapped_events as minion}
                <tr>
                    <!-- <th scope="row">{minion.id}</th> -->
                    <td>{minion.tag}</td>
                    <td>{minion.jid}</td>
                    <td>{minion.target}</td>
                    <td>{minion.fun}</td>
                    <td>{minion.timestamp}</td>
                </tr>
            {/each}
        </tbody>
    </Table>
</div>

{#if events.length === 0}
    <div class="p-3">Loading events...</div>
{:else}
    <div class="p-3">Fetched the last 100 events.</div>
{/if}
