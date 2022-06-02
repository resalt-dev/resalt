<script lang="ts">
    import { onMount } from "svelte";
    import { load_minions } from "../../controller";
    import { minions as minions_store, theme } from "../../stores";
    import Icon from "../../components/Icon.svelte";
    import paths from "../../paths";
    import { Link, useNavigate } from "svelte-navigator";
    import {
        Button,
        Card,
        CardBody,
        CardHeader,
        Col,
        Row,
        Table,
    } from "sveltestrap";
    import TablePaginate from "../../components/TablePaginate.svelte";

    enum FilterPage {
        Search,
        Groups,
    }

    const navigate = useNavigate();

    let filter_page: FilterPage = FilterPage.Search;
    let pagination_size: number = 10;
    let pagination_page: number = 1;

    $: minions = $minions_store ?? [];
    $: filtered_minions = minions.filter((minion) => true);
    $: paginated_minions = filtered_minions.slice(
        (pagination_page - 1) * pagination_size,
        pagination_page * pagination_size
    );
    $: mapped_minions = paginated_minions.map((minion) => {
        const grains = JSON.parse(minion.grains ?? "{}");
        return {
            ...minion,
            datatable_type: (
                (grains["osfullname"] ?? "Unknown") +
                " " +
                (grains["osrelease"] ?? "")
            ).trim(),
        };
    });

    onMount(() => {
        load_minions(navigate);
    });
</script>

<h1>Minions</h1>

<div class="nav bg-dark w-100">
    {#each Object.keys(FilterPage).filter((k) => isNaN(Number(k))) as fpage}
        <div
            on:click={() => (filter_page = FilterPage[fpage])}
            class="nav-link text-white px-4 py-3 fw-bold mouse-pointer {fpage ===
                FilterPage[filter_page] &&
                'bg-' + $theme.color} {$theme.color === 'yellow' &&
                fpage === FilterPage[filter_page] &&
                'text-dark'}"
        >
            {fpage}
        </div>
    {/each}
</div>

<Card
    class="mb-3 {$theme.dark ? 'bg-dark border-0' : ''}"
    style="border-radius: 0px !important"
>
    <CardBody>
        <Row>
            <Col class="mb-4">
                <label for="minionsSearch" class="form-label d-inline"
                    >ABC</label
                >
                <input
                    id="minionsSearch"
                    type="email"
                    class="form-control ms-2 d-inline"
                    style="width: 15rem;"
                />
            </Col>
        </Row>
        <Button
            color="secondary"
            size="sm"
            on:click={() => load_minions(navigate, true)}
        >
            Force reload minions
        </Button>
    </CardBody>
</Card>

{#if minions.length === 0}
    <div class="p-3">No minions detected. Try force reload.</div>
{:else}
    <div class="table-responsive card {$theme.dark ? 'bg-dark' : ''}">
        <Table
            dark={$theme.dark}
            hover
            id="minionListTable"
            class="b-0 mb-0 {$theme.dark ? 'text-white border-secondary' : ''}"
        >
            <thead class="bg-dark text-white border-0">
                <tr>
                    <th scope="col" class="border-secondary">
                        <div class="row g-1">
                            <div class="col-auto align-self-center ps-2">
                                ID
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
                                <input
                                    type="text"
                                    class="ms-1 lh-1"
                                    size="15"
                                />
                            </div>
                        </div>
                    </th>
                    <th scope="col" class="border-secondary">
                        <div class="row g-1">
                            <div class="col-auto align-self-center">Type</div>
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
                                <input
                                    type="text"
                                    class="ms-1 lh-1"
                                    size="15"
                                />
                            </div>
                        </div>
                    </th>
                    <th scope="col" class="border-secondary">
                        <div class="row g-1">
                            <div class="col-auto align-self-center">
                                Last seen
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
                        </div>
                    </th>
                    <th scope="col" class="border-secondary">
                        <div class="row g-1">
                            <div class="col-auto align-self-center">
                                Conformity
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
                        </div>
                    </th>
                    <th scope="col" class="border-secondary">Actions</th>
                </tr>
            </thead>
            <tbody class="align-middle">
                {#each mapped_minions as minion}
                    <tr>
                        <th
                            scope="row"
                            class="mouse-pointer"
                            on:click={() =>
                                navigate(paths.minion.getPath(minion.id))}
                            >{minion.id}</th
                        >
                        <td>{minion.datatable_type}</td>
                        <td>{minion.last_seen}</td>
                        <td>
                            {#if minion.last_updated_conformity == null}
                                <span class="badge mb-1 align-middle bg-purple">
                                    Unknown
                                </span>
                            {:else}
                                <span
                                    class="badge mb-1 align-middle bg-green fw-bold"
                                >
                                    {minion.conformity_success ?? "?"}
                                </span>
                                /
                                <span
                                    class="badge mb-1 align-middle bg-warning fw-bold"
                                >
                                    {minion.conformity_incorrect ?? "?"}
                                </span>
                                /
                                <span
                                    class="badge mb-1 align-middle bg-red fw-bold"
                                >
                                    {minion.conformity_error ?? "?"}
                                </span>
                            {/if}
                        </td>
                        <td>
                            <Link
                                to={paths.minion.getPath(minion.id)}
                                class={`btn btn-primary btn-sm px-3`}>View</Link
                            >
                        </td>
                    </tr>
                {/each}
            </tbody>
        </Table>

        <TablePaginate
            data={filtered_minions}
            bind:size={pagination_size}
            bind:page={pagination_page}
        />
    </div>
{/if}
