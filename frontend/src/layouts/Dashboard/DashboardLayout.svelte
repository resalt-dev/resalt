<script lang="ts">
    import { onMount } from 'svelte';
    import { Router, Route, useNavigate } from 'svelte-navigator';
    import { currentUser, toasts } from '../../stores';
    import { loadCurrentUser } from '../../controller';
    import paths from '../../paths';
    import Sidebar from './DashboardSidebar.svelte';
    import DashboardHeader from './DashboardHeader.svelte';
    import Redirect from '../../components/Redirect.svelte';
    import SSEConnector from '../../components/SSEConnector.svelte';

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

    const navigate = useNavigate();

    onMount(() => {
        loadCurrentUser()
            .then((data) => {
                currentUser.set(data);
            })
            .catch((err) => {
                console.error(err);
                navigate(paths.logout.path);
            });
    });
</script>

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
                    <Route
                        path="preferences/:subPage"
                        component={Preferences}
                    />
                    <Route path="preferences" component={Preferences} />
                    <Route path="settings/:subPage" component={Settings} />
                    <Route path="settings" component={Settings} />
                    <Route path="*">
                        <Redirect to={paths.home.path} />
                    </Route>
                </Router>
            </div>
        </div>
    </div>
{/if}

<!-- Toast / Alerts -->
<div class="position-fixed top-0 end-0 mt-5 me-5" style="z-index: 11">
    {#each $toasts as toast}
        <Toast class="{'toast-' + toast.type} mb-2">
            <ToastHeader>{toast.title}</ToastHeader>
            <ToastBody>{toast.message}</ToastBody>
        </Toast>
    {/each}
</div>
