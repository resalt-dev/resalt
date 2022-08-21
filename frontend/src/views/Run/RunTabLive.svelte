<script>
    import {
        Button,
        Col,
        Form,
        FormGroup,
        Input,
        Label,
        Row,
    } from "sveltestrap";

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

    $: if (runBatch) {
        runAsync = false;
    }

    function formSaveTemplate() {}

    function formApproval() {}

    function formRunNow() {}
</script>

<Form>
    <Row>
        <Col class="form-check ps-3 mb-0" md="2">
            <FormGroup>
                <Label for="clientType">Client Type</Label>
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
            </FormGroup>
        </Col>
        <Col class="form-check ps-3 mb-0" md="1" />
        <Col class="form-check ps-3 mb-0" md="2">
            {#if !runBatch}
                <Label for="async" class="mb-0">Async</Label>
                <FormGroup class="mt-2 pt-1 form-check form-switch ps-0">
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
                <Label for="batch" class="mb-0">Batch</Label>
                <FormGroup class="mt-2 pt-1 form-switch ps-0">
                    <input
                        id="batch"
                        type="checkbox"
                        class="form-check-input fs-3 ms-1 mt-0 mouse-pointer"
                        bind:checked={runBatch}
                    />
                </FormGroup>
            {/if}
        </Col>
        <Col class="form-check ps-3 mb-0" md="1">
            {#if runClientType === "local" && runBatch}
                <FormGroup>
                    <Label for="batchSize">Batch Size</Label>
                    <Input
                        id="batchSize"
                        type="text"
                        bind:value={runBatchSize}
                    />
                </FormGroup>
            {/if}
        </Col>
        <Col class="form-check ps-3 mb-0" md="1">
            {#if !runAsync}
                <FormGroup>
                    <Label for="timeout">Timeout</Label>
                    <Input id="timeout" type="number" bind:value={runTimeout} />
                </FormGroup>
            {/if}
        </Col>
    </Row>

    <Row>
        <Col class="form-check ps-3 mb-0" md="1">
            {#if runClientType === "local"}
                <FormGroup>
                    <Label for="targetType">Target Type</Label>
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
                </FormGroup>
            {/if}
        </Col>
        <Col class="form-check ps-3 mb-0" md="2">
            {#if runClientType === "local"}
                <FormGroup>
                    <Label for="target">Target</Label>
                    <Input id="target" type="text" bind:value={runTarget} />
                </FormGroup>
            {/if}
        </Col>
        <Col class="form-check ps-3 mb-0" md="2">
            <FormGroup>
                <Label for="function">Function</Label>
                <Input id="function" type="text" bind:value={runFunction} />
            </FormGroup>
        </Col>
        <Col class="form-check ps-3 mb-0" md="3">
            <FormGroup>
                <Label for="arguments">Arguments</Label>
                <Input id="arguments" type="text" bind:value={runArguments} />
            </FormGroup>
        </Col>
        <Col class="form-check ps-3 mb-0" md="4">
            <FormGroup>
                <Label for="keywordArguments">Keyword Arguments</Label>
                <Input
                    id="keywordArguments"
                    type="text"
                    bind:value={runKeywordArguments}
                />
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
            <Button
                class="me-3"
                color="primary"
                on:click={formApproval}
                disabled>Line up for Approval</Button
            >
            <Button class="me-1" color="warning" on:click={formRunNow}
                >Run Now</Button
            >
        </Col>
    </Row>
</Form>
