<script lang="ts">
	import ConsoleChangeBranch from '$component/ConsoleChangeBranch.svelte';
	import TerminalBox from '$component/TerminalBox.svelte';

	const SHIFT = 10;
	let internalCollapsed = false;

	export let color: string;
	export let num: number;
	export let sls: string;
	export let stateName: string;
	export let fun: string;
	export let name: string;
	export let result: boolean | null;
	export let comment: string;
	export let startTime: string;
	export let duration: number;
	export let changes: unknown;
	export let showCollapsed: boolean;
	export let collapsed: boolean;

	function leftPadToTotalLength(str: string, maxLength: number, char = ' ') {
		return char.repeat(maxLength - str.length) + str;
	}
	function rightShiftLinesExceptFirst(str: string, paddingLength: number, char = ' ') {
		// Append paddingLength of spaces to each line except the first
		let lines = str.split('\n');
		let firstLine = lines.shift() ?? '';
		let padding = '';
		for (let i = 0; i < paddingLength; i++) {
			padding += char;
		}
		let paddedLines = [firstLine];
		for (let line of lines) {
			paddedLines.push(padding + line);
		}
		return paddedLines.join('\n');
	}
</script>

{#if !((internalCollapsed || collapsed) && !showCollapsed)}
	<TerminalBox
		class="startside-{color}"
		toggleCollapse={() => (internalCollapsed = !internalCollapsed)}
		collapsed={internalCollapsed || collapsed}
	>
		<div slot="header">
			<span>{sls} : </span>
			{stateName}
			<small class="text-muted">({fun})</small>
			<small class="float-end text-muted pt-1">
				# {num + 1}
			</small>
		</div>
		<div slot="body" class="text-console text-{color}">
			<pre class="m-0">{leftPadToTotalLength('ID', SHIFT)}: {stateName}</pre>
			<pre class="m-0">{leftPadToTotalLength('Function', SHIFT)}: {fun}</pre>
			<pre class="m-0">{leftPadToTotalLength('Name', SHIFT)}: {name}</pre>
			<pre class="m-0">{leftPadToTotalLength('Result', SHIFT)}: <span
					style="text-transform:capitalize;">{result === null ? 'None' : result}</span
				></pre>
			<pre class="m-0">{leftPadToTotalLength('Comment', SHIFT)}: {rightShiftLinesExceptFirst(
					comment,
					SHIFT + 2,
				)}</pre>
			<pre class="m-0">{leftPadToTotalLength('Started', SHIFT)}: {startTime}</pre>
			<pre class="m-0">{leftPadToTotalLength('Duration', SHIFT)}: {duration}</pre>
			<pre class="m-0">{leftPadToTotalLength('Changes', SHIFT)}:</pre>
			{#if typeof changes === 'object' && !changes}
				<!-- {#if Object.keys(changes).length != 0} -->
				<ConsoleChangeBranch data={changes} shift={SHIFT + 2} />
				<!-- {/if} -->
			{/if}
		</div>
	</TerminalBox>
{/if}
