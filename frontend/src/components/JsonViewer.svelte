<script lang="ts">
    import {
        EditorState,
        EditorView,
        basicSetup,
    } from "@codemirror/basic-setup";
    import { ensureSyntaxTree } from "@codemirror/language";
    import { json } from "@codemirror/lang-json";
    import { onDestroy, onMount } from "svelte";
    import { theme } from "../stores";

    export let data: any;

    let editorElement: HTMLElement | null = null;
    let cm = undefined;

    $: dataFormatted = JSON.stringify(data, null, 2);
    $: {
        if (cm) {
            // Update value
            cm.dispatch({
                changes: {
                    from: 0,
                    to: cm.state.doc.length,
                    insert: dataFormatted,
                },
            });
            console.log(cm);
            console.log("code view updated!");
        }
    }

    onMount(() => {
        let state = EditorState.create({
            doc: dataFormatted,
            extensions: [basicSetup, EditorState.readOnly.of(true), json()],
        });
        cm = new EditorView({ state });
        editorElement.replaceChildren(cm.dom);
        ensureSyntaxTree(state, state.doc.length, 5000);
        cm.dispatch({});
    });

    onDestroy(() => {
        editorElement.replaceChildren();
        cm = undefined;
    });
</script>

<div
    class="cm-resalt cm-resalt-{$theme.dark ? 'dark' : 'light'}"
    bind:this={editorElement}
/>

<style>
    :global(.cm-resalt-dark) {
        background: var(--black);
        color: var(--light);
    }

    :global(.cm-resalt-light) {
        background: var(--white);
        color: var(--black);
    }

    /* string */
    :global(.cm-resalt .ͼe) {
        color: var(--primary);
    }

    /* bool */
    :global(.cm-resalt .ͼc) {
        color: var(--orange);
        font-weight: bold;
    }

    /* number */
    :global(.cm-resalt .ͼd) {
        color: var(--magenta);
    }

    /* null */
    :global(.cm-resalt .ͼb) {
        color: var(--purple);
        font-style: italic;
    }
</style>
