<script lang="ts">
	import Clickable from '$component/Clickable.svelte';
	import Icon from '$component/Icon.svelte';
	import constants from '$lib/constants';
	import { P_ADMIN_SUPERADMIN, hasResaltPermission } from '$lib/perms';
	import { currentUser, theme, toasts } from '$lib/stores';
	import { MessageType } from '$model/MessageType';

	function selectColor(color: string): void {
		console.log('selectColor', color);
		if (color === 'reset') {
			$theme.color = 'primary';
		} else {
			$theme.color = color;
		}
	}
</script>

<svelte:head>
	<title>Preferences</title>
</svelte:head>

<div class="card mb-3">
	<div class="card-header">Color</div>
	<div class="card-body">
		<div class="row">
			{#each constants.themeColors as color}
				<div class="col-auto">
					<Clickable
						type="div"
						event={() => selectColor(color)}
						class="theme-selector-box bg-{color} mb-4 border-light"
					>
						{#if $theme.color === color}
							<Icon
								name="check"
								style="color: {color === 'yellow' ? 'black' : 'white'} !important;"
								size="3"
							/>
						{/if}
					</Clickable>
				</div>
			{/each}
		</div>

		<button type="button" class="btn btn-{$theme.color}" on:click={() => selectColor('reset')}>
			Reset
		</button>
	</div>
</div>

{#if hasResaltPermission($currentUser, P_ADMIN_SUPERADMIN)}
	<div class="card mb-0">
		<div class="card-header">Debug</div>
		<div class="card-body">
			<button
				class="btn btn-warning"
				on:click={() => {
					toasts.add(
						Object.values(MessageType)[
							$toasts.length % Object.values(MessageType).length
						],
						'Testing toast!',
						"This is a test toast message. It's a bit longer than the others, but that's okay.",
					);
				}}
			>
				Show toast
			</button>
		</div>
	</div>
{/if}

<style>
	:global(.theme-selector-box) {
		width: 75px;
		height: 75px;
		border-radius: 10px;
		border: 6px solid;

		display: flex;
		justify-content: center; /* align horizontal */
		align-items: center; /* align vertical */
	}
</style>
