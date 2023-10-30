<script lang="ts">
	import hljs from 'highlight.js/lib/core';
	import json from 'highlight.js/lib/languages/json';
	import { onMount } from 'svelte';
	import Icon from './Icon.svelte';
	import Clickable from './Clickable.svelte';
	hljs.registerLanguage('json', json);

	export let data: unknown;
	export let sort = true;

	let viewElement: HTMLElement;
	let collapsable: Array<string | undefined> = [];
	let collapsed: Set<string> = new Set(['cpu_flags']);
	let lineNumbers: Array<number> = [];

	function isObject(v: unknown): boolean {
		return '[object Object]' === Object.prototype.toString.call(v);
	}

	function sortJSON(o: unknown): unknown {
		if (Array.isArray(o)) {
			// Do NOT sort arrays
			return o;
		} else if (isObject(o)) {
			return Object.keys(o as object)
				.sort()
				.reduce(function (prev: { [fun: string]: unknown }, curr: string) {
					prev[curr] = sortJSON((o as { [fun: string]: unknown })[curr]);

					return prev;
				}, {});
		}
		return o;
	}

	function render(data: unknown) {
		let removedValues: Map<string, number> = new Map();

		// Clone if non-undefined
		let clone = data ? JSON.parse(JSON.stringify(data)) : undefined;
		// Sort if non-undefined and if sort=true
		if (clone && sort) {
			clone = sortJSON(clone);
		}
		// Check if cloen is falsy
		if (!clone) {
			return '';
		}
		// Collapse elements in root obj which are collaposed
		if (typeof clone === 'object') {
			for (const item of collapsed) {
				// Check if item exists in root object
				const value = clone[item];
				if (Array.isArray(value)) {
					clone[item] = [];
				} else if (typeof value === 'object') {
					clone[item] = {};
				}
				if (typeof value === 'object') {
					removedValues.set(item, JSON.stringify(value, null, 2).split('\n').length);
				}
			}
		}

		let res = hljs.highlight(JSON.stringify(clone, null, 2), { language: 'json' }).value;
		// replace /n with <br>
		res = res.replace(/\n/g, '<br>\n');
		// replace spacespace with &nbsp;&nbsp;
		res = res.replace(/ {2}/g, '&nbsp;&nbsp;');

		let lines = res.split('\n');
		collapsable = [];
		lineNumbers = [];
		let lastAdder = 1;
		for (let i = 0; i != lines.length; i++) {
			const isCollapsable =
				(lines[i].indexOf('{') != -1 || lines[i].indexOf('[') != -1) &&
				lines[i].startsWith('&nbsp;&nbsp;<span');
			// get value >"value"<
			const key = (lines[i].match(/>&quot;(.*)&quot;</) || [])[0]
				?.replaceAll('&quot;', '')
				.replaceAll('>', '')
				.replaceAll('<', '');
			collapsable[i] = isCollapsable ? key : undefined;

			const rB1 = '<span class="hljs-punctuation">[</span>';
			const rB2 = '<span class="hljs-punctuation">]</span>';
			const rB3 = '<span class="hljs-punctuation">{</span>';
			const rB4 = '<span class="hljs-punctuation">}</span>';
			const rDots = '<span class="no-select text-muted">&lt;collapsed&gt;</span>';

			if (typeof key === 'string' && collapsed.has(key)) {
				lines[i] = lines[i]
					.replaceAll(`${rB1}${rB2}`, `${rB1} ${rDots} ${rB2}`)
					.replaceAll(`${rB3}${rB4}`, `${rB3} ${rDots} ${rB4}`);
			}
			lineNumbers[i] = (lineNumbers[i - 1] ?? 0) + lastAdder;
			lastAdder = removedValues.get(key ?? '') ?? 1;
		}
		return lines.join('\n');
	}

	function toggleCollapse(entry: string | undefined) {
		if (!entry) {
			return;
		}

		let toggled = collapsed.has(entry);
		if (toggled) {
			collapsed.delete(entry);
		} else {
			collapsed.add(entry);
		}
		viewElement.innerHTML = render(data);
	}

	$: {
		if (viewElement) {
			viewElement.innerHTML = render(data);
		}
	}
	onMount(() => {
		viewElement.innerHTML = render(data);
	});
</script>

<div class="container-fluid font-monospace">
	<div class="row">
		<div class="col-auto text-primary text-end no-select">
			{#each lineNumbers as v}
				{v}
				<br />
			{/each}
		</div>
		<div class="col-auto p-0 text-primary text-end no-select">
			{#each [...Array(collapsable.length).keys()] as i}
				{#if collapsable[i]}
					<Clickable
						type="div"
						event={() => {
							toggleCollapse(collapsable[i]);
						}}
					>
						<Icon size="1" name="chevron-down" class="text-primary" />
					</Clickable>
				{:else}
					<br />
				{/if}
			{/each}
		</div>
		<div class="col text-nowrap" bind:this={viewElement} style="overflow-x: auto;"></div>
	</div>
</div>

<style>
	* {
		outline: 0px solid red;
	}
</style>
