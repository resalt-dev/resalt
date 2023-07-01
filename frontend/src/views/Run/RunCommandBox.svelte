<script lang="ts">
	import { Input } from 'sveltestrap';
	import RunClientType from '../../models/RunClientType';
	import RunCommand from '../../models/RunCommand';
	import { quoteSplit } from '../../utils';

	const urlParams = new URLSearchParams(window.location.search);

	export const validate = _validate;

	type clientType = 'local' | 'runner' | 'wheel';
	let clientTypeFieldValue: clientType = (urlParams.get('client_type') as clientType) || 'local';
	let clientTypeFieldError: boolean = false;
	let targetTypeFieldValue: string = urlParams.get('target_type') || 'glob';
	let targetTypeFieldError: boolean = false;
	let targetFieldValue: string = urlParams.get('target') || '';
	let targetFieldError: boolean = false;
	let functionFieldValue: string = urlParams.get('fun') || '';
	let functionFieldError: boolean = false;
	let argsFieldValue: string = urlParams.get('arg') || '';
	let argsFieldError: boolean = false;
	let kwargsFieldValue: string = urlParams.get('kwarg') || '';
	let kwargsFieldError: boolean = false;
	let asyncFieldValue: boolean = urlParams.get('async') === 'true';
	let batchFieldValue: boolean = urlParams.get('batch') === 'true';
	let batchSizeFieldValue: string = urlParams.get('batch_size') || '';
	let batchSizeFieldError: boolean = false;

	function formToCommand(): RunCommand {
		// client
		let client: RunClientType = null;
		switch (clientTypeFieldValue) {
			case 'local':
				if (batchFieldValue && batchSizeFieldValue.length > 0) {
					client = RunClientType.LOCAL_BATCH;
				} else if (asyncFieldValue) {
					client = RunClientType.LOCAL_ASYNC;
				} else {
					client = RunClientType.LOCAL;
				}
				break;
			case 'runner':
				if (asyncFieldValue) {
					client = RunClientType.RUNNER_ASYNC;
				} else {
					client = RunClientType.RUNNER;
				}
				break;
			case 'wheel':
				if (asyncFieldValue) {
					client = RunClientType.WHEEL_ASYNC;
				} else {
					client = RunClientType.WHEEL;
				}
				break;
		}
		// arg
		let arg = quoteSplit(argsFieldValue);
		// kwarg
		let kwarg = new Map<string, string>();
		quoteSplit(kwargsFieldValue).forEach((item) => {
			let [key, value] = item.split('=');
			kwarg.set(key, value);
		});

		// Clone / keep a local copy of all run parameters,
		// in case they change between running the job and storing
		// the result in results array.
		return new RunCommand(
			client,
			targetTypeFieldValue,
			targetFieldValue,
			functionFieldValue,
			arg,
			kwarg,
			batchSizeFieldValue,
		);
	}

	/*
    // VALIDATION
    */

	function _validate(): RunCommand | null {
		validateClientTypeField();
		validateTargetTypeField();
		validateTargetField();
		validateFunctionField();
		validateArgsField();
		validateKwargsField();
		validateBatchSizeField();

		let invalid =
			clientTypeFieldError ||
			(clientTypeFieldValue === 'local' && targetTypeFieldError) ||
			(clientTypeFieldValue === 'local' && targetFieldError) ||
			functionFieldError ||
			argsFieldError ||
			kwargsFieldError ||
			(clientTypeFieldValue === 'local' && batchSizeFieldError);

		if (invalid) {
			return null;
		} else {
			return formToCommand();
		}
	}

	function validateClientTypeField(): void {
		clientTypeFieldError = false;
	}

	function validateTargetTypeField(): void {
		if (targetTypeFieldValue.length === 0) {
			targetTypeFieldError = true;
			return;
		}
		targetTypeFieldError = false;
	}

	function validateTargetField(): void {
		if (targetFieldValue.length === 0) {
			targetFieldError = true;
			return;
		}
		targetFieldError = false;
	}

	function validateFunctionField(): void {
		functionFieldValue = functionFieldValue.toLowerCase();
		if (functionFieldValue.length < 3) {
			functionFieldError = true;
			return;
		}
		if (functionFieldValue.indexOf('.') === -1) {
			functionFieldError = true;
			return;
		}
		if (functionFieldValue.startsWith('.') || functionFieldValue.endsWith('.')) {
			functionFieldError = true;
			return;
		}
		functionFieldError = false;
	}

	function validateArgsField(): void {
		argsFieldError = false;
	}

	function validateKwargsField(): void {
		kwargsFieldError = false;
	}

	function validateBatchSizeField(): void {
		if (batchFieldValue && batchSizeFieldValue.length === 0) {
			batchSizeFieldError = true;
			return;
		}
		// Error if contains anything else but numbers and percent sign
		if (!batchSizeFieldValue.match(/[^0-9%]/)) {
			batchSizeFieldError = false;
			return;
		}
		batchSizeFieldError = true;
	}
</script>

<div class="row">
	<div class="col col-md-3 col-lg-2 mb-0">
		<div class="form-floating mb-3">
			<Input
				id="clientType"
				type="select"
				name="select"
				invalid={clientTypeFieldError}
				bind:value={clientTypeFieldValue}
				on:blur={validateClientTypeField}
			>
				<option value="local" selected>Local</option>
				<option value="runner">Runner</option>
				<option value="wheel">Wheel</option>
			</Input>
			<label class="form-label" for="clientType">Client Type</label>
		</div>
	</div>
	<div class="col col-md-2 col-lg-1 offset-lg-1 mb-0">
		{#if !batchFieldValue}
			<div class="clearfix" />
			<label class="form-label ms-1 mb-0" for="async">Async</label>
			<div class="form-floating mb-3 ps-0 form-switch">
				<Input
					id="async"
					type="switch"
					class="fs-3"
					invalid={clientTypeFieldError}
					bind:checked={asyncFieldValue}
					on:blur={validateClientTypeField}
				/>
			</div>
		{/if}
	</div>
	<div class="col col-md-2 col-lg-1 offset-lg-1 mb-0">
		{#if clientTypeFieldValue === 'local'}
			<div class="clearfix" />
			<label class="form-label ms-1 mb-0" for="batch">Batch</label>
			<div class="form-floating mb-3 ps-0 form-switch">
				<Input
					id="batch"
					type="switch"
					class="fs-3"
					invalid={clientTypeFieldError}
					bind:checked={batchFieldValue}
					on:blur={validateClientTypeField}
				/>
			</div>
		{/if}
	</div>
	<div class="col col-md-2 col-lg-2 col-xl-1 mb-0">
		{#if clientTypeFieldValue === 'local' && batchFieldValue}
			<div class="form-floating mb-3">
				<Input
					id="batchSize"
					type="text"
					invalid={batchSizeFieldError}
					bind:value={batchSizeFieldValue}
					on:blur={validateBatchSizeField}
				/>
				<label class="form-label" for="batchSize">Batch Size</label>
			</div>
		{/if}
	</div>
</div>

<div class="row">
	<div class="col col-md-3 col-lg-2 col-xl-2 col-xxl-1 mb-0">
		{#if clientTypeFieldValue === 'local'}
			<div class="form-floating mb-3">
				<Input
					id="targetType"
					type="select"
					invalid={targetTypeFieldError}
					bind:value={targetTypeFieldValue}
					on:blur={validateTargetTypeField}
				>
					<option value="glob" selected>Glob</option>
					<option value="pcre">PCRE</option>
					<option value="list">List</option>
					<option value="grain">Grain</option>
					<option value="grain_pcre">Grain PCRE</option>
					<option value="pillar">Pillar</option>
					<option value="pillar_pcre">Pillar PCRE</option>
					<option value="nodegroup">Node Group</option>
					<option value="range">Range</option>
					<option value="compound">Compound</option>
					<option value="ipcidr">IPCIDR</option>
				</Input>
				<label class="form-label" for="targetType">Target Type</label>
			</div>
		{/if}
	</div>
	<div class="col col-md-5 col-lg-2 mb-0">
		{#if clientTypeFieldValue === 'local'}
			<div class="form-floating mb-3">
				<Input
					id="target"
					type="text"
					invalid={targetFieldError}
					bind:value={targetFieldValue}
					on:blur={validateTargetField}
				/>
				<label class="form-label" for="target">Target</label>
			</div>
		{/if}
	</div>
	<div class="col col-md-4 col-lg-2 mb-0">
		<div class="form-floating mb-3">
			<Input
				id="function"
				type="text"
				invalid={functionFieldError}
				bind:value={functionFieldValue}
				on:blur={validateFunctionField}
			/>
			<label class="form-label" for="function">Function</label>
		</div>
	</div>
	<div class="col col-md-12 col-lg-3 mb-0">
		<div class="form-floating mb-3">
			<Input
				id="arguments"
				type="text"
				invalid={argsFieldError}
				bind:value={argsFieldValue}
				on:blur={validateArgsField}
			/>
			<label class="form-label" for="arguments">Arguments</label>
		</div>
	</div>
	<div class="col col-md-12 col-lg-3 col-xl-3 col-xxl-4 mb-0">
		<div class="form-floating mb-3">
			<Input
				id="keywordArguments"
				type="text"
				invalid={kwargsFieldError}
				bind:value={kwargsFieldValue}
				on:blur={validateKwargsField}
			/>
			<label class="form-label" for="keywordArguments">Keyword Arguments</label>
		</div>
	</div>
</div>
