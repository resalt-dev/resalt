<script lang="ts">
    export let data: any;
    export let shift: number;

    function rightShiftLines(
        str: string,
        paddingLength: number,
        char: string = " "
    ) {
        // Append paddingLength of spaces to all lines
        let lines = str.split("\n");
        let padding = "";
        for (let i = 0; i < paddingLength; i++) {
            padding += char;
        }
        let paddedLines = [];
        for (let line of lines) {
            paddedLines.push(padding + line);
        }
        return paddedLines.join("\n");
    }

    $: console.log("data2", data);
</script>

{#if typeof data === "object" && data != null}
    {#if Object.keys(data).length != 0}
        <pre class="text-console m-0 text-cyan">{rightShiftLines(
                "----------",
                shift
            )}</pre>
        {#each Object.entries(data) as [changeKey, changeValue]}
            <pre class="text-console m-0 text-cyan">{rightShiftLines(
                    changeKey,
                    shift
                )}<span class="text-light">:</span></pre>
            {#if typeof data === "string"}
                <pre class="text-console m-0 text-green">{rightShiftLines(
                        (data + "").trim(),
                        shift
                    )}</pre>
            {:else}
                <svelte:self data={changeValue} shift={shift + 4} />
            {/if}
        {/each}
    {/if}
{:else}
    <pre class="text-console m-0 text-green">{rightShiftLines(
            (data + "").trim(),
            shift
        )}</pre>
{/if}
