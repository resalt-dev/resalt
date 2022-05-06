<script>
    import { onMount } from "svelte";
    import { Router, Route, useNavigate } from "svelte-navigator";
    import { user } from "../stores";
    import { load_user } from "../controller";
    import paths from "../paths";
    import Sidebar from "../components/Sidebar/Sidebar.svelte";
    import Topbar from "../components/Topbar.svelte";
    import Redirect from "../components/Redirect.svelte";

    import Home from "../views/Home.svelte";
    import Minions from "../views/Minions.svelte";
    import Minion from "../views/Minion/Minion.svelte";

    const navigate = useNavigate();

    onMount(() => {
        load_user(navigate);
    });
</script>

{#if $user == null}
    <p>Loading...</p>
{:else}
    <div class="d-flex flex-row h-100">
        <div class="">
            <Sidebar />
        </div>
        <div class="stretch-width overflow-auto">
            <Topbar />
            <div class="px-4 py-3">
                <Router primary={false}>
                    <Route path="home" component={Home} />
                    <Route path="minions/:minionId/*" component={Minion} />
                    <Route path="minions" component={Minions} />
                    <Route path="*">
                        <Redirect to={paths.home.path} />
                    </Route>
                </Router>
            </div>
        </div>
    </div>
{/if}

<style>
    .stretch-width {
        width: 100%;
        width: -moz-available;
        width: -webkit-fill-available;
        width: stretch;
    }
</style>
