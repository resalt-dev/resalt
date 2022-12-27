<script lang="ts">
    import {
        Button,
        Col,
        FormGroup,
        Input,
        Label,
        Row,
    } from 'sveltestrap';
    import { quoteSplit } from '../../utils';
    import { toasts } from '../../stores';
    import { runJob } from '../../api';
    import RunResult from '../../models/RunResult';
    import RunCommand from '../../models/RunCommand';
    import { MessageType } from '../../models/MessageType';
	import RunClientType from '../../models/RunClientType';
	import RunConfirmLiveBox from './RunConfirmLiveBox.svelte';
	import type { Writable } from 'svelte/store';

    export let returns: Writable<RunResult[]>;

    let clientTypeFieldValue: string = 'local';
    let clientTypeFieldError: boolean = false;
    let targetTypeFieldValue: string = 'glob';
    let targetTypeFieldError: boolean = false;
    let targetFieldValue: string = '';
    let targetFieldError: boolean = false;
    let functionFieldValue: string = '';
    let functionFieldError: boolean = false;
    let argsFieldValue: string = '';
    let argsFieldError: boolean = false;
    let kwargsFieldValue: string = '';
    let kwargsFieldError: boolean = false;
    let asyncFieldValue = false;
    let batchFieldValue = false;
    let batchSizeFieldValue: string = '';
    let batchSizeFieldError: boolean = false;

    // Pre-computed before showing the confirmation modal.
    let command: RunCommand = null;

    function formSaveTemplate() {
        if (!validate()) {
            return;
        }

        // TODO formSaveTemplate
    }

    function formApproval() {
        if (!validate()) {
            return;
        }

        // TODO formApproval
    }

    function formRunNow() {
        if (!validate()) {
            return;
        }

        openRunNowDialog();
    }

    function openRunNowDialog() {
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
        command = new RunCommand(
            client,
            targetTypeFieldValue,
            targetFieldValue,
            functionFieldValue,
            arg,
            kwarg,
            batchSizeFieldValue,
        );
    }

    function closeRunNowDialog() {
        command = null;
    }

    function _runNow() {
        let localCommand = command;
        runJob(
            localCommand.client,
            localCommand.targetType,
            localCommand.target,
            localCommand.fun,
            localCommand.arg,
            localCommand.kwarg,
            localCommand.batchSize,
        )
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

    /*
    // VALIDATION
    */

    function validate(): boolean {
        validateClientTypeField();
        validateTargetTypeField();
        validateTargetField();
        validateFunctionField();
        validateArgsField();
        validateKwargsField();
        validateBatchSizeField();

        return (
            !clientTypeFieldError &&
            !targetTypeFieldError &&
            !targetFieldError &&
            !functionFieldError &&
            !argsFieldError &&
            !kwargsFieldError &&
            !batchSizeFieldError
        );
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

<Row>
    <Col class="ps-3 mb-0" md="3" lg="2">
        <FormGroup floating={true}>
            <Input
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
            <Label for="clientType">Client Type</Label>
        </FormGroup>
    </Col>
    <Col class="ps-3 mb-0" md="2" lg={{ size: 1, offset: 1 }}>
        {#if !batchFieldValue}
            <div class="clearfix" />
            <Label for="async" class="ms-1 mb-0">Async</Label>
            <FormGroup floating={true} class="form-switch ps-0">
                <Input
                    type="switch"
                    class="fs-3"
                    invalid={clientTypeFieldError}
                    bind:checked={asyncFieldValue}
                    on:blur={validateClientTypeField}
                />
            </FormGroup>
        {/if}
    </Col>
    <Col class="ps-3 mb-0" md="2" lg={{ size: 1, offset: 1 }}>
        {#if clientTypeFieldValue === 'local'}
            <div class="clearfix" />
            <Label for="batch" class="ms-1 mb-0">Batch</Label>
            <FormGroup floating={true} class="form-switch ps-0">
                <Input
                    type="switch"
                    class="fs-3"
                    invalid={clientTypeFieldError}
                    bind:checked={batchFieldValue}
                    on:blur={validateClientTypeField}
                />
            </FormGroup>
        {/if}
    </Col>
    <Col class="ps-3 mb-0" md="2" lg="2" xl="1">
        {#if clientTypeFieldValue === 'local' && batchFieldValue}
            <FormGroup floating={true}>
                <Input
                    type="text"
                    invalid={batchSizeFieldError}
                    bind:value={batchSizeFieldValue}
                    on:blur={validateBatchSizeField}
                />
                <Label for="batchSize">Batch Size</Label>
            </FormGroup>
        {/if}
    </Col>
</Row>

<Row>
    <Col class="ps-3 mb-0" md="3" lg="2" xl="2" xxl="1">
        {#if clientTypeFieldValue === 'local'}
            <FormGroup floating={true}>
                <Input
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
                <Label for="targetType">Target Type</Label>
            </FormGroup>
        {/if}
    </Col>
    <Col class="ps-3 mb-0" md="5" lg="2">
        {#if clientTypeFieldValue === 'local'}
            <FormGroup floating={true}>
                <Input
                    type="text"
                    invalid={targetFieldError}
                    bind:value={targetFieldValue}
                    on:blur={validateTargetField}
                />
                <Label for="target">Target</Label>
            </FormGroup>
        {/if}
    </Col>
    <Col class="ps-3 mb-0" md="4" lg="2">
        <FormGroup floating={true}>
            <Input
                type="text"
                invalid={functionFieldError}
                bind:value={functionFieldValue}
                on:blur={validateFunctionField}
            />
            <Label for="function">Function</Label>
        </FormGroup>
    </Col>
    <Col class="ps-3 mb-0" md="12" lg="3">
        <FormGroup floating={true}>
            <Input
                type="text"
                invalid={argsFieldError}
                bind:value={argsFieldValue}
                on:blur={validateArgsField}
            />
            <Label for="arguments">Arguments</Label>
        </FormGroup>
    </Col>
    <Col class="ps-3 mb-0" md="12" lg="3" xl="3" xxl="4">
        <FormGroup floating={true}>
            <Input
                type="text"
                invalid={kwargsFieldError}
                bind:value={kwargsFieldValue}
                on:blur={validateKwargsField}
            />
            <Label for="keywordArguments">Keyword Arguments</Label>
        </FormGroup>
    </Col>
</Row>

<hr class="bg-light mt-1" />

<Row>
    <Col />
    <Col xs="auto">
        <Button
            class="me-3"
            color="success"
            on:click={formSaveTemplate}
            disabled
        >
            Save as Template
        </Button>
        <Button class="me-3" color="primary" on:click={formApproval} disabled>
            Submit for Approval
        </Button>
        <Button class="me-1" color="warning" on:click={formRunNow}>
            Run Now
        </Button>
    </Col>
</Row>

<RunConfirmLiveBox
    {command}
    close={closeRunNowDialog}
    execute={_runNow}
/>
