<script lang="ts">
	import { useLocation, Link, useNavigate } from 'svelte-navigator';
	import { currentUser, socket, theme, toasts } from '../../stores';
	import paths from '../../paths';
	import { Col, Row } from 'sveltestrap';
	import Icon from '../../components/Icon.svelte';
	import { logout } from '../../api';
	import { MessageType } from '../../models/MessageType';
	import Clickable from '../../components/Clickable.svelte';

	const navigate = useNavigate();
	const location = useLocation();

	$: isAuth = $location.pathname.indexOf('/auth/') !== -1;
	$: navbar = isAuth
		? []
		: $location.pathname
				.split('/')
				.filter(Boolean)
				.map((str) => {
					return {
						title: str.startsWith('usr_')
							? str
							: str.charAt(0).toUpperCase() + str.slice(1),
						path: paths[str.toLowerCase()]?.getPath(),
					};
				});
</script>

<Row id="dashboard-header" class="g-0 d-flex align-items-center bg-light">
	<Col>
		<div class="btn-group me-3" role="group">
			{#each navbar as item}
				{#if item.path}
					<Link
						to={item.path}
						class={`btn btn-${$theme.color} ${
							$theme.color === 'yellow' ? '' : 'text-white'
						} btn-arrow-right fw-bold`}>{item.title}</Link
					>
				{:else}
					<div class="btn btn-dark text-white btn-arrow-right fw-bold">
						{item.title}
					</div>
				{/if}
			{/each}
		</div>
	</Col>
	<Col xs="auto" class="pe-3 d-flex align-items-center">
		{#if $socket.connected}
			<!-- display lastPing as hh:mm:ss -->
			<span class="font-monospace pt-1 ps-3"
				>Connected: {new Date($socket.lastPing)
					.toLocaleTimeString('en-US', {
						timeZone: 'UTC',
						timeZoneName: 'short',
						hourCycle: 'h23',
					})
					.replace(/\./g, ':')}</span
			>
		{:else}
			<span class="font-monospace pt-1 text-danger">Disconnected</span>
		{/if}
	</Col>
	<Col xs="auto">
		<div class="vr sep" />
	</Col>
	<Col xs="auto" class="px-3 text-reset text-decoration-none">
		<Link to={paths.user.getPath($currentUser.id)} class="text-decoration-none text-reset">
			<Icon name="user" size="1.5" type="solid" class="pe-1" />
			{$currentUser.username}
		</Link>
	</Col>
	<!-- <Col xs="auto">
		<div class="vr sep" />
	</Col>
	<Col xs="auto" class="px-3 text-reset text-decoration-none">
		<Icon name="bell" size="1.5" />
	</Col> -->
	<Col xs="auto">
		<div class="vr sep" />
	</Col>
	<Clickable
		type="div"
		event={() => {
			logout()
				.then(() => {
					toasts.add(
						MessageType.SUCCESS,
						'Logout Success',
						'You have now been logged out.',
					);
					navigate(paths.login.getPath());
				})
				.catch((err) => {
					toasts.add(MessageType.ERROR, 'Logout Error', err);
				});
		}}
		class="col-auto px-3 text-reset text-decoration-none mouse-pointer"
	>
		<Icon name="log-out" size="1.5" class="pe-1" />
		Logout
	</Clickable>
</Row>

<style lang="scss">
	.sep {
		margin-top: 0.4rem;
		height: 1.5rem;
	}
</style>
