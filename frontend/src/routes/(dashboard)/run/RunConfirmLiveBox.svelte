<script lang="ts">
	import Clickable from '$component/Clickable.svelte';
	import CopyButton from '$component/CopyButton.svelte';
	import { hasPermission } from '$lib/perms';
	import { currentUser, theme } from '$lib/stores';
	import { getRCTBaseType, isRCTAsync, isRCTBatch } from '$model/RunClientType';
	import type RunCommand from '$model/RunCommand';
	import { Modal } from 'bootstrap';
	import { v4 as uuidv4 } from 'uuid';

	export let command: RunCommand | null;
	export let close: () => void;
	export let execute: () => void;

	const randomId = uuidv4();

	let commandLine: string;
	$: commandLine = command !== null ? command.toCommandLine({}) : '';
	let prettyKwargs = '';
	$: {
		prettyKwargs = '';
		if (command !== null) {
			command.kwarg.forEach((value, key) => {
				prettyKwargs += ` ${key}=${value}`;
			});
		}
	}

	$: {
		if (command !== null) {
			showDialog();
		} else {
			hideDialog();
		}
	}

	function showDialog(): void {
		// Show dialog BS5
		const modalWidget = document.getElementById(randomId);
		if (modalWidget !== null) {
			const modal = new Modal(modalWidget);
			modal.show();
		}
	}

	function hideDialog(): void {
		// Hide dialog BS5
		const modalWidget = document.getElementById(randomId);
		if (modalWidget !== null) {
			const modal = Modal.getInstance(modalWidget);
			if (modal !== null) {
				modal.hide();
			}
		}
	}
</script>

<!-- isOpen={command !== null} -->
<div class="modal {$theme.dark ? 'theme-dark' : ''}" tabindex="-1" role="dialog" id={randomId}>
	<div class="modal-dialog">
		<div class="modal-content {$theme.dark ? 'bg-secondary text-white' : ''}">
			<Clickable type="div" event={close} class="modal-header bg-warning text-dark">
				Live-Run Execution
			</Clickable>
			{#if command !== null}
				<div class="modal-body">
					The following command is about to be executed:
					<br />
					<br />

					<!-- Summarize what is about to be run -->
					<table class="table">
						<tbody>
							<tr>
								<th style="width: 50%">Client Type</th>
								<td>{getRCTBaseType(command.client)}</td>
							</tr>
							{#if getRCTBaseType(command.client) === 'local'}
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
								<td>{isRCTAsync(command.client)}</td>
							</tr>
							{#if getRCTBaseType(command.client) === 'local'}
								<tr>
									<th>Batch</th>
									<td
										>{isRCTBatch(command.client)}
										{#if isRCTBatch(command.client)}({command.batchSize}){/if}</td
									>
								</tr>
							{/if}
						</tbody>
					</table>

					<br />
					Command-line equivalent:<br />
					<br />

					<code>{commandLine}</code>
					<CopyButton name="Command" value={commandLine} />

					<br />
					<br />

					{#if !hasPermission($currentUser, command.toPermissionTarget(), command.fun, command.arg, Object.fromEntries(command.kwarg))}
						<div class="alert alert-warning" role="alert">
							<strong>Warning!</strong> You likely don't have sufficient permissions to
							execute this command. Please verify the target group and function name before
							proceeding. Please contact a system administrator if unsure.
						</div>
						<br />
					{/if}

					<div class="text-center">
						<button type="button" class="btn btn-warning me-2" on:click={close}>
							Cancel
						</button>
						<button type="button" class="btn btn-danger" on:click={execute}>
							Execute
						</button>
					</div>
				</div>
			{/if}
		</div>
	</div>
</div>
