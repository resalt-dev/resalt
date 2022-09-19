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
        addUserToPermissionGroup,
        createPermissionGroup,
        deletePermissionGroup,
        getPermissionGroups,
        removeUserFromPermissionGroup,
        showAlert,
        updatePermissionGroup,
    } from "../../controller";
    import { AlertType } from "../../models/AlertType";
    import type PermissionGroup from "../../models/PermissionGroup";
    import { theme } from "../../stores";

    let paginationSize: number = 20;
    let paginationPage: number = 1;

    const groups = writable(null);
    const selectedGroup = writable(null);

    let groupNameFieldValue: string = "";
    let groupNameFieldError: boolean = false;
    let groupLdapSyncFieldValue: string = "";
    let groupLdapSyncFieldError: boolean = false;
    let addUserFieldValue: string = "";
    let addUserFieldError: boolean = false;

    function updateData(): Promise<void> {
        return new Promise((resolve, reject) => {
            getPermissionGroups(
                paginationSize,
                (paginationPage - 1) * paginationSize
            )
                .then((data: PermissionGroup[]) => {
                    groups.set(data);

                    if ($selectedGroup === null) {
                        if (data.length > 0) {
                            selectGroup(data[0]);
                        }
                    } else {
                        for (let group of data) {
                            if (group.id === $selectedGroup.id) {
                                selectGroup(group);
                                break;
                            }
                        }
                    }

                    resolve();
                })
                .catch((err) => {
                    showAlert(AlertType.ERROR, "Failed fetching groups", err);
                    reject();
                });
        });
    }

    function selectGroup(group: PermissionGroup): void {
        selectedGroup.set(group);
        groupNameFieldValue = group.name;
        groupNameFieldError = false;
        groupLdapSyncFieldValue = group.ldapSync ?? "";
        groupLdapSyncFieldError = false;
        addUserFieldValue = "";
        addUserFieldError = false;
    }

    function addGroup(): void {
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

    function deleteSelectedGroup(): void {
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

    function addUserToSelectedGroup(): void {
        if ($selectedGroup === null) {
            return;
        }
        validateAddUserField();
        if (addUserFieldError) {
            return;
        }
        addUserToPermissionGroup(addUserFieldValue, $selectedGroup.id)
            .then(() => {
                updateData();
                showAlert(
                    AlertType.SUCCESS,
                    "Add user to group",
                    "Added user to group!"
                );
            })
            .catch((err) => {
                console.error(err);
                showAlert(AlertType.ERROR, "Failed adding user to group", err);
            });
    }

    function removeUserFromSelectedGroup(userId: string): void {
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

    function updateSelectedGroup(): void {
        if ($selectedGroup === null) {
            return;
        }
        validateGroupNameField();
        validateGroupLdapSyncField();
        if (groupNameFieldError || groupLdapSyncFieldError) {
            return;
        }
        updatePermissionGroup(
            $selectedGroup.id,
            groupNameFieldValue,
            $selectedGroup.perms,
            groupLdapSyncFieldValue.length > 0 ? groupLdapSyncFieldValue : null
        )
            .then(() => {
                updateData();
                showAlert(
                    AlertType.SUCCESS,
                    "Update group",
                    "Updated group name!"
                );
            })
            .catch((err) => {
                console.error(err);
                showAlert(AlertType.ERROR, "Failed updating group", err);
            });
    }

    /*
    // VALIDATION
    */

    function validateGroupNameField(): void {
        groupNameFieldError = false;
        if (groupNameFieldValue.length === 0) {
            groupNameFieldError = true;
            return;
        }
        if (groupNameFieldValue === "$superadmins") {
            groupNameFieldError = true;
            return;
        }
    }

    function validateGroupLdapSyncField(): void {
        groupLdapSyncFieldError = false;
        if (groupLdapSyncFieldValue.length === 0) {
            // Allow empty
            return;
        }

        if (!groupLdapSyncFieldValue.toLocaleLowerCase().startsWith("cn=")) {
            groupLdapSyncFieldError = true;
            return;
        }
        // https://stackoverflow.com/a/26492530/2479087
        let regex =
            /^(?:[A-Za-z][\w-]*|\d+(?:\.\d+)*)=(?:#(?:[\dA-Fa-f]{2})+|(?:[^,=\+<>#;\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*|"(?:[^\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*")(?:\+(?:[A-Za-z][\w-]*|\d+(?:\.\d+)*)=(?:#(?:[\dA-Fa-f]{2})+|(?:[^,=\+<>#;\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*|"(?:[^\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*"))*(?:,(?:[A-Za-z][\w-]*|\d+(?:\.\d+)*)=(?:#(?:[\dA-Fa-f]{2})+|(?:[^,=\+<>#;\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*|"(?:[^\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*")(?:\+(?:[A-Za-z][\w-]*|\d+(?:\.\d+)*)=(?:#(?:[\dA-Fa-f]{2})+|(?:[^,=\+<>#;\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*|"(?:[^\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*"))*)*$/;
        if (!regex.test(groupLdapSyncFieldValue)) {
            console.log("Invalid LDAP sync string", groupLdapSyncFieldValue);
            groupLdapSyncFieldError = true;
            return;
        }
    }

    function validateAddUserField(): void {
        addUserFieldError = false;
        if (addUserFieldValue.length !== 40) {
            addUserFieldError = true;
            return;
        }
        if (!addUserFieldValue.startsWith("usr_")) {
            addUserFieldError = true;
            return;
        }
    }

    onMount(() => {
        updateData();
    });
</script>

<Row>
    <Col xs="12" md="4">
        <Card
            class="table-responsive border-bottom-0 mb-3 {$theme.dark
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
                                    selectGroup(group);
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
    <Col xs="12" md="8">
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
                        <Col class="ps-3 mb-0" xs="12">
                            <FormGroup floating={true}>
                                <Input
                                    type="text"
                                    bind:value={$selectedGroup.id}
                                    disabled
                                />
                                <Label for="arguments">Group ID</Label>
                            </FormGroup>
                        </Col>
                        <Col class="ps-3 mb-0" xs="12" lg="6" xxl="5">
                            <FormGroup floating={true}>
                                <Input
                                    type="text"
                                    disabled={$selectedGroup.name ===
                                        "$superadmins"}
                                    invalid={groupNameFieldError}
                                    bind:value={groupNameFieldValue}
                                    on:blur={validateGroupNameField}
                                    required
                                />
                                <Label for="arguments">Group Name</Label>
                            </FormGroup>
                        </Col>
                        <Col class="ps-3 mb-0" xs="12" lg="6" xxl="7">
                            <FormGroup floating={true}>
                                <Input
                                    type="text"
                                    disabled={$selectedGroup.name ===
                                        "$superadmins"}
                                    invalid={groupLdapSyncFieldError}
                                    bind:value={groupLdapSyncFieldValue}
                                    on:blur={validateGroupLdapSyncField}
                                />
                                <Label for="arguments" class="text-muted">
                                    LDAP Sync DN (optional)
                                </Label>
                            </FormGroup>
                        </Col>
                        <Col class="ps-3 mb-0" xs="12">
                            <Button
                                color="primary"
                                class="float-end"
                                disabled={$selectedGroup.name ===
                                    "$superadmins"}
                                on:click={updateSelectedGroup}
                            >
                                Save changes
                            </Button>
                        </Col>
                        <Col class="ps-3 mb-0" xs="12">
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
                                                <Button
                                                    color="danger"
                                                    size="sm"
                                                    class="float-end"
                                                    disabled={$selectedGroup.name ===
                                                        "$superadmins" &&
                                                        user.username ===
                                                            "admin"}
                                                    on:click={() => {
                                                        removeUserFromSelectedGroup(
                                                            user.id
                                                        );
                                                    }}
                                                >
                                                    Remove
                                                </Button>
                                            </td>
                                        </tr>
                                    {/each}
                                </tbody>
                            </Table>
                        </Col>
                        <Col class="ps-3 mb-0" xs="12">
                            <div class="input-group flex-nowrap">
                                <div class="form-floating w-100">
                                    <Input
                                        type="text"
                                        bsSize="sm"
                                        style="height: 2.5rem;"
                                        invalid={addUserFieldError}
                                        bind:value={addUserFieldValue}
                                        on:blur={validateAddUserField}
                                    />
                                    <Label
                                        for="arguments"
                                        style="padding-top: 0.4rem;"
                                        >Add by User ID</Label
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
                            {#if addUserFieldError}
                                <div class="invalid-feedback d-block">
                                    Invalid User ID. Please see the User List
                                    tab.
                                </div>
                            {/if}
                            <div class="mb-3" />
                        </Col>
                        <Col class="ps-3 mb-0" xs="12">
                            <h3>Permissions</h3>
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
                                                    Permission
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
                                                    Description
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
                                    {#each $selectedGroup.perms as permission}
                                        <tr>
                                            <th scope="row">
                                                {permission.name}
                                            </th>
                                            <td>
                                                <small>
                                                    {permission.description}
                                                </small>
                                            </td>
                                            <td>a</td>
                                        </tr>
                                    {/each}
                                </tbody>
                            </Table>
                        </Col>
                        <Col class="ps-3 mb-0" xs="12">
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
