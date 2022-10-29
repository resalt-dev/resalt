<script lang="ts">
    import { Router, Route } from 'svelte-navigator';
    import { Toast, ToastBody, ToastHeader } from 'sveltestrap';
    import { toasts, theme } from '../../stores';
    import AuthLogin from '../../views/Auth/AuthLogin.svelte';
    import AuthLogout from '../../views/Auth/AuthLogout.svelte';
    import Logo from '../../components/Logo.svelte';
</script>

<div class="portal-background h-100">
    <div class="h-100 w-100">
        <div class="row g-0 h-100">
            <!-- Right side -->
            <div class="offset-8 col-4 h-100 bg-white">
                <div
                    class="row h-100 g-0 justify-content-center align-items-center"
                >
                    <div class="col-12">
                        <!-- Title -->
                        <div class="m-3 px-5 py-3">
                            <Logo color={$theme.color} />
                        </div>

                        <hr class="mx-5 my-3 bg-light" />

                        <!-- Content -->
                        <div class="px-5 py-4" style="max-width: 54rem;">
                            <Router primary={false}>
                                <Route path="login" component={AuthLogin} />
                                <Route path="logout" component={AuthLogout} />
                            </Router>

                            <div class="clearfix" />

                            {#each $toasts as toast}
                                <Toast class="{'toast-' + toast.type} mb-2">
                                    <ToastHeader>{toast.title}</ToastHeader>
                                    <ToastBody>{toast.message}</ToastBody>
                                </Toast>
                            {/each}
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
        background-image: url('/assets/images/0da7530ac9cd4c88850c62138da12e66.jpg');
        background-size: cover;
        background-position: center;
        background-repeat: no-repeat;
        background-attachment: fixed;
    }
</style>
