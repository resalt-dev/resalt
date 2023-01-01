<script lang="ts">
	export let onclick: () => void;
	export let label: string;
</script>

<button
	class="btn btn-light float-end border border-1 rounded-none"
	style="margin-top: -1rem;z-index: 4;position: absolute;right: 0;"
	on:click={onclick}
>
	{label}
</button>
