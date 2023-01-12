<script lang="ts">
	import { Button, Col, Row } from 'sveltestrap';
	import { toasts } from '../../stores';
	import { runJob } from '../../api';
	import RunResult from '../../models/RunResult';
	import { MessageType } from '../../models/MessageType';
	import RunConfirmLiveBox from './RunConfirmLiveBox.svelte';
	import type { Writable } from 'svelte/store';
	import RunCommandBox from './RunCommandBox.svelte';
	import type RunCommand from '../../models/RunCommand';

	export let returns: Writable<RunResult[]>;

	let validate: () => RunCommand;

	// Pre-computed before showing the confirmation modal.
	let command: RunCommand = null;

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
		let localCommand = command;
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

<Row>
	<Col />
	<Col xs="auto">
		<Button class="me-3" color="success" on:click={formSaveTemplate} disabled>
			Save as Template
		</Button>
		<Button class="me-3" color="primary" on:click={formApproval} disabled>
			Submit for Approval
		</Button>
		<Button class="me-1" color="warning" on:click={formRunNow}>Run Now</Button>
	</Col>
</Row>

<RunConfirmLiveBox {command} close={closeRunNowDialog} execute={_runNow} />
