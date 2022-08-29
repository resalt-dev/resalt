<script lang="ts">
    import { EditorView, basicSetup } from "codemirror";
    import {
        ensureSyntaxTree,
        foldInside,
        foldNodeProp,
        foldAll,
        foldCode,
        foldable,
    } from "@codemirror/language";
    import { EditorState, EditorSelection } from "@codemirror/state";
    import { json } from "@codemirror/lang-json";
    import { onDestroy, onMount } from "svelte";
    import { theme } from "../stores";
    import { resaltDark } from "./codemirror-resalt-theme-dark";
    import { resaltLight } from "./codemirror-resalt-theme-light";
    import type {
        Tree,
        TreeBuffer,
        SyntaxNode,
        SyntaxNodeRef,
    } from "@lezer/common";

    export let data: any;

    let editorElement: HTMLElement;
    let cm: EditorView = undefined;
    let tree: Tree | null = null;

    $: {
        if (cm) {
            createJSONView("$ update");

            // Find all array nodes
            let arrayNodes: SyntaxNode[] = [];
            tree.iterate({
                enter: (ref: SyntaxNodeRef) => {
                    //console.log("enter", node.type.name, node);
                    if (ref.type.name === "Array") {
                        console.log("Array", ref.type.name, ref);
                        arrayNodes.push(ref.node);
                        foldInside(ref.node);
                    }
                },
                leave: (node) => {
                    //console.log("leave", node);
                },
                from: 0,
                to: tree.length,
            });

            console.log("arrayNodes", arrayNodes);

            if (!(data instanceof Array)) {
                foldAll(cm);
            }

            // Convert SyntaxNode's to SelectionRange's

            // Select them
            cm.dispatch({
                //selection: EditorSelection.create(
            });
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
        tree = ensureSyntaxTree(state, state.doc.length, 5000);
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
