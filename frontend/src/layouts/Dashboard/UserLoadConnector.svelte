<script lang="ts">
	import { onMount } from 'svelte';
	import { useNavigate } from 'svelte-navigator';
	import { currentUser, toasts } from '../../stores';
	import { getCurrentUser, logout } from '../../api';
	import paths from '../../paths';
	import { MessageType } from '../../models/MessageType';

	const navigate = useNavigate();

	onMount(() => {
		getCurrentUser()
			.then((data) => {
				currentUser.set(data);
			})
			.catch((err) => {
				console.error(err);
				logout()
					.then(() => {
						toasts.add(
							MessageType.WARNING,
							'Logged out',
							'You have been logged out due to the token being invalid.',
						);
						navigate(paths.login.getPath());
					})
					.catch((err) => {
						toasts.add(MessageType.ERROR, 'Logout Error', err);
					});
			});
	});
</script>

<div />
