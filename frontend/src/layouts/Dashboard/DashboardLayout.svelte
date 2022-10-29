<script lang="ts">
    import { Router, Route, type NavigatorHistory } from 'svelte-navigator';
    import { currentUser, toasts } from '../../stores';
    import paths from '../../paths';
    import Sidebar from './DashboardSidebar.svelte';
    import DashboardHeader from './DashboardHeader.svelte';
    import Redirect from '../../components/Redirect.svelte';
    import SSEConnector from './SSEConnector.svelte';
    import UserLoadConnector from './UserLoadConnector.svelte';

    import Home from '../../views/Home/Home.svelte';
    import Minion from '../../views/Minion/Minion.svelte';
    import Minions from '../../views/Minions/Minions.svelte';
    import Run from '../../views/Run/Run.svelte';
    import Job from '../../views/Job/Job.svelte';
    import Jobs from '../../views/Jobs/Jobs.svelte';
    import Keys from '../../views/Keys/Keys.svelte';
    import Events from '../../views/Events/Events.svelte';
    import User from '../../views/User/User.svelte';
    import Users from '../../views/Users/Users.svelte';
    import Settings from '../../views/Settings/Settings.svelte';
    import Preferences from '../../views/Preferences/Preferences.svelte';
    import { Toast, ToastBody, ToastHeader } from 'sveltestrap';

    export let history: NavigatorHistory;
</script>

<Router primary={false} {history}>
    <UserLoadConnector />
    {#if $currentUser === null}
        <p>Loading...</p>
    {:else}
        <SSEConnector />
        <div class="d-flex flex-row h-100">
            <div class="">
                <Sidebar />
            </div>
            <div class="w-100 overflow-auto bg-white">
                <DashboardHeader />
                <div class="px-4 py-3">
                    <Route path="dashboard" component={Home} />
                    <Route
                        path="minion/:minionId/*subPage"
                        component={Minion}
                    />
                    <Route path="minions/*subPage" component={Minions} />
                    <Route path="run/*subPage" component={Run} />
                    <Route path="job/:jobId" component={Job} />
                    <Route path="jobs" component={Jobs} />
                    <Route path="keys" component={Keys} />
                    <Route path="events" component={Events} />
                    <Route path="user/:userId" component={User} />
                    <Route path="users" component={Users} />
                    <Route
                        path="preferences/*subPage"
                        component={Preferences}
                    />
                    <Route path="settings/*subPage" component={Settings} />
                    <Route path="*">
                        <Redirect to={paths.dashboard.path} />
                    </Route>
                </div>
            </div>
        </div>
    {/if}
</Router>

<!-- Toast / Alerts -->
<div class="position-fixed top-0 end-0 mt-5 me-5" style="z-index: 11">
    {#each $toasts as toast}
        <Toast class="{'toast-' + toast.type} mb-2">
            <ToastHeader>{toast.title}</ToastHeader>
            <ToastBody>{toast.message}</ToastBody>
        </Toast>
    {/each}
</div>
