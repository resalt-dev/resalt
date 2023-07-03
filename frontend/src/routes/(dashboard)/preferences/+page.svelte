<script lang="ts">
	import Icon from '../../../components/Icon.svelte';
	import constants from '$lib/constants';
	import { MessageType } from '../../../models/MessageType';
	import { config, currentUser, theme, toasts } from '$lib/stores';
	import Clickable from '../../../components/Clickable.svelte';
	import { hasResaltPermission, P_ADMIN_SUPERADMIN } from '$lib/perms';

	function selectColor(color: string): void {
		console.log('selectColor', color);
		if (color === 'reset') {
			$theme.color = $config?.themeDefaultColor ?? 'primary';
		} else {
			$theme.color = color;
		}
	}

	function setDarkMode(dark: boolean): void {
		console.log('toggleDarkMode');
		theme.update((t) => ({ ...t, dark: dark }));
	}
</script>

<svelte:head>
	<title>Preferences</title>
</svelte:head>

{#if $config?.themeEnableSwitching ?? true}
	<div class="card mb-3">
		<div class="card-header">Color</div>
		<div class="card-body">
			<div class="row">
				{#each constants.themeColors as color}
					<div class="col-auto">
						<Clickable
							type="div"
							event={() => selectColor(color)}
							class="theme-selector-box bg-{color} mb-4 border-{$theme.dark
								? 'secondary'
								: 'light'}"
						>
							{#if $theme.color === color}
								<Icon
									name="check"
									style="color: {color === 'yellow'
										? 'black'
										: 'white'} !important;"
									size="3"
								/>
							{/if}
						</Clickable>
					</div>
				{/each}
			</div>

			<button
				type="button"
				class="btn btn-{$theme.color}"
				on:click={() => selectColor('reset')}
			>
				Reset
			</button>
		</div>
	</div>
{/if}

<div class="card mb-0">
	<div class="card-header">Dark mode</div>
	<div class="card-body">
		<div class="row">
			<div class="col-auto">
				<Clickable
					type="div"
					event={() => setDarkMode(false)}
					class="theme-selector-box mb-4 border-{$theme.dark ? 'secondary' : 'light'}"
					style="background-color: #fff;"
				>
					{#if $theme.dark === false}
						<Icon name="check" class="text-black" size="3" />
					{/if}
				</Clickable>
			</div>
			<div class="col-auto">
				<Clickable
					type="div"
					event={() => setDarkMode(true)}
					class="theme-selector-box bg-dark mb-4 border-{$theme.dark
						? 'secondary'
						: 'light'}"
				>
					{#if $theme.dark === true}
						<Icon name="check" class="text-white" size="3" />
					{/if}
				</Clickable>
			</div>
		</div>

		<button
			type="button"
			class="btn btn-{$theme.color}"
			on:click={() => setDarkMode($config?.themeDefaultDark ?? false)}
		>
			Reset
		</button>

		{#if hasResaltPermission($currentUser, P_ADMIN_SUPERADMIN)}
			<button
				class="btn btn-warning"
				on:click={() => {
					toasts.add(
						$toasts.length < Object.values(MessageType).length
							? Object.values(MessageType)[$toasts.length]
							: MessageType.SUCCESS,
						'Testing toast!',
						"This is a test toast message. It's a bit longer than the others, but that's okay.",
					);
				}}
			>
				Show toast
			</button>
		{/if}
	</div>
</div>

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
