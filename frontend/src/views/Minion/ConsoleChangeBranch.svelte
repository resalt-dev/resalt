<script lang="ts">
    export let changes;
    export let shift;

    function rightShiftLines(str, paddingLength, char = " ") {
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
</script>

{#if typeof changes === "object"}
    {#if Object.keys(changes).length != 0}
        <pre class="text-console m-0 text-cyan">{rightShiftLines(
                "----------",
                shift
            )}</pre>
        {#each Object.entries(changes) as [changeKey, changeValue]}
            <pre class="text-console m-0 text-cyan">{rightShiftLines(
                    changeKey,
                    shift
                )}</pre>
            {#if typeof changes === "string"}
                <pre class="text-console m-0 text-green">{rightShiftLines(
                        (changes + "").trim(),
                        shift
                    )}</pre>
            {:else}
                <svelte:self changes={changeValue} shift={shift + 4} />
            {/if}
        {/each}
    {/if}
{:else}
    <pre class="text-console m-0 text-green">{rightShiftLines(
            (changes + "").trim(),
            shift
        )}</pre>
{/if}
