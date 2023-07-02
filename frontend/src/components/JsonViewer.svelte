<script lang="ts">
	import { EditorView, basicSetup } from 'codemirror';
	import { ensureSyntaxTree, foldAll } from '@codemirror/language';
	import { EditorState } from '@codemirror/state';
	import { json } from '@codemirror/lang-json';
	import { onDestroy, onMount } from 'svelte';
	import { theme } from '$lib/stores';
	import { resaltDark } from './codemirror-resalt-theme-dark';
	import { resaltLight } from './codemirror-resalt-theme-light';
	import type { Unsubscriber } from 'svelte/store';

	export let data: any;
	export let sort = true;

	let editorElement: HTMLElement;
	let cm: EditorView | undefined = undefined;

	$: {
		if (cm) {
			createJSONView();

			if (
				data !== undefined &&
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
			// Do NOT sort arrays
			return o;
		} else if (isObject(o)) {
			return Object.keys(o)
				.sort()
				.reduce(function (a: any, k: string) {
					a[k] = sortJSON(o[k]);

					return a;
				}, {});
		}
		return o;
	}

	function createJSONView() {
		let clone = data ? JSON.parse(JSON.stringify(data)) : undefined;
		if (clone && sort) {
			clone = sortJSON(clone);
		}
		let state = EditorState.create({
			doc: JSON.stringify(clone, null, 2),
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

	let unsub: Unsubscriber | null = null;
	onMount(() => {
		// Theme listener
		if (unsub != null) {
			unsub();
			unsub = null;
		}
		unsub = theme.subscribe(createJSONView);
	});

	onDestroy(() => {
		// Theme listener
		if (unsub != null) {
			unsub();
			unsub = null;
		}

		// Cleanup
		editorElement.replaceChildren();
		cm = undefined;
	});
</script>

<div class="d-none">
	<!-- This MUST be here to force Svelte to re-render on changes -->
	{data}
</div>
<div bind:this={editorElement} />
