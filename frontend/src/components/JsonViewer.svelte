<script lang="ts">
    import {
        EditorState,
        EditorView,
        basicSetup,
    } from "@codemirror/basic-setup";
    import { ensureSyntaxTree } from "@codemirror/language";
    import { json } from "@codemirror/lang-json";
    import { onDestroy, onMount } from "svelte";

    export let code;

    let editorElement;
    let view = undefined;

    $: code = JSON.stringify(JSON.parse(code), null, 2);
    $: {
        if (view) {
            // Update value
            view.dispatch({
                changes: {
                    from: 0,
                    to: view.state.doc.length,
                    insert: code,
                },
            });
            console.log("code view updated!");
        }
    }

    onMount(() => {
        let state = EditorState.create({
            doc: code,
            extensions: [basicSetup, EditorState.readOnly.of(true), json()],
        });
        view = new EditorView({ state });
        editorElement.replaceChildren(view.dom);
        ensureSyntaxTree(state, state.doc.length, 5000);
        view.dispatch({});
    });

    onDestroy(() => {
        editorElement.replaceChildren();
        view = undefined;
    });
</script>

<div bind:this={editorElement} />

<style>
    :global(.CodeMirror) {
        height: 100%;
    }
</style>
