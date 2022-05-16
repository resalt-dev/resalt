<script>
    import { Router, Route } from "svelte-navigator";
    import constants from "../constants";
    import AuthLogin from "../views/AuthLogin.svelte";
    import AuthLogout from "../views/AuthLogout.svelte";
</script>

<!-- style_old="background-image: url('{constants.basePath}/assets/images/denisse-leon-OVEWbIgffDk-unsplash.jpg')" -->
<div
    class="background1 h-100"
    style="background-image: url('{constants.basePath}/assets/images/40-96-0-c-n.jpg')"
>
    <div class="background2 h-100 w-100">
        <!-- Title -->
        <div class="row g-0">
            <div class="col-9 offset-2 px-5" style="padding-top: 30rem;">
                <h1 class="text-uppercase text-white">
                    {constants.appName} Admin Console
                </h1>
            </div>
        </div>
        <!-- Content -->
        <div class="row g-0">
            <div class="col-2 bg-light p-5" />
            <div class="col-6 bg-white p-5">
                <Router primary={false}>
                    <Route path="login" component={AuthLogin} />
                    <Route path="logout" component={AuthLogout} />
                </Router>
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
    .background2 {
        /* background: rgba(0, 0, 0, 1); */
        background: rgba(255, 255, 255, 0.25);
    }
</style>
