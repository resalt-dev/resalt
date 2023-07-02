<script lang="ts">
	import { onMount } from 'svelte';
	import { writable, type Writable } from 'svelte/store';
	import Icon from '../../../components/Icon.svelte';
	import TablePaginate from '../../../components/TablePaginate.svelte';
	import {
		addUserToPermissionGroup,
		createPermissionGroup,
		deletePermissionGroup,
		getPermissionGroups,
		removeUserFromPermissionGroup,
		updatePermissionGroup,
	} from '$lib/api';
	import { MessageType } from '../../../models/MessageType';
	import { resaltWebPermissions } from '$lib/perms';
	import { theme, toasts } from '$lib/stores';
	import type PermissionGroup from '../../../models/PermissionGroup';
	import ResaltProgress from '../../../components/ResaltProgress.svelte';
	import { validateLdapDN } from '$lib/utils';
	import { v4 as uuidv4 } from 'uuid';

	let paginationSize = 20;
	let paginationPage = 1;

	const groups: Writable<PermissionGroup[] | null> = writable(null);
	const selectedGroup: Writable<PermissionGroup | null> = writable(null);

	type PermissionMinionTargetModule = {
		moduleId: string;
		name: string;
		args: string[];
		error: boolean;
	};
	type PermissionMinionTarget = {
		targetId: string;
		target: string;
		modules: PermissionMinionTargetModule[];
		error: boolean;
	};

	let groupNameFieldValue = '';
	let groupNameFieldError = false;
	let groupLdapSyncFieldValue = '';
	let groupLdapSyncFieldError = false;
	let addUserFieldValue = '';
	let addUserFieldError = false;
	let permissionWebFields: { [key: string]: boolean } = {};
	const permissionMinionsFields: Writable<PermissionMinionTarget[]> = writable([]);
	let permissionMinionsFieldsError = false;

	function updateData(): Promise<void> {
		return new Promise((resolve, reject) => {
			getPermissionGroups(paginationSize, (paginationPage - 1) * paginationSize)
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
					toasts.add(MessageType.ERROR, 'Failed fetching groups', err);
					reject();
				});
		});
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
		for (let perm of resaltWebPermissions) {
			permissionWebFields[perm.permission] = group.hasResaltPermission(perm.permission);
		}
		let minionPerms: PermissionMinionTarget[] = [];
		for (let minionPermissionRaw of group.perms.filter(
			// Filter Resalt web permissions so they don't show up twice
			(perm) =>
				(typeof perm === 'object' && !('@resalt' in perm)) || typeof perm === 'string',
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
						moduleId: uuidv4(),
						name: moduleName,
						args: [...moduleArgs],
						error: false,
					});
				}
			}
			minionPerms.push({
				targetId: uuidv4(),
				target: targetName,
				modules: targetModules,
				error: false,
			});
		}
		permissionMinionsFields.set(minionPerms);
	}

	function addGroup(): void {
		createPermissionGroup('- Temporary Group Name - ')
			.then((group) => {
				updateData();
				selectedGroup.set(group);
				toasts.add(MessageType.SUCCESS, 'Create group', 'Created group!');
			})
			.catch((err) => {
				console.error(err);
				toasts.add(MessageType.ERROR, 'Failed creating group', err);
			});
	}

	function deleteSelectedGroup(): void {
		const selected = $selectedGroup;
		if (selected === null) {
			return;
		}
		const groups_local = $groups;
		if (groups_local === null) {
			return;
		}
		let indexOfCurrentSelected = groups_local.findIndex((group) => group.id === selected.id);
		deletePermissionGroup(selected.id)
			.then(() => {
				updateData().then(() => {
					if (groups_local.length > 0) {
						selectedGroup.set(groups_local[Math.max(0, indexOfCurrentSelected - 1)]);
					} else {
						selectedGroup.set(null);
					}
				});
				toasts.add(MessageType.SUCCESS, 'Delete group', 'Deleted group!');
			})
			.catch((err) => {
				console.error(err);
				toasts.add(MessageType.ERROR, 'Failed deleting group', err);
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
				toasts.add(MessageType.SUCCESS, 'Add user to group', 'Added user to group!');
			})
			.catch((err) => {
				console.error(err);
				toasts.add(MessageType.ERROR, 'Failed adding user to group', err);
			});
	}

	function removeUserFromSelectedGroup(userId: string): void {
		if ($selectedGroup === null) {
			return;
		}
		removeUserFromPermissionGroup(userId, $selectedGroup.id)
			.then(() => {
				updateData();
				toasts.add(
					MessageType.SUCCESS,
					'Remove user from group',
					'Removed user from group!',
				);
			})
			.catch((err) => {
				console.error(err);
				toasts.add(MessageType.ERROR, 'Failed removing user from group', err);
			});
	}

	function updateSelectedGroup(): void {
		if ($selectedGroup === null) {
			return;
		}

		validateGroupNameField();
		validateGroupLdapSyncField();
		validatePermissionMinionTargetsFields();
		if (groupNameFieldError || groupLdapSyncFieldError || permissionMinionsFieldsError) {
			return;
		}

		let perms: any[] = localCalculateSaltPermissions();
		console.log('perms', perms);
		updatePermissionGroup(
			$selectedGroup.id,
			groupNameFieldValue,
			perms,
			groupLdapSyncFieldValue.length > 0 ? groupLdapSyncFieldValue : null,
		)
			.then(() => {
				updateData();
				toasts.add(MessageType.SUCCESS, 'Update group', 'Updated group!');
			})
			.catch((err) => {
				console.error(err);
				toasts.add(MessageType.ERROR, 'Failed updating group', err);
			});
	}

	function localCalculateSaltPermissions(): any[] {
		let perms = [];

		// Resalt web permissions
		// { "@resalt": [ webPerms ] }
		let webPerms = [];
		for (let perm of Object.keys(permissionWebFields)) {
			if (permissionWebFields[perm]) {
				webPerms.push(perm);
			}
		}
		if (webPerms.length > 0) {
			perms.push({ '@resalt': webPerms });
		}

		// Resalt minion permissions
		// Can be pushed as either a string or an object
		for (let minion of $permissionMinionsFields) {
			if (minion.modules.length === 0) {
				perms.push(minion.target);
			} else {
				let minionModules = [];
				for (let minionModule of minion.modules) {
					if (minionModule.args.length === 0) {
						minionModules.push(minionModule.name);
					} else {
						minionModules.push({
							[minionModule.name]: { args: minionModule.args },
						});
					}
				}
				perms.push({ [minion.target]: minionModules });
			}
		}
		return perms;
	}

	function localAddMinionTarget(): void {
		permissionMinionsFields.update((minions) => {
			minions.push({
				targetId: uuidv4(),
				target: '',
				modules: [],
				error: false,
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
				moduleId: uuidv4(),
				name: '',
				args: [],
				error: false,
			});
			return minions;
		});
	}

	function localAddMinionTargetModuleArg(targetId: string, moduleId: string): void {
		permissionMinionsFields.update((minions) => {
			let target = minions.find((target) => target.targetId === targetId);
			if (target === undefined) {
				return minions;
			}
			let minionModule = target.modules.find(
				(minionModule) => minionModule.moduleId === moduleId,
			);
			if (minionModule === undefined) {
				return minions;
			}
			minionModule.args.push('');
			return minions;
		});
	}

	function localRemoveMinionTarget(targetId: string): void {
		permissionMinionsFields.update((minions) => {
			return minions.filter((target) => target.targetId !== targetId);
		});
	}

	function localRemoveMinionTargetModule(targetId: string, moduleId: string): void {
		permissionMinionsFields.update((minions) => {
			let target = minions.find((target) => target.targetId === targetId);
			if (target === undefined) {
				return minions;
			}
			target.modules = target.modules.filter(
				(minionModule) => minionModule.moduleId !== moduleId,
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
			let minionModule = target.modules.find(
				(minionModule) => minionModule.moduleId === moduleId,
			);
			if (minionModule === undefined) {
				return minions;
			}
			minionModule.args = minionModule.args.filter((_arg, index) => index !== argNum);
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

		if (!validateLdapDN(groupLdapSyncFieldValue)) {
			console.log('Invalid LDAP sync string', groupLdapSyncFieldValue);
			groupLdapSyncFieldError = true;
			return;
		}
	}

	function validateAddUserField(): void {
		addUserFieldValue = addUserFieldValue.trim();
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

	function validatePermissionMinionTargetsFields(): void {
		let changed = false;
		permissionMinionsFieldsError = false;

		// Clear: Set all errors to false
		for (let minion of $permissionMinionsFields) {
			if (minion.error) {
				minion.error = false;
				changed = true;
			}
			for (let minionModule of minion.modules) {
				if (minionModule.error) {
					minionModule.error = false;
					changed = true;
				}
			}
		}

		// Loop over all minion targets
		for (let minion of $permissionMinionsFields) {
			// Check if any target starts with @resalt ignorecase
			if (minion.target.toLocaleLowerCase().startsWith('@resalt')) {
				permissionMinionsFieldsError = true;
				minion.error = true;
				changed = true;
			}
			// Check if target is empty
			if (minion.target.length === 0) {
				permissionMinionsFieldsError = true;
				minion.error = true;
				changed = true;
			}

			// Check if value is '*', which should be '.*'
			if (minion.target === '*') {
				permissionMinionsFieldsError = true;
				minion.error = true;
				changed = true;
			}

			// Loop over all modules
			for (let minionModule of minion.modules) {
				// Check if length is 0
				if (minionModule.name.length === 0) {
					permissionMinionsFieldsError = true;
					minionModule.error = true;
					changed = true;
				}

				// Check if value is '*', which should be '.*'
				if (minionModule.name === '*') {
					permissionMinionsFieldsError = true;
					minionModule.error = true;
					changed = true;
				}

				// Check if any module arg starts with @resalt ignorecase
				// No checks currently.
				// for (let arg of minionModule.args) {
				// 	// Args can actually be empty, to force empty argument, rather than emply which allow all arguments.
				// }
			}
		}

		if (changed) {
			// Force refresh the mutated state
			permissionMinionsFields.update((minions) => {
				return minions;
			});
		}
	}

	onMount(() => {
		updateData();
	});
</script>

<div class="row">
	<div class="col-12 col-md-4">
		<div class="table-responsive border-bottom-0 mb-3">
			<table class="table b-0 mb-0">
				<thead class="bg-dark border-0 text-white">
					<tr>
						<th class="border-secondary">
							<div class="row g-1">
								<div class="col-auto align-self-center ps-2">Group Name</div>
							</div>
						</th>
						<th class="border-secondary">
							<div class="row g-1">
								<div class="col-auto align-self-center">Members</div>
							</div>
						</th>
					</tr>
				</thead>
				<tbody class="align-middle">
					{#if $groups}
						{#each $groups as group}
							<tr
								class="mouse-pointer {$selectedGroup?.id === group.id
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
										  $theme.color +
										  ' text-' +
										  ($theme.color === 'yellow' ? 'black' : 'white')
										: ''}
								>
									{group.name}
								</th>
								<td
									class={$selectedGroup?.id === group.id
										? 'bg-' +
										  $theme.color +
										  ' border-' +
										  $theme.color +
										  ' text-' +
										  ($theme.color === 'yellow' ? 'black' : 'white')
										: ''}
								>
									{group.users.length}
								</td>
							</tr>
						{/each}
					{/if}
				</tbody>
			</table>
			<TablePaginate
				bind:size={paginationSize}
				bind:page={paginationPage}
				last={$groups === null || $groups.length < paginationSize}
				{updateData}
			/>
			{#if !$groups}
				<ResaltProgress />
			{/if}
		</div>
	</div>
	<div class="col-12 col-md-8">
		<div class="card">
			<div class="card-header">
				Group Details
				<button
					type="button"
					class="btn btn-sm btn-success float-end"
					style="margin-top: -4px;margin-bottom: -4px;"
					on:click={addGroup}
				>
					<Icon name="plus" size="1" style="margin-top: -2px;" />
				</button>
			</div>
			<div class="card-body">
				{#if $selectedGroup === null}
					<h1>Select a group to edit</h1>
				{:else}
					<div class="row">
						<div class="col-12 ps-3 mb-0">
							{#if $selectedGroup.name === '$superadmins'}
								<div class="alert alert-warning" role="alert">
									<strong>Warning!</strong> You have selected the "<strong
										>$superadmins</strong
									>" group. This is a special system-protected group that cannot
									be edited or deleted.
								</div>
							{/if}
						</div>
						<div class="col-12 ps-3 mb-0">
							<div class="form-floating mb-3">
								<input
									id="selectedGroupId"
									type="text"
									class="form-control"
									bind:value={$selectedGroup.id}
									disabled
								/>
								<label class="form-label" for="selectedGroupId">Group ID</label>
							</div>
						</div>
						<div class="col-12 col-lg-6 col-xxl-5 ps-3 mb-0">
							<div class="form-floating mb-3">
								<input
									id="selectedGroupName"
									type="text"
									class="form-control {groupNameFieldError ? 'is-invalid' : ''}"
									disabled={$selectedGroup.name === '$superadmins'}
									bind:value={groupNameFieldValue}
									on:blur={validateGroupNameField}
									required
								/>
								<label class="form-label" for="selectedGroupName">Group Name</label>
							</div>
						</div>
						<div class="col-12 col-lg-6 col-xxl-7 ps-3 mb-0">
							<div class="form-floating mb-3">
								<input
									id="selectedGroupLdapSync"
									type="text"
									class="form-control {groupLdapSyncFieldError
										? 'is-invalid'
										: ''}"
									disabled={$selectedGroup.name === '$superadmins'}
									bind:value={groupLdapSyncFieldValue}
									on:blur={validateGroupLdapSyncField}
								/>
								<label class="form-label" for="selectedGroupLdapSync"
									>LDAP Sync DN (optional)</label
								>
							</div>
						</div>
						<div class="col-12 ps-3 mb-0">
							<button
								type="button"
								class="btn btn-primary float-end"
								disabled={$selectedGroup.name === '$superadmins'}
								on:click={updateSelectedGroup}
							>
								Save
							</button>
						</div>
						<div class="col-12 ps-3 mb-0">
							<h3>Members</h3>
							<table class="table b-0 mb-3">
								<thead class="bg-dark border-0 text-white">
									<tr>
										<th class="border-secondary"> User ID </th>
										<th class="border-secondary"> Username </th>
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
												<button
													type="button"
													class="btn btn-sm btn-danger float-end"
													disabled={$selectedGroup.name ===
														'$superadmins' ||
														groupLdapSyncFieldValue.length > 0}
													on:click={() => {
														removeUserFromSelectedGroup(user.id);
													}}
												>
													Remove
												</button>
											</td>
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
						<div class="col-12 ps-3 mb-0">
							<div class="input-group input-group-sm flex-nowrap">
								<div class="form-floating w-100">
									<input
										id="addUserField"
										type="text"
										class="form-control {addUserFieldError ? 'is-invalid' : ''}"
										style="height: 2.5rem;"
										disabled={$selectedGroup.name === '$superadmins' ||
											groupLdapSyncFieldValue.length > 0}
										bind:value={addUserFieldValue}
										on:blur={validateAddUserField}
									/>
									<label
										class="form-label"
										for="addUserField"
										style="padding-top: 0.4rem;"
									>
										{#if groupLdapSyncFieldValue.length > 0}
											Manually managing users is disabled because LDAP Sync is
											active.
										{:else}
											Add by User ID
										{/if}
									</label>
								</div>
								<button
									type="button"
									class="btn btn-primary float-end text-nowrap px-4"
									disabled={$selectedGroup.name === '$superadmins' ||
										groupLdapSyncFieldValue.length > 0}
									on:click={addUserToSelectedGroup}
								>
									Add user
								</button>
							</div>
							{#if addUserFieldError}
								<div class="invalid-feedback d-block">
									Invalid User ID. Please see the User List tab.
								</div>
							{/if}
							<div class="mb-3" />
						</div>
						<div class="col-12 ps-3 mb-0">
							<h3>Permissions</h3>
							<h5>Web Dashboard</h5>
							<table class="table b-0 mb-3">
								<thead class="bg-dark border-0 text-white">
									<tr>
										<th class="border-secondary" />
										<th class="border-secondary ps-0"> Permission </th>
										<th class="border-secondary"> Description </th>
									</tr>
								</thead>
								<tbody class="align-middle">
									{#each resaltWebPermissions as resaltPermission}
										<tr>
											<td
												class="px-5"
												style="width: fit-content; padding-right: 0.25rem !important;"
											>
												<input
													type="checkbox"
													class="form-check-input form-check-input-primary fs-3 ms-0 mt-0"
													disabled={$selectedGroup.name ===
														'$superadmins'}
													bind:checked={permissionWebFields[
														resaltPermission.permission
													]}
												/>
												{#if resaltPermission.danger}
													<Icon
														name="shield-quarter"
														class="ms-4 text-danger"
														size="1.75"
														tooltip="This is a DANGEROUS permission and may cause SEVERE damage if abused."
													/>
												{/if}
												{#if resaltPermission.warning}
													<Icon
														name="shield-quarter"
														class="ms-4 text-warning"
														size="1.75"
														tooltip="This is a WARNING permission and may cause MEDIUM damage if abused."
													/>
												{/if}</td
											>
											<th class="ps-0">
												{resaltPermission.title}
											</th>
											<td>
												<small>
													{resaltPermission.description}
												</small>
											</td>
										</tr>
									{/each}
								</tbody>
							</table>
							<h5>Minion Targets</h5>
							<table class="table b-0 mb-3">
								<thead class="bg-dark border-0 text-white">
									<tr>
										<th class="border-secondary ps-3"> Target </th>
										<th class="border-secondary ps-3"> Module </th>
										<th class="border-secondary ps-3"> Arguments </th>
										<td class="border-secondary">
											<button
												type="button"
												class="btn btn-sm btn-success float-end"
												style="margin-top: -4px;margin-bottom: -4px;"
												disabled={$selectedGroup.name === '$superadmins'}
												on:click={localAddMinionTarget}
											>
												<Icon
													name="plus"
													size="1"
													style="margin-top: -2px;"
												/>
											</button>
										</td>
									</tr>
								</thead>
								<tbody>
									{#each $permissionMinionsFields as minionTarget}
										<tr>
											<td style="width: 12rem;">
												<div class="input-group input-group-sm flex-nowrap">
													<div class="form-floating">
														<input
															id="minionTarget_{minionTarget.targetId}"
															type="text"
															class="form-control {minionTarget.error
																? 'is-invalid'
																: ''}"
															style="height: 2.5rem;"
															disabled={$selectedGroup.name ===
																'$superadmins'}
															bind:value={minionTarget.target}
															on:blur={validatePermissionMinionTargetsFields}
														/>
														<label
															class="form-label"
															for="minionTarget_{minionTarget.targetId}"
															style="padding-top: 0.4rem;"
														>
															Target
														</label>
													</div>
													<button
														type="button"
														class="btn btn-sm btn-success float-end"
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
													</button>
												</div>
											</td>
											<td style="width: 12rem;">
												{#each minionTarget.modules as minionModule, mi}
													{#if mi > 0}
														<hr class="text-light my-2" />
													{/if}
													<div
														class="input-group input-group-sm flex-nowrap"
													>
														<div class="form-floating">
															<input
																id="minionModule_{minionTarget.targetId}_{minionModule.moduleId}"
																type="text"
																class="form-control {minionModule.error
																	? 'is-invalid'
																	: ''}"
																style="height: 2.5rem;"
																disabled={$selectedGroup.name ===
																	'$superadmins'}
																bind:value={minionModule.name}
																on:blur={validatePermissionMinionTargetsFields}
															/>
															<label
																class="form-label"
																for="minionModule_{minionTarget.targetId}_{minionModule.moduleId}"
																style="padding-top: 0.4rem;"
															>
																Module
															</label>
														</div>
														<button
															type="button"
															class="btn btn-sm btn-danger float-end"
															disabled={$selectedGroup.name ===
																'$superadmins'}
															on:click={() => {
																localRemoveMinionTargetModule(
																	minionTarget.targetId,
																	minionModule.moduleId,
																);
															}}
														>
															<Icon
																name="x"
																size="1"
																style="margin-top: -2px;"
															/>
														</button>
													</div>
												{/each}
											</td>
											<td>
												{#each minionTarget.modules as minionModule, mi}
													{#if mi > 0}
														<hr class="text-light my-2" />
													{/if}
													<div
														class="input-group input-group-sm flex-nowrap width-fit-content"
													>
														{#each minionModule.args as arg, ai}
															<div class="form-floating">
																<input
																	id="minionModuleArg_{minionTarget.targetId}_{minionModule.moduleId}_{ai}"
																	type="text"
																	class="form-control"
																	style="height: 2.5rem; max-width: 7rem;"
																	disabled={$selectedGroup.name ===
																		'$superadmins'}
																	bind:value={arg}
																	on:blur={validatePermissionMinionTargetsFields}
																/>
																<label
																	class="form-label"
																	for="minionModuleArg_{minionTarget.targetId}_{minionModule.moduleId}_{ai}"
																	style="padding-top: 0.4rem;"
																>
																	Arg {ai}
																</label>
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
																		$selectedGroup?.name ===
																		'$superadmins'
																	)
																		return;
																	localRemoveMinionTargetModuleArg(
																		minionTarget.targetId,
																		minionModule.moduleId,
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
																	$selectedGroup?.name ===
																	'$superadmins'
																)
																	return;
																localAddMinionTargetModuleArg(
																	minionTarget.targetId,
																	minionModule.moduleId,
																);
															}}
														/>
													</div>
												{/each}
											</td>
											<td>
												<button
													type="button"
													class="btn btn-sm btn-danger float-end mt-1"
													style="height: 2rem;"
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
												</button>
											</td>
										</tr>
									{/each}
								</tbody>
							</table>
							<!-- Display warning if any has "*" as target, instead of ".*" -->
							{#if $permissionMinionsFields.some((mt) => mt.target === '*')}
								<div class="alert alert-warning mt-0" role="alert">
									<strong>Warning!</strong> One or more minion targets have a
									target of <code>"*"</code> instead of <code>".*"</code>.
									<br />
									This will not match any minions. Please change the target to
									<code>".*"</code> to match all minions.
								</div>
							{/if}
							<!-- Display warning if any has "*" as module, instead of ".*" -->
							{#if $permissionMinionsFields.some( (mt) => mt.modules.some((mtm) => mtm.name === '*'), )}
								<div class="alert alert-warning mt-0" role="alert">
									<strong>Warning!</strong> One or more minion targets have a
									module of <code>"*"</code> instead of <code>".*"</code>.
									<br />
									This will not match any modules. Please change the module to
									<code>".*"</code> to match all modules.
								</div>
							{/if}
						</div>
						<div class="col-12 ps-3 mb-0">
							<h3>Actions</h3>
							{#if groupNameFieldError}
								<div class="alert alert-danger" role="alert">
									<strong>Invalid group name.</strong>
								</div>
							{/if}
							{#if groupLdapSyncFieldError}
								<div class="alert alert-danger" role="alert">
									<strong>Invalid LDAP sync DN.</strong>
								</div>
							{/if}
							{#if permissionMinionsFieldsError}
								<div class="alert alert-danger" role="alert">
									<strong>Invalid minion permissions.</strong>
								</div>
							{/if}

							<button
								type="button"
								class="btn btn-primary"
								disabled={$selectedGroup.name === '$superadmins'}
								on:click={updateSelectedGroup}
							>
								Save
							</button>
							<button
								type="button"
								class="btn btn-danger float-end"
								disabled={$selectedGroup.name === '$superadmins'}
								on:click={deleteSelectedGroup}
							>
								Delete Group
							</button>
						</div>
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>

<style>
	.width-fit-content {
		width: fit-content;
	}
</style>
