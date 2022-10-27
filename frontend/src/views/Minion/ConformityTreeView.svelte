<script lang="ts">
    import Icon from '../../components/Icon.svelte';
    import type { ConformTreeNode } from './ConformityTypes';
    export let node: ConformTreeNode;
    export let depth: number = 0;
</script>

<!--
export type ConformTreeNode = {
    name: string;
    subtree: ConformTreeNode[];
    items: Conform[];
};
-->

<Icon name="folder-open" size="1" class="text-blue" />
{' '}{node.name}

{#if node.items.length > 0}
    <span>({node.items.length})</span>
{/if}

<ul>
    {#each node.subtree as subNode}
        <li>
            <svelte:self node={subNode} depth={depth + 1} />
        </li>
    {/each}
</ul>

<style lang="scss">
    span {
        font-size: 13px;
        font-style: italic;
        letter-spacing: 0.4px;
        color: #a8a8a8;
    }

    ul {
        margin-bottom: 0; // override bootstrap
        padding-left: 5px;
        list-style: none;

        li {
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
