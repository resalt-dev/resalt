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
    import { EditorState } from "@codemirror/state";
    import { json } from "@codemirror/lang-json";
    import { onDestroy, onMount } from "svelte";
    import { theme } from "../stores";
    import { resaltDark } from "./codemirror-resalt-theme-dark";
    import { resaltLight } from "./codemirror-resalt-theme-light";
    import { Tree, SyntaxNode } from "@lezer/common";

    export let data: any;

    let editorElement: HTMLElement;
    let cm: EditorView = undefined;
    let tree: Tree | null = null;

    $: {
        if (cm) {
            createJSONView("$ update");
            let nodes = findLongNodes(tree);
        }
    }

    function findLongNodes(node: Tree): SyntaxNode[] {
        let nodes: SyntaxNode[] = [];
        if (node.type.name === "Array") {
            console.log("Array", node);
        }
        for (let child of node.children) {
            if (child instanceof Tree) {
                nodes.push(...findLongNodes(child));
            } else {
                // console.log("TreeBuffer", child);
            }
        }
        return nodes;
    }

    // const foldAllByNode = (view: EditorView, node: SyntaxNode) => {
    //     let { state } = view,
    //         effects = [];
    //     for (let pos = 0; pos < state.doc.length; ) {
    //         let line = view.lineBlockAt(pos),
    //             range = foldable(state, line.from, line.to);
    //         //if (range) effects.push(foldEffect.of(range));
    //         pos = (range ? view.lineBlockAt(range.to) : line).to + 1;
    //     }
    //     if (effects.length) view.dispatch({ effects });
    //     return !!effects.length;
    // };

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

    onMount(() => {
        theme.subscribe((newTheme) => {
            createJSONView("theme.subscribe");
        });
    });

    onDestroy(() => {
        editorElement.replaceChildren();
        cm = undefined;
    });
</script>

<div bind:this={editorElement} />
