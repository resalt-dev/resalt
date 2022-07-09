<script lang="ts">
    import { onMount } from "svelte";
    import { Router, Route, useNavigate } from "svelte-navigator";
    import { user, theme } from "../stores";
    import { loadUser, logout } from "../controller";
    import paths from "../paths";
    import Sidebar from "../components/Sidebar/Sidebar.svelte";
    import Topbar from "../components/Topbar.svelte";
    import Redirect from "../components/Redirect.svelte";

    import Home from "../views/Home.svelte";
    import Minion from "../views/Minion/Minion.svelte";
    import Minions from "../views/Minions/Minions.svelte";
    import Run from "../views/Run.svelte";
    import Job from "../views/Job.svelte";
    import Jobs from "../views/Jobs/Jobs.svelte";
    import Schedules from "../views/Schedules.svelte";
    import Keys from "../views/Keys.svelte";
    import Events from "../views/Events.svelte";
    import Users from "../views/Users.svelte";
    import Settings from "../views/Settings.svelte";
    import Preferences from "../views/Preferences/Preferences.svelte";

    const navigate = useNavigate();

    onMount(() => {
        loadUser().catch(() => {
            logout();
            navigate(paths.logout.path);
        });
    });
</script>

{#if $user == null}
    <p>Loading...</p>
{:else}
    <div class="d-flex flex-row h-100">
        <div class="">
            <Sidebar />
        </div>
        <div
            class="stretch-width overflow-auto {$theme.dark
                ? 'bg-darker text-light'
                : ''}"
        >
            <Topbar />
            <div class="px-4 py-3">
                <Router primary={false}>
                    <Route path="home" component={Home} />
                    <Route path="minions/:minionId/*" component={Minion} />
                    <Route path="minions" component={Minions} />
                    <Route path="run" component={Run} />
                    <Route path="jobs/:jobId/*" component={Job} />
                    <Route path="jobs" component={Jobs} />
                    <Route path="schedules" component={Schedules} />
                    <Route path="keys" component={Keys} />
                    <Route path="events" component={Events} />
                    <Route path="users" component={Users} />
                    <Route path="settings" component={Settings} />
                    <Route path="preferences" component={Preferences} />
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
