<script lang="ts">
    import type { Writable } from 'svelte/store';
    import { Card, CardBody, CardHeader, Col, Row } from 'sveltestrap';
    import JsonViewer from '../../components/JsonViewer.svelte';
    import ResultBox from '../../components/ResultBox.svelte';
    import type Minion from '../../models/Minion';
    import { theme } from '../../stores';
    import ConformityTreeView from './ConformityTreeView.svelte';
    import type {
        ConformData,
        Conform,
        ConformTreeNode,
    } from './ConformityTypes';

    export let minion: Writable<Minion>;
    let rawData = false;

    enum ConformSortOption {
        Incremental = 'Incremental order',
        Decremental = 'Decremental order',
        LongestRuntime = 'Longest runtime',
        BestResult = 'Success first',
        WorstResult = 'Errors first',
    }

    let sortOrder: ConformSortOption = ConformSortOption.Incremental;
    let showSuccess: boolean = true;
    let showIncorrect: boolean = true;
    let showError: boolean = true;
    let showCollapsed: boolean = true;

    $: conformity = Object.entries(JSON.parse($minion.conformity) ?? [])
        .map(([key, v]) => {
            let value: any = v;

            let parts = key.split('_|-');
            let conform: Conform = {
                title: key,
                fun: parts[0] + '.' + parts[parts.length - 1],
                // color should be success/warning/danger based on true/null/false
                color:
                    value.result === true
                        ? 'success'
                        : value.result === false
                        ? 'danger'
                        : 'warning',
                data: {
                    __id__: value.__id__ ?? parts[1] ?? 'UKNOWN ID',
                    __run_num__: value.__run_num__,
                    __sls__: value.__sls__,
                    changes: value.changes ?? {},
                    comment: value.comment,
                    duration: value.duration,
                    name: value.name ?? parts[2] ?? 'UKNOWN NAME',
                    result: value.result,
                    start_time: value.start_time,
                } as ConformData,
            };
            return conform;
        })
        .sort((a, b) => {
            switch (sortOrder) {
                case ConformSortOption.Incremental:
                    return a.data.__run_num__ - b.data.__run_num__;
                case ConformSortOption.Decremental:
                    return b.data.__run_num__ - a.data.__run_num__;
                case ConformSortOption.LongestRuntime:
                    return b.data.duration - a.data.duration;
                case ConformSortOption.BestResult:
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
                case ConformSortOption.WorstResult:
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

    // Reduce above Conformity states to a tree of SLS files
    // - a (1)
    //   - aa (1)
    //   - ab (1)
    // - common
    //   - init
    //     - test (2)
    // - editor (1)
    //   - vim (7)
    function sortSubtreeRecursively(subtree: ConformTreeNode[]) {
        subtree.sort((a, b) => a.name.localeCompare(b.name));
        subtree.forEach((node) => {
            sortSubtreeRecursively(node.subtree);
        });
    }

    $: conformity_tree = conformity
        // .filter((c) => {
        //     if (!showSuccess && c.data.result === true) return false;
        //     if (!showIncorrect && c.data.result === null) return false;
        //     if (!showError && c.data.result === false) return false;
        //     return true;
        // })
        .reduce(
            (acc, c) => {
                let parts = c.data.__sls__.split('.');
                let current = acc;
                for (let i = 0; i < parts.length; i++) {
                    let part = parts[i];
                    let existing = current.subtree.find((e) => e.name === part);
                    if (!existing) {
                        existing = {
                            name: part,
                            subtree: [],
                            items: [],
                        };
                        current.subtree.push(existing);
                    }
                    current = existing;
                }
                current.items.push(c);
                return acc;
            },
            {
                name: '#',
                subtree: [],
                items: [],
            } as ConformTreeNode,
        );
    // Recursively sort tree alphabetically
    $: sortSubtreeRecursively(conformity_tree.subtree);

    $: console.log(conformity_tree);
</script>

{#if !$minion.conformity}
    <div class="p-3">No conformity data. Please refresh minion.</div>
{:else}
    <button
        class="btn btn-light float-end border border-1 rounded-none"
        style="margin-top: -1rem;z-index: 4;position: absolute;right: 0;"
        on:click={() => (rawData = !rawData)}
    >
        {rawData ? 'View List' : 'View JSON'}
    </button>

    {#if rawData}
        <JsonViewer data={JSON.parse($minion.conformity)} />
    {:else}
        <Row>
            <Col xs="3">
                <Card class="mb-3">
                    <CardHeader>Options</CardHeader>
                    <CardBody>
                        <h5 class="card-title">Sort method</h5>

                        <!-- loop through sort orders-->
                        {#each Object.entries(ConformSortOption) as [sortKey, sortTitle]}
                            <div class="form-check">
                                <input
                                    class="form-check-input form-check-input-{$theme.color}"
                                    type="radio"
                                    name="sortMethod"
                                    id={`sortMethod-${sortKey}`}
                                    on:click={() =>
                                        (sortOrder =
                                            ConformSortOption[sortKey])}
                                    checked={sortOrder ===
                                        ConformSortOption[sortKey]}
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
                        <div class="form-check">
                            <input
                                class="form-check-input form-check-input-success"
                                type="checkbox"
                                id="showSuccess"
                                on:click={() => (showSuccess = !showSuccess)}
                                checked={showSuccess}
                            />
                            <label class="form-check-label" for="showSuccess">
                                Show Succeeded ({conformity.filter(
                                    (c) => c.data.result === true,
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
                                    (c) => c.data.result === null,
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
                                    (c) => c.data.result === false,
                                ).length})
                            </label>
                        </div>
                        <br />
                        <!-- showCollapsed -->
                        <div class="form-check">
                            <input
                                class="form-check-input form-check-input-{$theme.color}"
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
                    </CardBody>
                </Card>

                <Card class="mb-3">
                    <CardHeader>File Structure</CardHeader>
                    <CardBody>
                        <!-- Render Tree structure in a recursive fashion. -->
                        <ConformityTreeView node={conformity_tree} />
                    </CardBody>
                </Card>
            </Col>
            <Col>
                <div class="d-grid">
                    {#each conformity as conform}
                        <div
                            class=" {!(
                                (showSuccess && conform.data.result === true) ||
                                (showIncorrect &&
                                    conform.data.result === null) ||
                                (showError && conform.data.result === false)
                            ) && !showCollapsed
                                ? 'd-none'
                                : ''}"
                        >
                            <ResultBox
                                color={conform.color}
                                num={conform.data.__run_num__}
                                sls={conform.data.__sls__}
                                stateName={conform.data.__id__}
                                fun={conform.fun}
                                name={conform.data.name}
                                result={conform.data.result}
                                comment={conform.data.comment}
                                startTime={conform.data.start_time}
                                duration={conform.data.duration}
                                changes={conform.data.changes}
                                show={(showSuccess &&
                                    conform.data.result === true) ||
                                    (showIncorrect &&
                                        conform.data.result === null) ||
                                    (showError &&
                                        conform.data.result === false)}
                            />
                        </div>
                    {/each}
                </div>
            </Col>
        </Row>
    {/if}
{/if}
