<script lang="ts">
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import {
        Button,
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
    import Icon from "../../components/Icon.svelte";
    import TablePaginate from "../../components/TablePaginate.svelte";
    import {
        createPermissionGroup,
        deletePermissionGroup,
        getPermissionGroups,
        removeUserFromPermissionGroup,
        showAlert,
    } from "../../controller";
    import { AlertType } from "../../models/AlertType";
    import type PermissionGroup from "../../models/PermissionGroup";
    import { theme } from "../../stores";

    let paginationSize: number = 20;
    let paginationPage: number = 1;

    const groups = writable(null);
    const selectedGroup = writable(null);

    function updateData(): Promise<void> {
        return new Promise((resolve, reject) => {
            getPermissionGroups(
                paginationSize,
                (paginationPage - 1) * paginationSize
            )
                .then((data: PermissionGroup[]) => {
                    groups.set(data);
                    if (data.length > 0 && $selectedGroup === null) {
                        selectedGroup.set(data[0]);
                    }
                    resolve();
                })
                .catch((err) => {
                    showAlert(AlertType.ERROR, "Failed fetching groups", err);
                    reject();
                });
        });
    }

    function addGroup() {
        createPermissionGroup("- Temporary Group Name - ")
            .then((group) => {
                updateData();
                selectedGroup.set(group);
                showAlert(AlertType.SUCCESS, "Create group", "Created group!");
            })
            .catch((err) => {
                console.error(err);
                showAlert(AlertType.ERROR, "Failed creating group", err);
            });
    }

    function deleteSelectedGroup() {
        let indexOfCurrentSelected = $groups.findIndex(
            (group) => group.id === $selectedGroup.id
        );
        deletePermissionGroup($selectedGroup.id)
            .then(() => {
                updateData().then(() => {
                    if ($groups.length > 0) {
                        selectedGroup.set(
                            $groups[Math.max(0, indexOfCurrentSelected - 1)]
                        );
                    } else {
                        selectedGroup.set(null);
                    }
                });
                showAlert(AlertType.SUCCESS, "Delete group", "Deleted group!");
            })
            .catch((err) => {
                console.error(err);
                showAlert(AlertType.ERROR, "Failed deleting group", err);
            });
    }

    function addUserToSelectedGroup() {
        if ($selectedGroup === null) {
            return;
        }
        // TODO: Add user to group
    }

    function removeUserFromSelectedGroup(userId: string) {
        if ($selectedGroup === null) {
            return;
        }
        removeUserFromPermissionGroup(userId, $selectedGroup.id)
            .then(() => {
                updateData();
                showAlert(
                    AlertType.SUCCESS,
                    "Remove user from group",
                    "Removed user from group!"
                );
            })
            .catch((err) => {
                console.error(err);
                showAlert(
                    AlertType.ERROR,
                    "Failed removing user from group",
                    err
                );
            });
    }

    function updateSelectedGroup() {}

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
                                        ? "bg-" +
                                          $theme.color +
                                          " border-" +
                                          $theme.color
                                        : ""}
                                >
                                    {group.name}
                                </th>
                                <td
                                    class={$selectedGroup?.id === group.id
                                        ? "bg-" +
                                          $theme.color +
                                          " border-" +
                                          $theme.color
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
                <span class="fw-bold">Group Details</span>
                <Button
                    size="sm"
                    color="success"
                    class="float-end"
                    style="margin-top: -4px;margin-bottom: -4px;"
                    on:click={addGroup}
                >
                    <Icon name="plus" size="1" style="margin-top: -2px;" />
                </Button>
            </CardHeader>
            <CardBody>
                {#if $selectedGroup === null}
                    <h1>Select a group to edit</h1>
                {:else}
                    <Row>
                        <Col class="ps-3 mb-0" md="12">
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
                        <Col class="ps-3 mb-0" md="12">
                            <FormGroup floating={true}>
                                <Input
                                    id="groupName"
                                    type="text"
                                    disabled={$selectedGroup.name ===
                                        "$superadmins"}
                                    bind:value={$selectedGroup.name}
                                />
                                <Label for="arguments">Group Name</Label>
                            </FormGroup>
                        </Col>
                        <Col class="ps-3 mb-0" md="12">
                            <FormGroup floating={true}>
                                <Input
                                    id="groupLdapSync"
                                    type="text"
                                    disabled={$selectedGroup.name ===
                                        "$superadmins"}
                                    bind:value={$selectedGroup.ldapSync}
                                />
                                <Label for="arguments" class="text-muted">
                                    LDAP Sync DN (optional)
                                </Label>
                            </FormGroup>
                        </Col>
                        <Col class="ps-3 mb-0" md="12">
                            <Button
                                color="primary"
                                class="float-end"
                                disabled={$selectedGroup.name ===
                                    "$superadmins"}
                                on:click={deleteSelectedGroup}
                            >
                                Save changes
                            </Button>
                        </Col>
                        <Col class="ps-3 mb-0" md="12">
                            <h3>Members</h3>
                            <Table
                                dark={$theme.dark}
                                class="b-0 mb-3 {$theme.dark
                                    ? 'text-light border-secondary'
                                    : ''}"
                            >
                                <thead
                                    class="bg-dark border-0 {$theme.dark
                                        ? 'text-light'
                                        : 'text-white'}"
                                >
                                    <tr>
                                        <th
                                            scope="col"
                                            class="border-secondary"
                                        >
                                            <div class="row g-1">
                                                <div
                                                    class="col-auto align-self-center ps-2"
                                                >
                                                    User ID
                                                </div>
                                            </div>
                                        </th>
                                        <th
                                            scope="col"
                                            class="border-secondary"
                                        >
                                            <div class="row g-1">
                                                <div
                                                    class="col-auto align-self-center"
                                                >
                                                    Username
                                                </div>
                                            </div>
                                        </th>
                                        <th
                                            scope="col"
                                            class="border-secondary"
                                        />
                                    </tr>
                                </thead>
                                <tbody class="align-middle">
                                    {#each $selectedGroup.users as user}
                                        <tr>
                                            <th scope="row">
                                                {user.username}
                                            </th>
                                            <td>
                                                <small>{user.id}</small>
                                            </td>
                                            <td>
                                                <!-- <Button
                                                    color="danger"
                                                    size="sm"
                                                    class="float-end"
                                                    on:click={() => {
                                                        removeUserFromSelectedGroup(
                                                            user.id
                                                        );
                                                    }}
                                                >
                                                    Remove
                                                </Button> -->
                                            </td>
                                        </tr>
                                    {/each}
                                </tbody>
                            </Table>
                        </Col>
                        <Col class="ps-3 mb-0" md="12">
                            <h3>Add user</h3>
                            <div class="input-group flex-nowrap mb-3">
                                <div class="form-floating w-100">
                                    <Input
                                        id="addUser"
                                        type="text"
                                        bsSize="sm"
                                        style="height: 2.5rem;"
                                    />
                                    <!-- bind:value={$addUser} -->
                                    <Label
                                        for="arguments"
                                        style="padding-top: 0.4rem;"
                                        >User ID</Label
                                    >
                                </div>
                                <Button
                                    color="primary"
                                    class="float-end text-nowrap px-4"
                                    on:click={addUserToSelectedGroup}
                                >
                                    Add user
                                </Button>
                            </div>
                        </Col>
                        <Col class="ps-3 mb-0" md="12">
                            <h3>Actions</h3>
                            <Button
                                color="danger"
                                class="float-end"
                                disabled={$selectedGroup.name ===
                                    "$superadmins"}
                                on:click={deleteSelectedGroup}
                            >
                                Delete Group
                            </Button>
                        </Col>
                    </Row>
                {/if}
            </CardBody>
        </Card>
    </Col>
</Row>
