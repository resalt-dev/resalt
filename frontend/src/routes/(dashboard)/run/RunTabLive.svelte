<script lang="ts">
	import { toasts } from '$lib/stores';
	import { runJob } from '$lib/api';
	import RunResult from '../../../models/RunResult';
	import { MessageType } from '../../../models/MessageType';
	import RunConfirmLiveBox from './RunConfirmLiveBox.svelte';
	import type { Writable } from 'svelte/store';
	import RunCommandBox from './RunCommandBox.svelte';
	import type RunCommand from '../../../models/RunCommand';

	export let returns: Writable<RunResult[]>;

	let validate: () => RunCommand;

	// Pre-computed before showing the confirmation modal.
	let command: RunCommand | null = null;

	function formSaveTemplate() {
		// TODO formSaveTemplate
	}

	function formApproval() {
		// TODO formApproval
	}

	function formRunNow() {
		let result: RunCommand | null = validate();
		if (result) {
			command = result;
		}
	}

	function closeRunNowDialog() {
		command = null;
	}

	function _runNow() {
		let localCommand: RunCommand;
		if (command === null) {
			return;
		} else {
			localCommand = command;
		}
		runJob(localCommand)
			.then((result) => {
				console.log(result);
				returns.update((returns: RunResult[]) => [
					new RunResult(localCommand, returns.length, result),
					...returns,
				]);
			})
			.catch((error) => {
				console.error(error);
				toasts.add(MessageType.ERROR, 'Failed executing job', error);
			});
		closeRunNowDialog();
	}
</script>

<RunCommandBox bind:validate />

<hr class="text-light mt-1" />

<div class="row">
	<div class="col" />
	<div class="col-auto">
		<button type="button" class="btn btn-success me-3" on:click={formSaveTemplate} disabled>
			Save as Template
		</button>
		<button type="button" class="btn btn-primary me-3" on:click={formApproval} disabled>
			Submit for Approval
		</button>
		<button type="button" class="btn btn-warning me-1" on:click={formRunNow}>Run Now</button>
	</div>
</div>

<RunConfirmLiveBox {command} close={closeRunNowDialog} execute={_runNow} />
