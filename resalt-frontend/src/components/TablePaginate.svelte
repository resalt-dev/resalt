<script lang="ts">
	import Icon from './Icon.svelte';

	export let size: number;
	export let page: number;
	export let last: boolean;
	export let updateData: () => void;
	export let resizeable = true;

	function paginateIncrement(): void {
		if (last) {
			return;
		}
		page++;
		updateData();
	}
	function paginateDecrement(): void {
		if (page === 1) {
			return;
		}
		page--;
		updateData();
	}
	function paginateFirst(): void {
		page = 1;
		updateData();
	}
	function setSize(newSize: number): void {
		size = newSize;
		page = 1;
		updateData();
	}
</script>

<div class="nav bg-dark w-100 justify-content-start no-select">
	<div
		aria-hidden="true"
		class="nav-link fw-bold mouse-pointer {page === 1 ? 'text-secondary' : 'text-white'}"
		on:click={paginateFirst}
	>
		&lt;&lt;
	</div>
	<div
		aria-hidden="true"
		class="nav-link fw-bold mouse-pointer {page === 1 ? 'text-secondary' : 'text-white'}"
		on:click={paginateDecrement}
	>
		&lt;
	</div>
	<div
		aria-hidden="true"
		class="nav-link fw-bold mouse-pointer {last ? 'text-secondary' : 'text-white'}"
		on:click={paginateIncrement}
	>
		&gt;
	</div>
	{#if resizeable}
		<div class="nav-item dropdown ms-3">
			<span
				class="nav-link text-white mouse-pointer"
				id="dropdownPaginatePageSize"
				role="button"
				data-bs-toggle="dropdown"
				aria-expanded="false"
			>
				Page size ({size})
				<Icon name="caret-down" size="1.125" />
			</span>
			<ul
				class="dropdown-menu dropdown-menu-dark ms-5 bg-secondary"
				aria-labelledby="dropdownPaginatePageSize"
			>
				{#each [20, 50, 100, 250] as s}
					<li>
						<span
							aria-hidden="true"
							class="dropdown-item mouse-pointer {size === s ? 'fw-bold' : ''}"
							on:click={() => setSize(s)}>{s}</span
						>
					</li>
				{/each}
			</ul>
		</div>
	{/if}
	<div class="nav-link text-muted">
		<small>
			Showing page {page}
		</small>
	</div>
</div>

<style>
	.dropdown-menu {
		min-width: auto;
	}
</style>
