<script lang="ts">
    import { MessageType } from '../../models/MessageType';
    import { currentUser } from '../../stores';
    import { hasResaltPermission, P_ADMIN_GROUP } from '../../perms';
    import { onMount } from 'svelte';
    import { showToast, getMinionById } from '../../controller';
    import { writable, type Writable } from 'svelte/store';
    import MinionConformity from './MinionConformity.svelte';
    import MinionGrains from './MinionGrains.svelte';
    import MinionInfo from './MinionInfo.svelte';
    import MinionPackages from './MinionPackages.svelte';
    import MinionPillars from './MinionPillars.svelte';
    import paths from '../../paths';
    import Tabs from '../../components/Tabs.svelte';
    import type { NavigateFn } from 'svelte-navigator';
    import type TabPage from '../../models/TabPage';
    import type Minion from '../../models/Minion';

    // svelte-ignore unused-export-let
    export let location: Location;
    // svelte-ignore unused-export-let
    export let navigate: NavigateFn;
    export let minionId: string;
    export let subPage: string = '';

    const minion: Writable<Minion | null> = writable(null);

    onMount(() => {
        getMinionById(minionId)
            .then((data) => {
                minion.set(data);
            })
            .catch((err) => {
                showToast(
                    MessageType.ERROR,
                    'Failed fetching minion: ' + minionId,
                    err,
                );
            });
    });

    let tabs: TabPage[] = [];
    $: tabs = [
        {
            key: 'info',
            label: 'General',
            path: paths.minion.getPath(minionId),
            component: MinionInfo,
            data: { minion },
        },
        {
            key: 'conformity',
            label: 'Conformity',
            path: paths.minion.getPath(minionId, 'conformity'),
            component: MinionConformity,
            data: { minion },
        },
        {
            key: 'grains',
            label: 'Grains',
            path: paths.minion.getPath(minionId, 'grains'),
            component: MinionGrains,
            data: { minion },
        },
        {
            key: 'pillars',
            label: 'Pillars',
            path: paths.minion.getPath(minionId, 'pillars'),
            component: MinionPillars,
            data: { minion },
            // TODO: add perm for pillars
            hidden: !hasResaltPermission($currentUser.perms, P_ADMIN_GROUP),
        },
        {
            key: 'packages',
            label: 'Packages',
            path: paths.minion.getPath(minionId, 'packages'),
            component: MinionPackages,
            data: { minion },
        },
    ];
</script>

{#if !$minion}
    <h1>Loading...</h1>
{:else}
    <h1>Minion {$minion.id}</h1>

    <Tabs {tabs} current={subPage} />
{/if}
