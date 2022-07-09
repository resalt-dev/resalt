<script lang="ts">
    import { onMount } from "svelte";
    import { theme } from "../../stores";
    import { useNavigate } from "svelte-navigator";
    import { AlertType, get_jobs, showAlert } from "../../controller";
    import { Table, Tooltip } from "sveltestrap";
    import Icon from "../../components/Icon.svelte";
    const navigate = useNavigate();

    let pagination_size: number = 20;
    let pagination_page: number = 1;

    let jobs = null;

    onMount(() => {
        get_jobs()
            .then((data) => {
                jobs = data;
            })
            .catch((err) => {
                showAlert(AlertType.ERROR, "Failed fetching jobs", err);
            });
    });

    let jobIdTooltipElement;
</script>

<h1>Jobs</h1>

<div class="table-responsive card {$theme.dark ? 'bg-dark' : ''}">
    <Table
        dark={$theme.dark}
        hover
        id="jobListTable"
        class="b-0 mb-0 {$theme.dark ? 'text-light border-secondary' : ''}"
    >
        <thead
            class="bg-dark border-0 {$theme.dark ? 'text-light' : 'text-white'}"
        >
            <tr>
                <th scope="col" class="border-secondary">
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
                        <div class="col-auto align-self-center">User</div>
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
            {#if jobs == null}
                <p>Loading</p>
            {:else}
                {#each jobs as job}
                    <tr>
                        <th scope="row">{job.jid}</th>
                        <td>{job.user}</td>
                        <td>-</td>
                        <td><small>{job.timestamp}</small></td>
                    </tr>
                {/each}
            {/if}
        </tbody>
    </Table>
</div>
