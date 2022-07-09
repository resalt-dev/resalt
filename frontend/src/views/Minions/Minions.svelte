<script lang="ts">
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import {
        AlertType,
        refreshMinions,
        getMinions,
        showAlert,
    } from "../../controller";
    import { theme } from "../../stores";
    import Icon from "../../components/Icon.svelte";
    import paths from "../../paths";
    import { Link, useNavigate } from "svelte-navigator";
    import {
        Button,
        Card,
        CardBody,
        Col,
        Input,
        Row,
        Table,
    } from "sveltestrap";
    import TablePaginate from "../../components/TablePaginate.svelte";
    const navigate = useNavigate();

    let paginationSize: number = 10;
    let paginationPage: number = 1;

    const minions = writable(null);

    function updateData() {
        getMinions(paginationSize, (paginationPage - 1) * paginationSize)
            .then((data) => {
                minions.set(
                    data.map((minion) => {
                        const grains = JSON.parse(minion.grains ?? "{}");
                        return {
                            ...minion,
                            datatable_type: (
                                (grains["osfullname"] ?? "Unknown") +
                                " " +
                                (grains["osrelease"] ?? "")
                            ).trim(),
                        };
                    })
                );
            })
            .catch((err) => {
                showAlert(AlertType.ERROR, "Failed fetching minions", err);
            });
    }

    onMount(() => {
        updateData();
    });
</script>

<h1>Minions</h1>

<div class="nav bg-dark w-100 no-select">
    {#each ["Search Options", "Groups"].filter( (k) => isNaN(Number(k)) ) as fpage}
        <div
            class="nav-link px-4 py-3 fw-bold mouse-pointer {fpage != 'Groups'
                ? 'bg-' + $theme.color
                : ''} {$theme.color === 'yellow' && fpage != 'Groups'
                ? 'text-dark'
                : 'text-white'}"
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
                <Input
                    id="minionsSearch"
                    type="text"
                    placeholder="Search minions"
                    class="form-control ms-2 d-inline"
                    style="width: 15rem;"
                />
                <input
                    id="minionsSearch"
                    type="email"
                    class="form-control ms-2 d-inline"
                    style="width: 15rem;"
                />
            </Col>
        </Row>

        <!-- TEMP -->
        <Button color="secondary" size="sm" on:click={() => refreshMinions()}>
            Force reload minions
        </Button>
    </CardBody>
</Card>

<div class="table-responsive card {$theme.dark ? 'bg-dark' : ''}">
    <Table
        dark={$theme.dark}
        hover
        id="minionListTable"
        class="b-0 mb-0 {$theme.dark ? 'text-light border-secondary' : ''}"
    >
        <thead
            class="bg-dark border-0 {$theme.dark ? 'text-light' : 'text-white'}"
        >
            <tr>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center ps-2">ID</div>
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
                    </div>
                </th>
                <th scope="col" class="border-secondary">
                    <div class="row g-1">
                        <div class="col-auto align-self-center">Last seen</div>
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
                        <div class="col-auto align-self-center">Conformity</div>
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
            {#if $minions == null}
                <p>Loading</p>
            {:else if $minions.length == 0}
                <div class="p-3">No minions detected. Try force reload.</div>
            {:else}
                {#each $minions as minion}
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
                                <span class="badge align-middle bg-purple">
                                    Unknown
                                </span>
                            {:else}
                                <span
                                    class="badge align-middle bg-green fw-bold"
                                >
                                    {minion.conformity_success ?? "?"}
                                </span>
                                /
                                <span
                                    class="badge align-middle bg-warning fw-bold"
                                >
                                    {minion.conformity_incorrect ?? "?"}
                                </span>
                                /
                                <span class="badge align-middle bg-red fw-bold">
                                    {minion.conformity_error ?? "?"}
                                </span>
                            {/if}
                        </td>
                        <td>
                            <Link
                                to={paths.minion.getPath(minion.id)}
                                class="btn btn-{$theme.color} btn-sm px-3"
                                >View</Link
                            >
                        </td>
                    </tr>
                {/each}
            {/if}
        </tbody>
    </Table>
    <TablePaginate
        bind:size={paginationSize}
        bind:page={paginationPage}
        last={$minions == null || $minions.length == 0}
        {updateData}
    />
</div>
