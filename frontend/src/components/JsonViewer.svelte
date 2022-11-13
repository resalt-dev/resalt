<script lang="ts">
    import { EditorView, basicSetup } from 'codemirror';
    import { ensureSyntaxTree, foldAll } from '@codemirror/language';
    import { EditorState } from '@codemirror/state';
    import { json } from '@codemirror/lang-json';
    import { onDestroy, onMount } from 'svelte';
    import { theme } from '../stores';
    import { resaltDark } from './codemirror-resalt-theme-dark';
    import { resaltLight } from './codemirror-resalt-theme-light';

    export let data: any;

    let editorElement: HTMLElement;
    let cm: EditorView = undefined;

    $: {
        if (cm) {
            if (
                !(data instanceof Array) &&
                JSON.stringify(data).includes('cpu_flags')
            ) {
                foldAll(cm);
            }
        }
    }

    function isObject(v: any): boolean {
        return '[object Object]' === Object.prototype.toString.call(v);
    }

    function sortJSON(o: any): any {
        if (Array.isArray(o)) {
            return o.sort().map(sortJSON);
        } else if (isObject(o)) {
            return Object.keys(o)
                .sort()
                .reduce(function (a, k) {
                    a[k] = sortJSON(o[k]);

                    return a;
                }, {});
        }
        return o;
    }

    function createJSONView() {
        let state = EditorState.create({
            doc: JSON.stringify(sortJSON(data), null, 2),
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
            console.error('Syntax tree parsing timed out.');
        }
        //cm.dispatch({});
    }

    let unsub = null;
    onMount(() => {
        if (unsub != null) {
            unsub();
            unsub = null;
        }
        unsub = theme.subscribe(createJSONView);
    });

    onDestroy(() => {
        unsub();
        editorElement.replaceChildren();
        cm = undefined;
    });
</script>

<div bind:this={editorElement} />
