<script lang="ts">
    import { onMount } from 'svelte';
    import { Card, Table } from 'sveltestrap';
    import Icon from '../../components/Icon.svelte';
    import { getEvents, showToast } from '../../controller';
    import TablePaginate from '../../components/TablePaginate.svelte';
    import { AlertType } from '../../models/AlertType';
    import { writable, type Writable } from 'svelte/store';

    let paginationSize: number = 20;
    let paginationPage: number = 1;

    const events: Writable<
        | {
              // SaltEvent
              id: string;
              timestamp: string;
              tag: string;
              data: string;

              // Added by loop
              jid: string;
              target: string;
              fun: string;
              data_parsed: any;
              data_formatted: string;
              unique_index: string;
          }[]
        | null
    > = writable(null);
    const expandedEvents: Writable<string[]> = writable([]);

    function toggleExpandEvent(index: string) {
        console.log(index);
        if ($expandedEvents.includes(index)) {
            expandedEvents.update((array) => array.filter((i) => i !== index));
        } else {
            expandedEvents.update((array) => [...array, index]);
        }
        console.log($expandedEvents); // TODO: remove
    }

    function updateData() {
        getEvents(paginationSize, (paginationPage - 1) * paginationSize)
            .then((data) => {
                events.set(
                    data.map((event) => {
                        const data: any = JSON.parse(
                            event.data ?? '{data: {}}',
                        ).data;
                        return {
                            ...event,
                            jid: data.jid ?? '',
                            target: data.id ?? '',
                            fun: data.fun ?? '',
                            data_parsed: data,
                            data_formatted: JSON.stringify(data, null, 2),
                            unique_index: (
                                (event.tag ?? '') +
                                '_' +
                                (event.timestamp ?? '')
                            ).replace(/ /g, '_'),
                        };
                    }),
                );
            })
            .catch((err) => {
                showToast(AlertType.ERROR, 'Failed fetching events', err);
            });
    }

    onMount(() => {
        updateData();
    });
</script>

<h1>Events</h1>

<Card class="table-responsive border-bottom-0">
    <Table hover class="b-0 mb-0">
        <thead class="bg-dark border-0 text-white">
            <tr>
                <th class="border-secondary">
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
                <th class="border-secondary">
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
                <th class="border-secondary">
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
                <th class="border-secondary">
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
                <th class="border-secondary">
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
            {#if $events === null}
                <p>Loading</p>
            {:else if $events.length === 0 && paginationPage === 1}
                <div class="p-3">No events exist. Very unusal.</div>
            {:else}
                {#each $events as event}
                    <tr>
                        <!-- <th>{event.id}</th> -->
                        <td
                            on:click={() =>
                                toggleExpandEvent(event.unique_index)}
                            class="mouse-pointer"
                        >
                            <Icon
                                size="1.125"
                                name={$expandedEvents.includes(
                                    event.unique_index,
                                )
                                    ? 'chevron-up'
                                    : 'chevron-down'}
                            />
                            {event.tag}
                        </td>
                        <td>{event.fun}</td>
                        <td>{event.target}</td>
                        <td>{event.jid}</td>
                        <td><small>{event.timestamp}</small></td>
                    </tr>
                    {#if $expandedEvents.includes(event.unique_index)}
                        <tr>
                            <td class="bg-light" colspan="5">
                                <pre
                                    class="text-left">{event.data_formatted}</pre>
                            </td>
                        </tr>
                    {/if}
                {/each}
            {/if}
        </tbody>
    </Table>
</Card>

<TablePaginate
    bind:size={paginationSize}
    bind:page={paginationPage}
    last={$events === null || $events.length < paginationSize}
    {updateData}
/>
