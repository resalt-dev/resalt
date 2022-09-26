<script lang="ts">
    import { onMount } from 'svelte';
    import { Router, Route, useNavigate } from 'svelte-navigator';
    import { currentUser, theme } from '../../stores';
    import { loadCurrentUser, logout } from '../../controller';
    import paths from '../../paths';
    import Sidebar from './DashboardSidebar.svelte';
    import DashboardTopbar from './DashboardTopbar.svelte';
    import Redirect from '../../components/Redirect.svelte';

    import Home from '../../views/Home/Home.svelte';
    import Minion from '../../views/Minion/Minion.svelte';
    import Minions from '../../views/Minions/Minions.svelte';
    import Run from '../../views/Run/Run.svelte';
    import Job from '../../views/Job/Job.svelte';
    import Jobs from '../../views/Jobs/Jobs.svelte';
    import Keys from '../../views/Keys/Keys.svelte';
    import Events from '../../views/Events.svelte';
    import User from '../../views/User/User.svelte';
    import Users from '../../views/Users/Users.svelte';
    import Settings from '../../views/Settings/Settings.svelte';
    import Preferences from '../../views/Preferences/Preferences.svelte';

    const navigate = useNavigate();

    onMount(() => {
        loadCurrentUser()
            .then((data) => {})
            .catch((err) => {
                logout();
                navigate(paths.logout.path);
            });
    });
</script>

{#if $currentUser === null}
    <p>Loading...</p>
{:else}
    <div class="d-flex flex-row h-100">
        <div class="">
            <Sidebar />
        </div>
        <div
            class="w-100 overflow-auto {$theme.dark
                ? 'bg-darker text-light'
                : ''}"
        >
            <DashboardTopbar />
            <div class="px-4 py-3">
                <Router primary={false}>
                    <Route path="home" component={Home} />
                    <Route path="minions/:minionId/*" component={Minion} />
                    <Route path="minions" component={Minions} />
                    <Route path="run" component={Run} />
                    <Route path="jobs/:jobId" component={Job} />
                    <Route path="jobs" component={Jobs} />
                    <Route path="keys" component={Keys} />
                    <Route path="events" component={Events} />
                    <Route path="users/:userId" component={User} />
                    <Route path="users" component={Users} />
                    <Route path="preferences" component={Preferences} />
                    <Route path="settings/*" component={Settings} />
                    <Route path="*">
                        <Redirect to={paths.home.path} />
                    </Route>
                </Router>
            </div>
        </div>
    </div>
{/if}
