<script lang="ts">
	import Clickable from '../../../components/Clickable.svelte';
	import Icon from '../../../components/Icon.svelte';
	import type { ConformTreeNode } from './ConformityTypes';
	export let node: ConformTreeNode;
	export let depth: number = 0;
	export let filterNamespace: string;
	export let collapseList: string[];

	$: sls = calculateFullNamespace(node);
	$: collapsed = collapseList.includes(sls);
	$: icon = collapsed
		? 'caret-up-square'
		: sls === filterNamespace
		? 'caret-right-square'
		: 'caret-down-square';

	function calculateFullNamespace(node: ConformTreeNode): string {
		// Traverse parents up
		let namespace = node.name;
		let parent = node.parent;
		while (parent && parent.name != '#') {
			namespace = parent.name + '.' + namespace;
			parent = parent.parent;
		}
		return namespace;
	}
</script>

<!--
export type ConformTreeNode = {
    name: string;
    color: string;
    subtree: ConformTreeNode[];
    items: Conform[];
};
-->

<Icon
	name={icon}
	size="1.5"
	class="text-{node.color} me-1 mouse-pointer"
	on:click={() => {
		if (collapseList.includes(sls)) {
			collapseList.splice(collapseList.indexOf(sls), 1);
		} else {
			collapseList.push(sls);
		}
		// Trigger svelte bind update
		collapseList = [...collapseList];
	}}
/>
<Clickable
	type="span"
	disabled={node.name === '#'}
	event={() => {
		if (sls === filterNamespace) {
			filterNamespace = '';
		} else {
			filterNamespace = sls;
		}
	}}
>
	<span class={sls === filterNamespace ? 'fw-bold text-orange' : ''}>
		{#if node.name === '#'}
			<span>top.sls</span>
		{:else}
			{node.name}
		{/if}
	</span>

	{#if node.items.length > 0}
		<em class="text-muted">
			({node.items.length}{#if collapsed}+...{/if})
		</em>
	{:else if collapsed}
		<em class="text-muted">(...)</em>
	{/if}
</Clickable>

{#if !collapsed}
	<ul>
		{#each node.subtree as subNode}
			<li>
				<svelte:self
					node={subNode}
					depth={depth + 1}
					bind:filterNamespace
					bind:collapseList
				/>
			</li>
		{/each}
	</ul>
{/if}

<style>
	ul {
		margin-bottom: 0;
		padding-left: 5px;
		list-style: none;

		& li {
			position: relative;
			padding-top: 5px;
			padding-bottom: 5px;
			padding-left: 15px;
			-webkit-box-sizing: border-box;
			-moz-box-sizing: border-box;
			box-sizing: border-box;

			&:before {
				position: absolute;
				top: 15px;
				left: 0;
				width: 10px;
				height: 1px;
				margin: auto;
				content: '';
				background-color: #666;
			}

			&:after {
				position: absolute;
				top: 0;
				bottom: 0;
				left: 0;
				width: 1px;
				height: 100%;
				content: '';
				background-color: #666;
			}

			&:last-child:after {
				height: 15px;
			}
		}
	}
</style>
