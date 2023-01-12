<script lang="ts">
	import { Row, Col, Button, Card, CardHeader, CardBody } from 'sveltestrap';
	import Icon from '../../components/Icon.svelte';
	import constants from '../../constants';
	import { MessageType } from '../../models/MessageType';
	import { config, currentUser, theme, toasts } from '../../stores';
	import Clickable from '../../components/Clickable.svelte';
	import { hasResaltPermission, P_ADMIN_SUPERADMIN } from '../../perms';

	function selectColor(color: string): void {
		console.log('selectColor', color);
		if (color === 'reset') {
			$theme.color = $config.themeDefaultColor;
		} else {
			$theme.color = color;
		}
	}

	function setDarkMode(dark: boolean): void {
		console.log('toggleDarkMode');
		theme.update((t) => ({ ...t, dark: dark }));
	}
</script>

{#if $config.themeEnableSwitching}
	<Card class="mb-3">
		<CardHeader>Color</CardHeader>
		<CardBody>
			<Row>
				{#each constants.themeColors as color}
					<Col xs="auto">
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
					</Col>
				{/each}
			</Row>

			<Button color={null} class="btn-{$theme.color}" on:click={() => selectColor('reset')}>
				Reset
			</Button>
		</CardBody>
	</Card>
{/if}

<Card class="mb-0">
	<CardHeader>Dark mode</CardHeader>
	<CardBody>
		<Row>
			<Col xs="auto">
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
			</Col>
			<Col xs="auto">
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
			</Col>
		</Row>

		<Button
			color={null}
			class="btn-{$theme.color}"
			on:click={() => setDarkMode($config.themeDefaultDark)}
		>
			Reset
		</Button>

		{#if hasResaltPermission($currentUser, P_ADMIN_SUPERADMIN)}
			<Button
				color="warning"
				on:click={() => {
					toasts.add(
						MessageType[
							Object.keys(MessageType).filter((k) => isNaN(Number(k)))[$toasts.length]
						],
						'Testing toast!',
						"This is a test toast message. It's a bit longer than the others, but that's okay.",
					);
				}}
			>
				Show toast
			</Button>
		{/if}
	</CardBody>
</Card>

<style lang="scss">
	$theme-box-size: 75px;
	:global(.theme-selector-box) {
		width: $theme-box-size;
		height: $theme-box-size;
		border-radius: 10px;
		border: 6px solid;

		display: flex;
		justify-content: center; /* align horizontal */
		align-items: center; /* align vertical */
	}
</style>
