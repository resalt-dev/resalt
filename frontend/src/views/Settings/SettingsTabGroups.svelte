<script lang="ts">
    import { onMount } from 'svelte';
    import { writable, type Writable } from 'svelte/store';
    import {
        Alert,
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
    } from 'sveltestrap';
    import Icon from '../../components/Icon.svelte';
    import TablePaginate from '../../components/TablePaginate.svelte';
    import {
        addUserToPermissionGroup,
        createPermissionGroup,
        deletePermissionGroup,
        getPermissionGroups,
        removeUserFromPermissionGroup,
        showAlert,
        updatePermissionGroup,
    } from '../../controller';
    import { AlertType } from '../../models/AlertType';
    import { resaltPermissions } from '../../perms';
    import { theme } from '../../stores';
    import type PermissionGroup from '../../models/PermissionGroup';

    let paginationSize: number = 20;
    let paginationPage: number = 1;

    const groups = writable<PermissionGroup[]>(null);
    const selectedGroup = writable<PermissionGroup | null>(null);

    type PermissionMinionTargetModule = {
        moduleId: string;
        name: string;
        args: string[];
    };
    type PermissionMinionTarget = {
        targetId: string;
        target: string;
        modules: PermissionMinionTargetModule[];
    };

    let groupNameFieldValue: string = '';
    let groupNameFieldError: boolean = false;
    let groupLdapSyncFieldValue: string = '';
    let groupLdapSyncFieldError: boolean = false;
    let addUserFieldValue: string = '';
    let addUserFieldError: boolean = false;
    let permissionWebFields: { [key: string]: boolean } = {};
    const permissionMinionsFields: Writable<PermissionMinionTarget[]> =
        writable<PermissionMinionTarget[]>([]);

    function updateData(): Promise<void> {
        return new Promise((resolve, reject) => {
            getPermissionGroups(
                paginationSize,
                (paginationPage - 1) * paginationSize,
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
                    showAlert(AlertType.ERROR, 'Failed fetching groups', err);
                    reject();
                });
        });
    }

    // Generate LONG random string
    // NOT cryptographically secure, only used to identify UI nodes.
    function randomId(): string {
        return (
            Math.random().toString(36).substring(2, 15) +
            Math.random().toString(36).substring(2, 15)
        );
    }

    function selectGroup(group: PermissionGroup): void {
        selectedGroup.set(group);
        groupNameFieldValue = group.name;
        groupNameFieldError = false;
        groupLdapSyncFieldValue = group.ldapSync ?? '';
        groupLdapSyncFieldError = false;
        addUserFieldValue = '';
        addUserFieldError = false;
        permissionWebFields = {};
        for (let perm of resaltPermissions) {
            permissionWebFields[perm[0]] = group.hasResaltPermission(perm[0]);
        }
        let minionPerms: PermissionMinionTarget[] = [];
        for (let minionPermissionRaw of group.perms.filter(
            // Filter Resalt web permissions so they don't show up twice
            (perm) =>
                (typeof perm === 'object' && !('@resalt' in perm)) ||
                typeof perm === 'string',
        )) {
            let targetName = '';
            let targetModules: PermissionMinionTargetModule[] = [];
            if (typeof minionPermissionRaw === 'string') {
                targetName = minionPermissionRaw;
            } else {
                targetName = Object.keys(minionPermissionRaw)[0];
                let modulesRaw = minionPermissionRaw[targetName]; // array
                if (!Array.isArray(modulesRaw)) {
                    console.error('Invalid permission parsing.', modulesRaw);
                    continue;
                }

                for (let moduleRaw of modulesRaw) {
                    let moduleName = '';
                    let moduleArgs = [];
                    if (typeof moduleRaw === 'string') {
                        moduleName = moduleRaw;
                    } else {
                        moduleName = Object.keys(moduleRaw)[0];
                        moduleArgs = moduleRaw[moduleName]['args'] ?? [];
                    }
                    targetModules.push({
                        moduleId: randomId(),
                        name: moduleName,
                        args: [...moduleArgs],
                    });
                }
            }
            minionPerms.push({
                targetId: randomId(),
                target: targetName,
                modules: targetModules,
            });
        }
        permissionMinionsFields.set(minionPerms);
    }

    function addGroup(): void {
        createPermissionGroup('- Temporary Group Name - ')
            .then((group) => {
                updateData();
                selectedGroup.set(group);
                showAlert(AlertType.SUCCESS, 'Create group', 'Created group!');
            })
            .catch((err) => {
                console.error(err);
                showAlert(AlertType.ERROR, 'Failed creating group', err);
            });
    }

    function deleteSelectedGroup(): void {
        let indexOfCurrentSelected = $groups.findIndex(
            (group) => group.id === $selectedGroup.id,
        );
        deletePermissionGroup($selectedGroup.id)
            .then(() => {
                updateData().then(() => {
                    if ($groups.length > 0) {
                        selectedGroup.set(
                            $groups[Math.max(0, indexOfCurrentSelected - 1)],
                        );
                    } else {
                        selectedGroup.set(null);
                    }
                });
                showAlert(AlertType.SUCCESS, 'Delete group', 'Deleted group!');
            })
            .catch((err) => {
                console.error(err);
                showAlert(AlertType.ERROR, 'Failed deleting group', err);
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
                    'Add user to group',
                    'Added user to group!',
                );
            })
            .catch((err) => {
                console.error(err);
                showAlert(AlertType.ERROR, 'Failed adding user to group', err);
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
                    'Remove user from group',
                    'Removed user from group!',
                );
            })
            .catch((err) => {
                console.error(err);
                showAlert(
                    AlertType.ERROR,
                    'Failed removing user from group',
                    err,
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
            groupLdapSyncFieldValue.length > 0 ? groupLdapSyncFieldValue : null,
        )
            .then(() => {
                updateData();
                showAlert(
                    AlertType.SUCCESS,
                    'Update group',
                    'Updated group name!',
                );
            })
            .catch((err) => {
                console.error(err);
                showAlert(AlertType.ERROR, 'Failed updating group', err);
            });
    }

    function localAddMinionTarget(): void {
        permissionMinionsFields.update((minions) => {
            minions.push({
                targetId: randomId(),
                target: '',
                modules: [],
            });
            return minions;
        });
    }

    function localAddMinionTargetModule(targetId: string): void {
        permissionMinionsFields.update((minions) => {
            let target = minions.find((target) => target.targetId === targetId);
            if (target === undefined) {
                return minions;
            }
            target.modules.push({
                moduleId: randomId(),
                name: '',
                args: [],
            });
            return minions;
        });
    }

    function localAddMinionTargetModuleArg(
        targetId: string,
        moduleId: string,
    ): void {
        permissionMinionsFields.update((minions) => {
            let target = minions.find((target) => target.targetId === targetId);
            if (target === undefined) {
                return minions;
            }
            let module = target.modules.find(
                (module) => module.moduleId === moduleId,
            );
            if (module === undefined) {
                return minions;
            }
            module.args.push('');
            return minions;
        });
    }

    function localRemoveMinionTarget(targetId: string): void {
        permissionMinionsFields.update((minions) => {
            return minions.filter((target) => target.targetId !== targetId);
        });
    }

    function localRemoveMinionTargetModule(
        targetId: string,
        moduleId: string,
    ): void {
        permissionMinionsFields.update((minions) => {
            let target = minions.find((target) => target.targetId === targetId);
            if (target === undefined) {
                return minions;
            }
            target.modules = target.modules.filter(
                (module) => module.moduleId !== moduleId,
            );
            return minions;
        });
    }

    function localRemoveMinionTargetModuleArg(
        targetId: string,
        moduleId: string,
        argNum: number,
    ): void {
        permissionMinionsFields.update((minions) => {
            let target = minions.find((target) => target.targetId === targetId);
            if (target === undefined) {
                return minions;
            }
            let module = target.modules.find(
                (module) => module.moduleId === moduleId,
            );
            if (module === undefined) {
                return minions;
            }
            module.args = module.args.filter((_arg, index) => index !== argNum);
            return minions;
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
        if (groupNameFieldValue === '$superadmins') {
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

        if (!groupLdapSyncFieldValue.toLocaleLowerCase().startsWith('cn=')) {
            groupLdapSyncFieldError = true;
            return;
        }
        // https://stackoverflow.com/a/26492530/2479087
        let regex =
            /^(?:[A-Za-z][\w-]*|\d+(?:\.\d+)*)=(?:#(?:[\dA-Fa-f]{2})+|(?:[^,=\+<>#;\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*|"(?:[^\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*")(?:\+(?:[A-Za-z][\w-]*|\d+(?:\.\d+)*)=(?:#(?:[\dA-Fa-f]{2})+|(?:[^,=\+<>#;\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*|"(?:[^\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*"))*(?:,(?:[A-Za-z][\w-]*|\d+(?:\.\d+)*)=(?:#(?:[\dA-Fa-f]{2})+|(?:[^,=\+<>#;\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*|"(?:[^\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*")(?:\+(?:[A-Za-z][\w-]*|\d+(?:\.\d+)*)=(?:#(?:[\dA-Fa-f]{2})+|(?:[^,=\+<>#;\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*|"(?:[^\\"]|\\[,=\+<>#;\\"]|\\[\dA-Fa-f]{2})*"))*)*$/;
        if (!regex.test(groupLdapSyncFieldValue)) {
            console.log('Invalid LDAP sync string', groupLdapSyncFieldValue);
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
        if (!addUserFieldValue.startsWith('usr_')) {
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
                        <th class="border-secondary">
                            <div class="row g-1">
                                <div class="col-auto align-self-center ps-2">
                                    Group Name
                                </div>
                            </div>
                        </th>
                        <th class="border-secondary">
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
                                    class={$selectedGroup?.id === group.id
                                        ? 'bg-' +
                                          $theme.color +
                                          ' border-' +
                                          $theme.color
                                        : ''}
                                >
                                    {group.name}
                                </th>
                                <td
                                    class={$selectedGroup?.id === group.id
                                        ? 'bg-' +
                                          $theme.color +
                                          ' border-' +
                                          $theme.color
                                        : ''}
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
        <Card class={$theme.dark ? 'bg-dark' : ''}>
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
                            {#if $selectedGroup.name === '$superadmins'}
                                <Alert
                                    color="warning"
                                    dismissible={false}
                                    fade={false}
                                >
                                    <strong>Warning!</strong> You have selected
                                    the "<strong>$superadmins</strong>" group.
                                    This is a special system-protected group
                                    that cannot be edited or deleted.
                                </Alert>
                            {/if}
                        </Col>
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
                                        '$superadmins'}
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
                                        '$superadmins'}
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
                                    '$superadmins'}
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
                                        <th class="border-secondary">
                                            User ID
                                        </th>
                                        <th class="border-secondary">
                                            Username
                                        </th>
                                        <th class="border-secondary" />
                                    </tr>
                                </thead>
                                <tbody class="align-middle">
                                    {#each $selectedGroup.users as user}
                                        <tr>
                                            <th>
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
                                                        '$superadmins' ||
                                                        groupLdapSyncFieldValue.length >
                                                            0}
                                                    on:click={() => {
                                                        removeUserFromSelectedGroup(
                                                            user.id,
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
                                        disabled={$selectedGroup.name ===
                                            '$superadmins' ||
                                            groupLdapSyncFieldValue.length > 0}
                                        bind:value={addUserFieldValue}
                                        on:blur={validateAddUserField}
                                    />
                                    <Label
                                        for="arguments"
                                        style="padding-top: 0.4rem;"
                                    >
                                        {#if groupLdapSyncFieldValue.length > 0}
                                            Manually managing users is disabled
                                            because LDAP Sync is active.
                                        {:else}
                                            Add by User ID
                                        {/if}
                                    </Label>
                                </div>
                                <Button
                                    color="primary"
                                    class="float-end text-nowrap px-4"
                                    disabled={$selectedGroup.name ===
                                        '$superadmins' ||
                                        groupLdapSyncFieldValue.length > 0}
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
                            <h5>Web Dashboard</h5>
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
                                        <th class="border-secondary" />
                                        <th class="border-secondary ps-0">
                                            Permission
                                        </th>
                                        <th class="border-secondary">
                                            Description
                                        </th>
                                    </tr>
                                </thead>
                                <tbody class="align-middle">
                                    {#each resaltPermissions as resaltPermission}
                                        <tr>
                                            <td class="px-5" style="width: 0">
                                                <input
                                                    type="checkbox"
                                                    class="form-check-input form-check-input-primary fs-3 ms-0 mt-0"
                                                    disabled={$selectedGroup.name ===
                                                        '$superadmins'}
                                                    bind:checked={permissionWebFields[
                                                        resaltPermission[0]
                                                    ]}
                                                />
                                            </td>
                                            <th class="ps-0">
                                                {resaltPermission[1]}
                                            </th>
                                            <td>
                                                <small>
                                                    {resaltPermission[2]}
                                                </small>
                                            </td>
                                        </tr>
                                    {/each}
                                </tbody>
                            </Table>
                            <h5>Minion Targets</h5>
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
                                        <th class="border-secondary ps-3">
                                            Target
                                        </th>
                                        <th class="border-secondary ps-3">
                                            Module
                                        </th>
                                        <th class="border-secondary ps-3">
                                            Arguments
                                        </th>
                                        <td class="border-secondary">
                                            <Button
                                                size="sm"
                                                color="success"
                                                class="float-end"
                                                style="margin-top: -4px;margin-bottom: -4px;"
                                                disabled={$selectedGroup.name ===
                                                    '$superadmins'}
                                                on:click={localAddMinionTarget}
                                                ><Icon
                                                    name="plus"
                                                    size="1"
                                                    style="margin-top: -2px;"
                                                /></Button
                                            >
                                        </td>
                                    </tr>
                                </thead>
                                <tbody>
                                    {#each $permissionMinionsFields as minionTarget}
                                        <tr>
                                            <td style="width: 12rem;">
                                                <div
                                                    class="input-group flex-nowrap"
                                                >
                                                    <div class="form-floating">
                                                        <Input
                                                            type="text"
                                                            bsSize="sm"
                                                            style="height: 2.5rem;"
                                                            disabled={$selectedGroup.name ===
                                                                '$superadmins'}
                                                            bind:value={minionTarget.target}
                                                        />
                                                        <Label
                                                            for="arguments"
                                                            style="padding-top: 0.4rem;"
                                                        >
                                                            Target
                                                        </Label>
                                                    </div>
                                                    <Button
                                                        size="sm"
                                                        color="success"
                                                        class="float-end"
                                                        disabled={$selectedGroup.name ===
                                                            '$superadmins'}
                                                        on:click={() => {
                                                            localAddMinionTargetModule(
                                                                minionTarget.targetId,
                                                            );
                                                        }}
                                                    >
                                                        <Icon
                                                            name="plus"
                                                            size="1"
                                                            style="margin-top: -2px;"
                                                        />
                                                    </Button>
                                                </div>
                                            </td>
                                            <td style="width: 12rem;">
                                                {#each minionTarget.modules as module, mi}
                                                    {#if mi > 0}
                                                        <hr
                                                            class="text-light my-2"
                                                        />
                                                    {/if}
                                                    <div
                                                        class="input-group flex-nowrap"
                                                    >
                                                        <div
                                                            class="form-floating"
                                                        >
                                                            <Input
                                                                type="text"
                                                                bsSize="sm"
                                                                style="height: 2.5rem;"
                                                                disabled={$selectedGroup.name ===
                                                                    '$superadmins'}
                                                                bind:value={module.name}
                                                            />
                                                            <Label
                                                                for="arguments"
                                                                style="padding-top: 0.4rem;"
                                                            >
                                                                Module
                                                            </Label>
                                                        </div>
                                                        <Button
                                                            size="sm"
                                                            color="danger"
                                                            class="float-end"
                                                            disabled={$selectedGroup.name ===
                                                                '$superadmins'}
                                                            on:click={() => {
                                                                localRemoveMinionTargetModule(
                                                                    minionTarget.targetId,
                                                                    module.moduleId,
                                                                );
                                                            }}
                                                        >
                                                            <Icon
                                                                name="x"
                                                                size="1"
                                                                style="margin-top: -2px;"
                                                            />
                                                        </Button>
                                                    </div>
                                                {/each}
                                            </td>
                                            <td>
                                                {#each minionTarget.modules as module, mi}
                                                    {#if mi > 0}
                                                        <hr
                                                            class="text-light my-2"
                                                        />
                                                    {/if}
                                                    <div
                                                        class="input-group flex-nowrap"
                                                    >
                                                        {#each module.args as arg, ai}
                                                            <div
                                                                class="form-floating"
                                                            >
                                                                <Input
                                                                    type="text"
                                                                    bsSize="sm"
                                                                    style="height: 2.5rem; max-width: 7rem;"
                                                                    disabled={$selectedGroup.name ===
                                                                        '$superadmins'}
                                                                    bind:value={arg}
                                                                />
                                                                <Label
                                                                    for="arguments"
                                                                    style="padding-top: 0.4rem;"
                                                                >
                                                                    Arg {ai}
                                                                </Label>
                                                            </div>
                                                            <Icon
                                                                name="x"
                                                                size="1.5"
                                                                class="mouse-pointer my-2 ms-1 me-3 {$selectedGroup.name ===
                                                                '$superadmins'
                                                                    ? 'text-muted'
                                                                    : 'text-danger'}"
                                                                on:click={() => {
                                                                    if (
                                                                        $selectedGroup.name ===
                                                                        '$superadmins'
                                                                    )
                                                                        return;
                                                                    localRemoveMinionTargetModuleArg(
                                                                        minionTarget.targetId,
                                                                        module.moduleId,
                                                                        ai,
                                                                    );
                                                                }}
                                                            />
                                                        {/each}
                                                        <Icon
                                                            name="plus"
                                                            size="1.5"
                                                            class="mouse-pointer my-2 ms-3 {$selectedGroup.name ===
                                                            '$superadmins'
                                                                ? 'text-muted'
                                                                : 'text-success'}"
                                                            on:click={() => {
                                                                if (
                                                                    $selectedGroup.name ===
                                                                    '$superadmins'
                                                                )
                                                                    return;
                                                                localAddMinionTargetModuleArg(
                                                                    minionTarget.targetId,
                                                                    module.moduleId,
                                                                );
                                                            }}
                                                        />
                                                    </div>
                                                {/each}
                                            </td>
                                            <td>
                                                <Button
                                                    size="sm"
                                                    color="danger"
                                                    style="height: 2rem;"
                                                    class="float-end mt-1"
                                                    disabled={$selectedGroup.name ===
                                                        '$superadmins'}
                                                    on:click={() => {
                                                        localRemoveMinionTarget(
                                                            minionTarget.targetId,
                                                        );
                                                    }}
                                                >
                                                    <Icon
                                                        name="x"
                                                        size="1"
                                                        style="margin-top: -2px;"
                                                    />
                                                </Button>
                                            </td>
                                        </tr>
                                    {/each}
                                </tbody>
                            </Table>
                        </Col>
                        <Col class="ps-3 mb-0" xs="12">
                            <h3>Actions</h3>
                            <Button
                                color="primary"
                                disabled={$selectedGroup.name ===
                                    '$superadmins'}
                                on:click={updateSelectedGroup}
                            >
                                Save changes
                            </Button>
                            <Button
                                color="danger"
                                class="float-end"
                                disabled={$selectedGroup.name ===
                                    '$superadmins'}
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
