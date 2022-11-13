<script lang="ts">
    import { Card } from 'sveltestrap';
    import ConsoleChangeBranch from '../../components/ConsoleChangeBranch.svelte';

    const SHIFT = 10;

    export let color: string;
    export let num: number;
    export let sls: string;
    export let stateName: string;
    export let fun: string;
    export let name: string;
    export let result: boolean;
    export let comment: string;
    export let startTime: string;
    export let duration: number;
    export let changes: any;
    export let show: boolean;

    function leftPadToTotalLength(
        str: string,
        maxLength: number,
        char: string = ' ',
    ) {
        return char.repeat(maxLength - str.length) + str;
    }
    function rightShiftLinesExceptFirst(
        str: string,
        paddingLength: number,
        char: string = ' ',
    ) {
        // Append paddingLength of spaces to each line except the first
        let lines = str.split('\n');
        let firstLine = lines.shift() ?? '';
        let padding = '';
        for (let i = 0; i < paddingLength; i++) {
            padding += char;
        }
        let paddedLines = [firstLine];
        for (let line of lines) {
            paddedLines.push(padding + line);
        }
        return paddedLines.join('\n');
    }
</script>

<Card class="result-box mb-3 startside-{color}">
    <div
        type="button"
        class="card-header"
        data-bs-toggle="collapse"
        data-bs-target="#conformityCollapse{num}"
    >
        <span>{sls} : </span>
        {stateName}
        <small class="text-muted">({fun})</small>
        <small class="float-end text-muted pt-1">
            # {num + 1}
        </small>
    </div>
    <div class="collapse {show ? 'show' : ''}" id="conformityCollapse{num}">
        <div class="card-body bg-dark text-light">
            <div class="card-text">
                <pre
                    class="text-console m-0 text-{color}">{leftPadToTotalLength(
                        'ID',
                        SHIFT,
                    )}: {stateName}</pre>
                <pre
                    class="text-console m-0 text-{color}">{leftPadToTotalLength(
                        'Function',
                        SHIFT,
                    )}: {fun}</pre>
                <pre
                    class="text-console m-0 text-{color}">{leftPadToTotalLength(
                        'Name',
                        SHIFT,
                    )}: {name}</pre>
                <pre
                    class="text-console m-0 text-{color}">{leftPadToTotalLength(
                        'Result',
                        SHIFT,
                    )}: <span style="text-transform:capitalize;"
                        >{result === null ? 'None' : result}</span
                    ></pre>
                <pre
                    class="text-console m-0 text-{color}">{leftPadToTotalLength(
                        'Comment',
                        SHIFT,
                    )}: {rightShiftLinesExceptFirst(comment, SHIFT + 2)}</pre>
                <pre
                    class="text-console m-0 text-{color}">{leftPadToTotalLength(
                        'Started',
                        SHIFT,
                    )}: {startTime}</pre>
                <pre
                    class="text-console m-0 text-{color}">{leftPadToTotalLength(
                        'Duration',
                        SHIFT,
                    )}: {duration}</pre>
                <pre
                    class="text-console m-0 text-{color}">{leftPadToTotalLength(
                        'Changes',
                        SHIFT,
                    )}:</pre>
                {#if Object.keys(changes).length != 0}
                    <ConsoleChangeBranch data={changes} shift={SHIFT + 2} />
                {/if}
            </div>
        </div>
    </div>
</Card>
