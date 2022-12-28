<script lang="ts">
	import { Router, Route, type NavigatorHistory } from 'svelte-navigator';
	import { auth, theme } from '../../stores';
	import AuthLogin from '../../views/Auth/AuthLogin.svelte';
	import Logo from '../../components/Logo.svelte';
	import Redirect from '../../components/Redirect.svelte';
	import paths from '../../paths';

	export let history: NavigatorHistory;
</script>

<div class="portal-background h-100">
	<div class="h-100 w-100">
		<div class="row g-0 h-100">
			<!-- Right side -->
			<div class="offset-8 col-4 h-100 bg-white">
				<div class="row h-100 g-0 justify-content-center align-items-center">
					<div class="col-12">
						<!-- Title -->
						<div class="m-3 px-5 py-3">
							<Logo color={$theme.color} />
						</div>

						<hr class="bg-light mx-5 my-3" />

						<!-- Content -->
						<div class="px-5 py-4" style="max-width: 54rem;">
							<Router primary={false} {history}>
								<Route path="auth/login" component={AuthLogin} />
								<!-- <Route
                                    path="auth/reset"
                                    component={AuthReset}
                                /> -->
								<Route path="*">
									{#if $auth === null}
										<Redirect to={paths.login.getPath()} />
									{:else}
										<Redirect to={paths.dashboard.getPath()} />
									{/if}
								</Route>
							</Router>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.portal-background {
		background: var(--dark);
		background-image: url('/assets/img/0da7530ac9cd4c88850c62138da12e66.jpg');
		background-size: cover;
		background-position: center;
		background-repeat: no-repeat;
		background-attachment: fixed;
	}
</style>
