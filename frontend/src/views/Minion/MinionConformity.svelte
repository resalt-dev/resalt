<script lang="ts">
    import JsonViewer from "../../components/JsonViewer.svelte";
    import { theme } from "../../stores";
    import ConsoleChangeBranch from "./ConsoleChangeBranch.svelte";

    const SHIFT = 10;

    export let minion;
    let rawData = false;

    enum SortOrder {
        Incremental = "Incremental order",
        Decremental = "Decremental order",
        LongestRuntime = "Longest runtime",
        BestResult = "Success first",
        WorstResult = "Errors first",
    }
    // Salt structure
    class ConformData {
        __id__: string;
        __run_num__: number;
        __sls__: string;
        changes: any;
        comment: string;
        duration: number;
        name: string;
        result: boolean | null;
        start_time: string;
    }
    class Conform {
        title: string;
        fun: string;
        color: string;
        data: ConformData;
    }

    let sortOrder: SortOrder = SortOrder.Incremental;
    let showSuccess: boolean = true;
    let showIncorrect: boolean = true;
    let showError: boolean = true;
    let showCollapsed: boolean = true;

    $: conformity = Object.entries(JSON.parse(minion.conformity) ?? [])
        .map(([key, v]) => {
            let value: any = v;

            let parts = key.split("_|-");
            let conform: Conform = {
                title: key,
                fun: parts[0] + "." + parts[parts.length - 1],
                // color should be success/warning/danger based on true/null/false
                color:
                    value.result === true
                        ? "success"
                        : value.result === false
                        ? "danger"
                        : "warning",
                data: {
                    __id__: value.__id__ ?? parts[1] ?? "UKNOWN ID",
                    __run_num__: value.__run_num__,
                    __sls__: value.__sls__,
                    changes: value.changes ?? {},
                    comment: value.comment,
                    duration: value.duration,
                    name: value.name ?? parts[2] ?? "UKNOWN NAME",
                    result: value.result,
                    start_time: value.start_time,
                } as ConformData,
            };
            return conform;
        })
        .sort((a, b) => {
            switch (sortOrder) {
                case SortOrder.Incremental:
                    return a.data.__run_num__ - b.data.__run_num__;
                case SortOrder.Decremental:
                    return b.data.__run_num__ - a.data.__run_num__;
                case SortOrder.LongestRuntime:
                    return b.data.duration - a.data.duration;
                case SortOrder.BestResult:
                    return (
                        (a.data.result === true
                            ? 1
                            : a.data.result === false
                            ? 3
                            : 2) -
                        (b.data.result === true
                            ? 1
                            : b.data.result === false
                            ? 3
                            : 2)
                    );
                case SortOrder.WorstResult:
                    return (
                        (b.data.result === true
                            ? 1
                            : b.data.result === false
                            ? 3
                            : 2) -
                        (a.data.result === true
                            ? 1
                            : a.data.result === false
                            ? 3
                            : 2)
                    );
                default:
                    return 0;
            }
        });

    function leftPadToTotalLength(
        str: string,
        maxLength: number,
        char: string = " "
    ) {
        return char.repeat(maxLength - str.length) + str;
    }
    function rightShiftLinesExceptFirst(
        str: string,
        paddingLength: number,
        char: string = " "
    ) {
        // Append paddingLength of spaces to each line except the first
        let lines = str.split("\n");
        let firstLine = lines.shift() ?? "";
        let padding = "";
        for (let i = 0; i < paddingLength; i++) {
            padding += char;
        }
        let paddedLines = [firstLine];
        for (let line of lines) {
            paddedLines.push(padding + line);
        }
        return paddedLines.join("\n");
    }
</script>

{#if !minion.conformity}
    <div class="p-3">No conformity data. Please refresh minion.</div>
{:else}
    <button
        class="btn btn-light float-end border border-1 rounded-none"
        style="margin-top: -0rem;z-index: 4;position: absolute;right: 0;"
        on:click={() => (rawData = !rawData)}
    >
        {rawData ? "View List" : "View JSON"}
    </button>

    {#if rawData}
        <JsonViewer code={minion.conformity} />
    {:else}
        <div class="row p-3">
            <div class="col-3">
                <div class="card mb-3 {$theme.dark ? 'bg-dark' : ''}">
                    <div class="card-header">
                        <span class="fw-bold">Options</span>
                    </div>
                    <div class="card-body">
                        <h5 class="card-title">Sort method</h5>

                        <!-- loop through sort orders-->
                        {#each Object.entries(SortOrder) as [sortKey, sortTitle]}
                            <div
                                class="form-check"
                                on:click={() =>
                                    (sortOrder = SortOrder[sortKey])}
                            >
                                <input
                                    class="form-check-input form-check-input-{$theme.dark
                                        ? $theme.color
                                        : 'dark'}"
                                    type="radio"
                                    name="sortMethod"
                                    id={`sortMethod-${sortKey}`}
                                    checked={sortOrder === SortOrder[sortKey]}
                                />
                                <label
                                    class="form-check-label"
                                    for={`sortMethod-${sortKey}`}
                                >
                                    {sortTitle}
                                </label>
                            </div>
                        {/each}

                        <h5 class="card-title mt-3">Visibility</h5>

                        <!-- showSuccess -->
                        <div
                            class="form-check"
                            on:click={() => (showSuccess = !showSuccess)}
                        >
                            <input
                                class="form-check-input form-check-input-success"
                                type="checkbox"
                                id="showSuccess"
                                checked={showSuccess}
                            />
                            <label class="form-check-label" for="showRawData">
                                Show Succeeded ({conformity.filter(
                                    (c) => c.data.result === true
                                ).length})
                            </label>
                        </div>
                        <!-- showIncorrect -->
                        <div class="form-check">
                            <input
                                class="form-check-input form-check-input-warning"
                                type="checkbox"
                                id="showIncorrect"
                                on:click={() =>
                                    (showIncorrect = !showIncorrect)}
                                checked={showIncorrect}
                            />
                            <label class="form-check-label" for="showIncorrect">
                                Show Incorrects ({conformity.filter(
                                    (c) => c.data.result === null
                                ).length})
                            </label>
                        </div>
                        <!-- showError -->
                        <div class="form-check">
                            <input
                                class="form-check-input form-check-input-danger"
                                type="checkbox"
                                id="showError"
                                on:click={() => (showError = !showError)}
                                checked={showError}
                            />
                            <label class="form-check-label" for="showError">
                                Show Errors ({conformity.filter(
                                    (c) => c.data.result === false
                                ).length})
                            </label>
                        </div>
                        <br />
                        <!-- showCollapsed -->
                        <div class="form-check">
                            <input
                                class="form-check-input form-check-input-{$theme.dark
                                    ? $theme.color
                                    : 'dark'}"
                                type="checkbox"
                                id="showCollapsed"
                                on:click={() =>
                                    (showCollapsed = !showCollapsed)}
                                checked={showCollapsed}
                            />
                            <label class="form-check-label" for="showCollapsed">
                                Show Collapsed
                            </label>
                        </div>
                    </div>
                </div>

                <div class="card mb-3 {$theme.dark ? 'bg-dark' : ''}">
                    <div class="card-header">
                        <span class="fw-bold">Tree view</span>
                    </div>
                    <div class="card-body">
                        <h5 class="card-title">Title</h5>
                        <p class="card-text">Text</p>
                    </div>
                </div>
            </div>
            <div class="col">
                <div class="d-grid">
                    {#each conformity as conform}
                        <div
                            class="card mb-3 {$theme.dark
                                ? 'bg-secondary'
                                : ''} startside-{conform.color} {!(
                                (showSuccess && conform.data.result === true) ||
                                (showIncorrect &&
                                    conform.data.result === null) ||
                                (showError && conform.data.result === false)
                            ) && !showCollapsed
                                ? 'd-none'
                                : ''}"
                        >
                            <div
                                type="button"
                                class="card-header"
                                data-bs-toggle="collapse"
                                data-bs-target="#conformityCollapse{conform.data
                                    .__run_num__}"
                            >
                                <span class=" fw-bold"
                                    >{conform.data.__sls__} :
                                </span>
                                {conform.data.__id__}
                                <small class="text-muted">({conform.fun})</small
                                >
                                <small class="float-end text-muted pt-1">
                                    # {conform.data.__run_num__}
                                </small>
                            </div>
                            <div
                                class="collapse {(showSuccess &&
                                    conform.data.result === true) ||
                                (showIncorrect &&
                                    conform.data.result === null) ||
                                (showError && conform.data.result === false)
                                    ? 'show'
                                    : ''}"
                                id="conformityCollapse{conform.data
                                    .__run_num__}"
                            >
                                <div class="card-body bg-dark text-light">
                                    <div class="card-text">
                                        <pre
                                            class="text-console m-0 text-{conform.color}">{leftPadToTotalLength(
                                                "ID",
                                                SHIFT
                                            )}: {conform.data.__id__}</pre>
                                        <pre
                                            class="text-console m-0 text-{conform.color}">{leftPadToTotalLength(
                                                "Function",
                                                SHIFT
                                            )}: {conform.fun}</pre>
                                        <pre
                                            class="text-console m-0 text-{conform.color}">{leftPadToTotalLength(
                                                "Name",
                                                SHIFT
                                            )}: {conform.data.name}</pre>
                                        <pre
                                            class="text-console m-0 text-{conform.color}">{leftPadToTotalLength(
                                                "Result",
                                                SHIFT
                                            )}: <span
                                                style="text-transform:capitalize;"
                                                >{conform.data.result === null
                                                    ? "None"
                                                    : conform.data.result}</span
                                            ></pre>
                                        <pre
                                            class="text-console m-0 text-{conform.color}">{leftPadToTotalLength(
                                                "Comment",
                                                SHIFT
                                            )}: {rightShiftLinesExceptFirst(
                                                conform.data.comment,
                                                SHIFT + 2
                                            )}</pre>
                                        <pre
                                            class="text-console m-0 text-{conform.color}">{leftPadToTotalLength(
                                                "Started",
                                                SHIFT
                                            )}: {conform.data.start_time}</pre>
                                        <pre
                                            class="text-console m-0 text-{conform.color}">{leftPadToTotalLength(
                                                "Duration",
                                                SHIFT
                                            )}: {conform.data.duration}</pre>
                                        <pre
                                            class="text-console m-0 text-{conform.color}">{leftPadToTotalLength(
                                                "Changes",
                                                SHIFT
                                            )}:</pre>
                                        {#if Object.keys(conform.data.changes).length != 0}
                                            <ConsoleChangeBranch
                                                changes={conform.data.changes}
                                                shift={SHIFT + 2}
                                            />
                                        {/if}
                                    </div>
                                </div>
                            </div>
                        </div>
                    {/each}
                </div>
            </div>
        </div>
    {/if}
{/if}
