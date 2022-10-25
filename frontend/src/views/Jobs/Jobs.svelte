<script lang="ts">
    import { onMount } from 'svelte';
    import { getJobs, showToast } from '../../controller';
    import { Card, Table, Tooltip } from 'sveltestrap';
    import Icon from '../../components/Icon.svelte';
    import { writable, type Writable } from 'svelte/store';
    import TablePaginate from '../../components/TablePaginate.svelte';
    import { MessageType } from '../../models/MessageType';
    import type Job from '../../models/Job';

    let filterUser: string | null = null;
    let filterStartDate: Date | null = null;
    let filterEndDate: Date | null = null;
    let paginationSize: number = 20;
    let paginationPage: number = 1;

    const jobs: Writable<Job[] | null> = writable(null);

    function updateData() {
        getJobs(
            filterUser,
            filterStartDate,
            filterEndDate,
            paginationSize,
            (paginationPage - 1) * paginationSize,
        )
            .then((data) => {
                jobs.set(data);
            })
            .catch((err) => {
                showToast(MessageType.ERROR, 'Failed fetching jobs', err);
            });
    }

    onMount(() => {
        updateData();
    });

    let jobIdTooltipElement;
</script>

<h1>Jobs</h1>

<Card class="table-responsive border-bottom-0">
    <Table hover class="b-0 mb-0">
        <thead class="bg-dark border-0 text-white">
            <tr>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center ps-2">
                            JID<Icon
                                size="0.95"
                                name="help-circle"
                                class="mb-0 h3 text-muted align-top"
                                bind:htmlElement={jobIdTooltipElement}
                            />
                            <Tooltip target={jobIdTooltipElement}
                                >Job ID</Tooltip
                            >
                        </div>
                        <div class="col-auto">
                            <Icon
                                size="1.25"
                                name="chevron-up"
                                class="sort-icon-up"
                            />
                            <Icon
                                size="1.25"
                                name="chevron-down"
                                class="sort-icon-down"
                            />
                        </div>
                        <div class="col-auto align-self-center">
                            <input type="text" class="ms-1 lh-1" size="15" />
                        </div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">User</div>
                        <div class="col-auto">
                            <Icon
                                size="1.25"
                                name="chevron-up"
                                class="sort-icon-up"
                            />
                            <Icon
                                size="1.25"
                                name="chevron-down"
                                class="sort-icon-down"
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
                        <div class="col-auto">
                            <Icon
                                size="1.25"
                                name="chevron-up"
                                class="sort-icon-up"
                            />
                            <Icon
                                size="1.25"
                                name="chevron-down"
                                class="sort-icon-down"
                            />
                        </div>
                        <div class="col-auto align-self-center">
                            <input type="text" class="ms-1 lh-1" size="12" />
                        </div>
                    </div>
                </th>
                <th class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Date</div>
                        <div class="col-auto">
                            <Icon
                                size="1.25"
                                name="chevron-up"
                                class="sort-icon-up"
                            />
                            <Icon
                                size="1.25"
                                name="chevron-down"
                                class="sort-icon-down"
                            />
                        </div>
                    </div>
                </th>
            </tr>
        </thead>
        <tbody class="align-middle">
            {#if $jobs === null}
                <p>Loading</p>
            {:else if $jobs.length === 0 && paginationPage === 1}
                <div class="p-3">No jobs exist. Very unusal.</div>
            {:else}
                {#each $jobs as job}
                    <tr>
                        <th>{job.jid}</th>
                        <td>{job.user}</td>
                        <td>-</td>
                        <td><small>{job.timestamp}</small></td>
                    </tr>
                {/each}
            {/if}
        </tbody>
    </Table>
</Card>

<TablePaginate
    bind:size={paginationSize}
    bind:page={paginationPage}
    last={$jobs === null || $jobs.length < paginationSize}
    {updateData}
/>
