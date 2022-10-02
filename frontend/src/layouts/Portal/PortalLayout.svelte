<script lang="ts">
    import { Router, Route } from 'svelte-navigator';
    import { writable, type Writable } from 'svelte/store';
    import { Card, CardBody, CardText, CardTitle } from 'sveltestrap';
    import { toasts, theme } from '../../stores';
    import AuthLogin from '../../views/Auth/AuthLogin.svelte';
    import AuthLogout from '../../views/Auth/AuthLogout.svelte';
    import constants from '../../constants';
    import Logo from '../../components/Logo.svelte';
    import type Alert from '../../models/Alert';

    let localAlerts: Writable<Alert[]> = writable([]);

    // Clone "alerts" to "localAlerts" then empty it on every Svelte update
    $: {
        localAlerts.set($toasts);
        toasts.set([]);
    }
</script>

<div
    class="background1 h-100"
    style="background-image: url('{constants.basePath}/assets/images/0da7530ac9cd4c88850c62138da12e66.jpg');"
>
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

                            {#each $localAlerts as alert}
                                <Card class="text-white bg-{alert.type} mb-3">
                                    <CardBody>
                                        <CardTitle>{alert.title}</CardTitle>
                                        <CardText>{alert.message}</CardText>
                                    </CardBody>
                                </Card>
                            {/each}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    .background1 {
        background: var(--dark);
        background-size: cover;
        background-position: center;
        background-repeat: no-repeat;
        background-attachment: fixed;
    }
</style>
