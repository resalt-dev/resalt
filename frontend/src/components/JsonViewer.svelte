<script lang="ts">
    import { EditorView, basicSetup } from "codemirror";
    import { ensureSyntaxTree, foldAll } from "@codemirror/language";
    import { EditorState } from "@codemirror/state";
    import { json } from "@codemirror/lang-json";
    import { onDestroy, onMount } from "svelte";
    import { theme } from "../stores";
    import { resaltDark } from "./codemirror-resalt-theme-dark";
    import { resaltLight } from "./codemirror-resalt-theme-light";

    export let data: any;

    let editorElement: HTMLElement;
    let cm: EditorView = undefined;

    $: {
        if (cm) {
            createJSONView("$ update");

            if (!(data instanceof Array)) {
                foldAll(cm);
            }
        }
    }

    function createJSONView(caller: string) {
        console.log("createJSONView caller: " + caller);
        let state = EditorState.create({
            doc: JSON.stringify(data, null, 2),
            extensions: [
                basicSetup,
                $theme.dark ? resaltDark : resaltLight,
                EditorState.readOnly.of(true),
                json(),
            ],
        });
        cm = new EditorView({ state });
        editorElement.replaceChildren(cm.dom);
        let tree = ensureSyntaxTree(state, state.doc.length, 5000);
        if (tree !== null) {
            console.log(tree);
        } else {
            console.error("Syntax tree parsing timed out.");
        }

        //cm.dispatch({});
    }

    let unsub = null;
    onMount(() => {
        if (unsub != null) {
            unsub();
            unsub = null;
        }
        unsub = theme.subscribe((newTheme) => {
            createJSONView("theme.subscribe");
        });
    });

    onDestroy(() => {
        unsub();
        editorElement.replaceChildren();
        cm = undefined;
    });
</script>

<div bind:this={editorElement} />
