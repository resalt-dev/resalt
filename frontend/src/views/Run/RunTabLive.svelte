<script lang="ts">
    import {
        Button,
        Col,
        FormGroup,
        Input,
        Label,
        Modal,
        ModalBody,
        ModalFooter,
        ModalHeader,
        Row,
    } from "sveltestrap";
    import { quoteSplit } from "../../utils";
    import { theme } from "../../stores";

    let runConfirmDialog = false;

    let runClientType = "local";
    let runTargetType = "glob";
    let runTarget = "";
    let runFunction = "";
    let runArguments = "";
    let runKeywordArguments = "";
    let runAsync = false;
    let runBatch = false;
    let runBatchSize = "";
    let runTimeout = null;

    // These are pre-comupted before showing the confirmation modal.
    let client = "";
    let arg: string[] = [];
    let kwarg: Map<string, string> = new Map();

    function formSaveTemplate() {}

    function formApproval() {}

    function formRunNow() {
        // Error-checking
        if (runTarget.length === 0) {
            return;
        }
        if (runFunction.length === 0) {
            return;
        }

        openRunNowDialog();
    }

    function openRunNowDialog() {
        // client
        switch (runClientType) {
            case "local":
                if (runBatch) {
                    client = "local_batch";
                } else if (runAsync) {
                    client = "local_async";
                } else {
                    client = "local";
                }
                break;
            case "runner":
                if (runAsync) {
                    client = "runner_async";
                } else {
                    client = "runner";
                }
                break;
            case "wheel":
                if (runAsync) {
                    client = "wheel_async";
                } else {
                    client = "wheel";
                }
                break;
        }
        // arg
        arg = quoteSplit(runArguments);
        // kwarg
        kwarg.clear();
        quoteSplit(runKeywordArguments).forEach((item) => {
            let [key, value] = item.split("=");
            kwarg.set(key, value);
        });

        // Show confirm dialog
        runConfirmDialog = true;
    }

    function closeRunNowDialog() {
        runConfirmDialog = false;
    }

    function executeRunNow() {
        /*
        export async function runJob(
    tgtType: string,
    tgt: string,
    fun: string,
    arg: Array<string>,
    kwarg: Map<string, string>,
    batchSize: string,
    timeout: number,
): Promise<Job> {*/
    }
</script>

<Row>
    <Col class="form-check ps-3 mb-0" md="2">
        <FormGroup floating={true}>
            <Input
                id="clientType"
                type="select"
                name="select"
                bind:value={runClientType}
            >
                <option value="local" selected>Local</option>
                <option value="runner">Runner</option>
                <option value="wheel">Wheel</option>
            </Input>
            <Label for="clientType">Client Type</Label>
        </FormGroup>
    </Col>
    <Col class="form-check ps-3 mb-0" md="1" />
    <Col class="form-check ps-3 mb-0" md="2">
        {#if !runBatch}
            <div class="clearfix" />
            <Label for="async" class="ms-1 mb-0">Async</Label>
            <FormGroup floating={true} class="form-switch ps-0">
                <input
                    id="async"
                    type="checkbox"
                    class="form-check-input fs-3 ms-0 mt-0 mouse-pointer"
                    bind:checked={runAsync}
                />
            </FormGroup>
        {/if}
    </Col>
    <Col class="form-check ps-3 mb-0" md="1">
        {#if runClientType === "local"}
            <div class="clearfix" />
            <Label for="batch" class="ms-1 mb-0">Batch</Label>
            <FormGroup floating={true} class="form-switch ps-0">
                <input
                    id="batch"
                    type="checkbox"
                    class="form-check-input fs-3 ms-0 mt-0 mouse-pointer"
                    bind:checked={runBatch}
                />
            </FormGroup>
        {/if}
    </Col>
    <Col class="form-check ps-3 mb-0" md="1">
        {#if runClientType === "local" && runBatch}
            <FormGroup floating={true}>
                <Input id="batchSize" type="text" bind:value={runBatchSize} />
                <Label for="batchSize">Batch Size</Label>
            </FormGroup>
        {/if}
    </Col>
    <Col class="form-check ps-3 mb-0" md="1">
        {#if runClientType === "local" && (runBatch || (!runBatch && !runAsync))}
            <FormGroup floating={true}>
                <Input id="timeout" type="number" bind:value={runTimeout} />
                <Label for="timeout">Timeout</Label>
            </FormGroup>
        {/if}
    </Col>
</Row>

<Row>
    <Col class="form-check ps-3 mb-0" md="1">
        {#if runClientType === "local"}
            <FormGroup floating={true}>
                <Input
                    id="targetType"
                    type="select"
                    name="select"
                    bind:value={runTargetType}
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
    <Col class="form-check ps-3 mb-0" md="2">
        {#if runClientType === "local"}
            <FormGroup floating={true}>
                <Input id="target" type="text" bind:value={runTarget} />
                <Label for="target">Target</Label>
            </FormGroup>
        {/if}
    </Col>
    <Col class="form-check ps-3 mb-0" md="2">
        <FormGroup floating={true}>
            <Input id="function" type="text" bind:value={runFunction} />
            <Label for="function">Function</Label>
        </FormGroup>
    </Col>
    <Col class="form-check ps-3 mb-0" md="3">
        <FormGroup floating={true}>
            <Input id="arguments" type="text" bind:value={runArguments} />
            <Label for="arguments">Arguments</Label>
        </FormGroup>
    </Col>
    <Col class="form-check ps-3 mb-0" md="4">
        <FormGroup floating={true}>
            <Input
                id="keywordArguments"
                type="text"
                bind:value={runKeywordArguments}
            />
            <Label for="keywordArguments">Keyword Arguments</Label>
        </FormGroup>
    </Col>
</Row>

<hr class="text-light" />

<Row>
    <Col />
    <Col xs="auto">
        <Button
            class="me-3"
            color="success"
            on:click={formSaveTemplate}
            disabled>Save as Template</Button
        >
        <Button class="me-3" color="primary" on:click={formApproval} disabled
            >Submit for Approval</Button
        >
        <Button class="me-1" color="warning" on:click={formRunNow}
            >Run Now</Button
        >
    </Col>
</Row>

<div>
    <Modal
        isOpen={runConfirmDialog}
        toggle={() => {
            runConfirmDialog = false;
        }}
        contentClassName={$theme.dark ? "bg-darker text-white" : ""}
    >
        <ModalHeader
            toggle={() => {
                runConfirmDialog = false;
            }}
            class="bg-warning text-dark"
        >
            Live-Run Execution
        </ModalHeader>
        <ModalBody>
            You are about to execute the following job:
            <br />
            <br />
            Client Type: <b>{client}</b>
            <br />
            Target Type: <b>{runTargetType}</b>
            <br />
            Target: <b>{runTarget}</b>
            <br />
            Function: <b>{runFunction}</b>
            <br />
            Arguments: <b>{JSON.stringify(arg)}</b>
            <br />
            Keyword Arguments:
            <pre class="fw-bold"><b
                    >{JSON.stringify(Object.fromEntries(kwarg), null, 4)}</b
                ></pre>
            Timeout:{" "}<b>{runTimeout}</b>
            <br />
        </ModalBody>
        <ModalFooter>
            <Button color="warning" on:click={executeRunNow}>Run Now</Button>
            <Button color="secondary" on:click={closeRunNowDialog}>Close</Button
            >
        </ModalFooter>
    </Modal>
</div>
