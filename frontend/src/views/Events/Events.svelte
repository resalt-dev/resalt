<script lang="ts">
    import { onMount } from 'svelte';
    import { Card, Table } from 'sveltestrap';
    import Icon from '../../components/Icon.svelte';
    import { getEvents } from '../../api';
    import { toasts } from '../../stores';
    import TablePaginate from '../../components/TablePaginate.svelte';
    import { MessageType } from '../../models/MessageType';
    import { writable, type Writable } from 'svelte/store';
    import type { NavigateFn } from 'svelte-navigator';
	import type EventsWriteableData from '../../models/EventsWriteableData';

    // svelte-ignore unused-export-let
    export let location: Location;
    // svelte-ignore unused-export-let
    export let navigate: NavigateFn;

    let paginationSize: number = 20;
    let paginationPage: number = 1;

    const events: Writable<EventsWriteableData[] | null> = writable(null);
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
                toasts.add(MessageType.ERROR, 'Failed fetching events', err);
            });
    }

    onMount(() => {
        updateData();
    });
</script>

<Card class="table-responsive border-bottom-0">
    <Table hover class="b-0 mb-0">
        <thead class="bg-dark border-0 text-white">
            <tr>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center ps-2">Tag</div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Function</div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Target</div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Job ID</div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Date</div>
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
                                size="1.25"
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
