<script lang="ts">
	import paths from '../../paths';
	import { sidebarCollapsed as collapsed, theme, config, currentUser } from '../../stores';
	import Icon from '../../components/Icon.svelte';
	import Logo from '../../components/Logo.svelte';
	import SidebarItem from './DashboardSidebarItem.svelte';
	import constants from '../../constants';
	import Clickable from '../../components/Clickable.svelte';
	import { Modal } from '../../../assets/js/bootstrap.esm-5.3.0.min';
	import { v4 as uuidv4 } from 'uuid';

	const randomId = uuidv4();

	function handleClickCollapse(): void {
		collapsed.update((n) => !n);
	}

	function showDialog(): void {
		// Show dialog BS5
		const modalWidget = document.getElementById(randomId);
		if (modalWidget !== null) {
			const modal = new Modal(modalWidget);
			modal.show();
		}
	}

	function hideDialog(): void {
		// Hide dialog BS5
		const modalWidget = document.getElementById(randomId);
		if (modalWidget !== null) {
			const modal = Modal.getInstance(modalWidget);
			if (modal !== null) {
				modal.hide();
			}
		}
	}
</script>

<div
	class="d-flex flex-column flex-shrink-0 bg-dark h-100 no-select"
	style="min-height: 100vh; overflow-y: auto; {$collapsed ? 'width: 4.5rem;' : 'width: 17.5rem;'}"
>
	<Clickable type="div" event={handleClickCollapse} class="d-flex text-decoration-none">
		<div
			class="d-flex align-items-center py-4 {$collapsed ? 'mx-auto' : 'w-100'}"
			style="height: 80px"
		>
			<div class="px-5 py-3 {$collapsed ? 'd-none' : 'w-100'}">
				<Logo color={$theme.color} />
			</div>
			<Icon name="menu" class="mb-0 h3 text-white {!$collapsed && 'd-none'}" />
		</div>
	</Clickable>

	<hr class="mt-0 mb-3" />

	<ul
		class="nav nav-pills flex-column mb-auto fs-5 {$collapsed
			? 'nav-flush text-center'
			: 'mx-2'}"
	>
		{#each Object.values(paths) as path}
			{#if path.showInNav && path.hasPermission($currentUser)}
				{#if path.name.startsWith('_')}
					<li><hr /></li>
				{:else}
					<SidebarItem {path} collapsed={$collapsed} />
				{/if}
			{/if}
		{/each}
	</ul>

	<hr class="mb-0" />

	<Clickable
		type="div"
		event={handleClickCollapse}
		class="btn-dark bg-dark border-0 pt-3 pb-3 px-3 fw-light d-flex align-items-center"
	>
		<Icon
			name={$collapsed ? 'right-arrow-alt' : 'left-arrow-alt'}
			class={$collapsed ? '' : 'me-3'}
			size="2.5"
			style="margin-bottom: -2px;"
		/>
		{#if !$collapsed}
			<span class="fs-5">Collapse</span>
		{/if}
	</Clickable>

	<hr class="mt-0 mb-0" />

	{#if $config.latestVersion === 'unknown'}
		<Clickable
			type="span"
			event={showDialog}
			class="text-center link-danger text-decoration-underline"
		>
			{#if $collapsed}
				{$config.currentVersion}
			{:else}
				<Icon name="chevrons-up" />
				{constants.appName} - {$config.currentVersion}
				<Icon name="chevrons-up" />
			{/if}
		</Clickable>
	{:else if $config.currentVersion !== $config.latestVersion}
		<Clickable
			type="span"
			event={showDialog}
			class="text-center link-warning text-decoration-underline"
		>
			{#if $collapsed}
				{$config.currentVersion}
			{:else}
				<Icon name="chevrons-up" />
				{constants.appName} - {$config.currentVersion}
				<Icon name="chevrons-up" />
			{/if}
		</Clickable>
	{:else}
		<span class="text-center text-secondary">
			{#if $collapsed}
				{$config.currentVersion}
			{:else}
				{constants.appName} - {$config.currentVersion}
			{/if}
		</span>
	{/if}
</div>

<div>
	<div class="modal {$theme.dark ? 'theme-dark' : ''}" tabindex="-1" role="dialog" id={randomId}>
		<div class="modal-dialog">
			<div class="modal-content">
				<Clickable
					type="div"
					event={hideDialog}
					class="modal-header {$config.latestVersion === 'unknown'
						? 'bg-danger'
						: 'bg-warning text-dark'} text-dark"
				>
					{#if $config.latestVersion === 'unknown'}
						Update Error!
					{:else}
						Update Warning
					{/if}
				</Clickable>
				<div class="modal-body">
					{#if $config.latestVersion === 'unknown'}
						<h1>
							<span class="update-label">Current: </span>
							<span class="badge bg-{$theme.color}">{$config.currentVersion}</span>
						</h1>
						<br />
						There was a critical error while trying to check for updates. Especially in a
						software that interracts with SaltStack, it is
						<b>CRITICAL</b> to run the latest version for security reasons.
						<br />
						<br />
						Double-check that the Resalt container is able to access
						<code>secure.resalt.dev</code> and without a proxy. Please contact your administrator,
						or the Resalt development team, if this issue persists.
					{:else}
						<h1>
							<span class="update-label">Current: </span>
							<span class="badge bg-{$theme.color}">{$config.currentVersion}</span>
						</h1>
						<h1>
							<span class="update-label">Latest: </span>
							<span class="badge bg-{$theme.color}">{$config.latestVersion}</span>
						</h1>
						<br />
						By not upgrading, you risk compromising the security and integrity of your infrastructure
						by not taking use of the latest bug fixes and security patches.
						<br />
						<hr class="text-light" />
						You can upgrade by increasing the version number of the Docker image in your
						compose/stack file to the latest version. If you have any questions, please reach
						out on GitHub:<a
							target="_blank"
							href={constants.githubUrl}
							rel="noreferrer"
						>
							{constants.githubUrl}</a
						>
					{/if}
				</div>
				<div class="modal-footer">
					<button type="button" class="btn btn-secondary" on:click={hideDialog}>
						Close
					</button>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.update-label {
		width: 150px;
		display: inline-block;
	}
</style>
