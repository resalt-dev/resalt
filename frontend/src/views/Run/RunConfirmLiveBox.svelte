<script lang="ts">
	import { Alert, Button, Modal, ModalBody, ModalHeader, Table } from 'sveltestrap';
	import CopyButton from '../../components/CopyButton.svelte';
	import RunClientType from '../../models/RunClientType';
	import type RunCommand from '../../models/RunCommand';
	import { currentUser, theme } from '../../stores';
	import { hasPermission } from '../../perms';

	export let command: RunCommand;
	export let close: () => void;
	export let execute: () => void;

	let commandLine: string;
	$: commandLine = command !== null ? command.toCommandLine({}) : '';
	let prettyKwargs: string = '';
	$: {
		prettyKwargs = '';
		if (command !== null) {
			command.kwarg.forEach((value, key) => {
				prettyKwargs += ` ${key}=${value}`;
			});
		}
	}
</script>

<Modal
	isOpen={command !== null}
	toggle={close}
	class={$theme.dark ? 'theme-dark' : ''}
	contentClassName={$theme.dark ? 'bg-secondary text-white' : ''}
>
	<ModalHeader toggle={close} class="bg-warning text-dark">Live-Run Execution</ModalHeader>
	<ModalBody>
		The following command is about to be executed:
		<br />
		<br />

		<!-- Summarize what is about to be run -->
		<Table>
			<tbody>
				<tr>
					<th style="width: 50%">Client Type</th>
					<td>{RunClientType.getBaseType(command.client)}</td>
				</tr>
				{#if RunClientType.getBaseType(command.client) === 'local'}
					<tr>
						<th>Target Type</th>
						<td>{command.targetType}</td>
					</tr>
					<tr>
						<th>Target</th>
						<td>{command.target}</td>
					</tr>
				{/if}
				<tr>
					<th>Function</th>
					<td>{command.fun}</td>
				</tr>
				<tr>
					<th>Arguments</th>
					<td>{command.arg.toString()}</td>
				</tr>
				<tr>
					<th>Keyword Arguments</th>
					<td>{prettyKwargs}</td>
				</tr>
				<tr>
					<th>Async</th>
					<td>{RunClientType.isAsync(command.client)}</td>
				</tr>
				{#if RunClientType.getBaseType(command.client) === 'local'}
					<tr>
						<th>Batch</th>
						<td
							>{RunClientType.isBatch(command.client)}
							{#if RunClientType.isBatch(command.client)}({command.batchSize}){/if}</td
						>
					</tr>
				{/if}
			</tbody>
		</Table>

		<br />
		Command-line equivalent:<br />
		<br />

		<code>{commandLine}</code>
		<CopyButton name="Command" value={commandLine} />

		<br />
		<br />

		{#if !hasPermission($currentUser, command.toPermissionTarget(), command.fun, command.arg, command.kwarg)}
			<Alert color="warning" dismissible={false} fade={false}>
				<strong>Warning!</strong> You likely don't have sufficient permissions to execute this
				command. Please verify the target group and function name before proceeding. Please contact
				a system administrator if unsure.
			</Alert>
			<br />
		{/if}

		<div class="text-center">
			<Button color="warning" on:click={close}>Cancel</Button>
			<Button color="danger" on:click={execute}>Execute</Button>
		</div>
	</ModalBody>
</Modal>
