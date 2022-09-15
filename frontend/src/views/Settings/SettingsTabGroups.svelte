<script lang="ts">
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import {
        Card,
        CardBody,
        CardHeader,
        Col,
        FormGroup,
        Input,
        Label,
        Progress,
        Row,
        Table,
    } from "sveltestrap";
    import TablePaginate from "../../components/TablePaginate.svelte";
    import { getPermissionGroups, showAlert } from "../../controller";
    import { AlertType } from "../../models/AlertType";
    import { theme } from "../../stores";

    let paginationSize: number = 20;
    let paginationPage: number = 1;

    const groups = writable(null);
    const selectedGroup = writable(null);

    function updateData() {
        getPermissionGroups(
            paginationSize,
            (paginationPage - 1) * paginationSize
        )
            .then((data) => {
                groups.set(data);
                if (data.length > 0 && $selectedGroup === null) {
                    selectedGroup.set(data[0]);
                }
            })
            .catch((err) => {
                showAlert(AlertType.ERROR, "Failed fetching groups", err);
            });
    }

    onMount(() => {
        updateData();
    });
</script>

<Row>
    <Col>
        <Card
            class="table-responsive border-bottom-0 {$theme.dark
                ? 'bg-dark'
                : ''}"
        >
            <Table
                dark={$theme.dark}
                class="b-0 mb-0 {$theme.dark
                    ? 'text-light border-secondary'
                    : ''}"
            >
                <thead
                    class="bg-dark border-0 {$theme.dark
                        ? 'text-light'
                        : 'text-white'}"
                >
                    <tr>
                        <th scope="col" class="border-secondary">
                            <div class="row g-1">
                                <div class="col-auto align-self-center ps-2">
                                    Group Name
                                </div>
                            </div>
                        </th>
                        <th scope="col" class="border-secondary">
                            <div class="row g-1">
                                <div class="col-auto align-self-center">
                                    Members
                                </div>
                            </div>
                        </th>
                    </tr>
                </thead>
                <tbody class="align-middle">
                    {#if $groups}
                        {#each $groups as group}
                            <tr
                                class="mouse-pointer {$selectedGroup?.id ===
                                group.id
                                    ? 'text-white'
                                    : ''}"
                                on:click={() => {
                                    selectedGroup.set(group);
                                }}
                            >
                                <th
                                    scope="row"
                                    class={$selectedGroup?.id === group.id
                                        ? "bg-" + $theme.color
                                        : ""}
                                >
                                    {group.name}
                                </th>
                                <td
                                    class={$selectedGroup?.id === group.id
                                        ? "bg-" + $theme.color
                                        : ""}
                                >
                                    {group.users.length}
                                </td>
                            </tr>
                        {/each}
                    {/if}
                </tbody>
            </Table>
            <TablePaginate
                bind:size={paginationSize}
                bind:page={paginationPage}
                last={$groups === null || $groups.length < paginationSize}
                {updateData}
            />
            {#if !$groups}
                <Progress animated color={$theme.color} value={100}>
                    Loading...
                </Progress>
            {/if}
        </Card>
    </Col>
    <Col>
        <Card class={$theme.dark ? "bg-dark" : ""}>
            <CardHeader>
                <span class="fw-bold">Edit Group</span>
            </CardHeader>
            <CardBody>
                {#if $selectedGroup === null}
                    <h1>Select a group to edit</h1>
                {:else}
                    <Row>
                        <Col class="form-check ps-3 mb-0" md="12">
                            <FormGroup floating={true}>
                                <Input
                                    id="groupID"
                                    type="text"
                                    bind:value={$selectedGroup.id}
                                    disabled
                                />
                                <Label for="arguments">Group ID</Label>
                            </FormGroup>
                        </Col>
                        <Col class="form-check ps-3 mb-0" md="12">
                            <FormGroup floating={true}>
                                <Input
                                    id="groupName"
                                    type="text"
                                    bind:value={$selectedGroup.name}
                                />
                                <Label for="arguments">Group Name</Label>
                            </FormGroup>
                        </Col>
                        <Col class="form-check ps-3 mb-0" md="12">
                            <FormGroup floating={true}>
                                <Input
                                    id="groupLdapSync"
                                    type="text"
                                    bind:value={$selectedGroup.ldapSync}
                                />
                                <Label for="arguments"
                                    >LDAP Sync Group (optional)</Label
                                >
                            </FormGroup>
                        </Col>
                        <Col class="form-check ps-3 mb-0" md="12">
                            <h3>Members</h3>
                            <!-- simple list -->
                            <ul class="list-group">
                                {#each $selectedGroup.users as user}
                                    <li class="list-group-item">
                                        {user.username}
                                    </li>
                                {/each}
                            </ul>
                        </Col>
                    </Row>
                {/if}
            </CardBody>
        </Card>
    </Col>
</Row>
