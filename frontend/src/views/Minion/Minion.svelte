<script lang="ts">
    import { AlertType } from '../../models/MessageType';
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
    import type { NavSubPage } from '../../utils';
    import type Minion from '../../models/Minion';
    import type User from '../../models/User';

    export let navigate: NavigateFn;
    export let location: { pathname: string };
    export let minionId: string;

    const minion: Writable<Minion | null> = writable(null);

    onMount(() => {
        getMinionById(minionId)
            .then((data) => {
                minion.set(data);
            })
            .catch((err) => {
                showToast(
                    AlertType.ERROR,
                    'Failed fetching minion: ' + minionId,
                    err,
                );
            });
    });

    function calcSubPagesNav(
        currentUser: User | null,
        minion: Writable<Minion | null>,
    ): NavSubPage[] {
        if (!currentUser) return [];

        let navs: NavSubPage[] = [
            {
                label: 'General',
                component: MinionInfo,
                data: minion,
            },
            {
                label: 'Conformity',
                component: MinionConformity,
                data: minion,
            },
            {
                label: 'Grains',
                component: MinionGrains,
                data: minion,
            },
        ];

        if (hasResaltPermission(currentUser.perms, P_ADMIN_GROUP)) {
            // TODO: add perm for pillars
            navs.push({
                label: 'Pillars',
                component: MinionPillars,
                data: minion,
            });
        }

        navs.push({
            label: 'Packages',
            component: MinionPackages,
            data: minion,
        });

        return navs;
    }

    $: subPagesNav = calcSubPagesNav($currentUser, minion);

    // Find index of subPage in subPagesNav, or 0 otherwise.
    $: currentSubPage = Math.max(
        subPagesNav.findIndex(
            (page) =>
                page.label.toLowerCase() === location.pathname.split('/')[4],
        ),
        0,
    );
</script>

{#if !$minion}
    <h1>Loading...</h1>
{:else}
    <h1>Minion {$minion.id}</h1>

    <Tabs
        children={subPagesNav}
        selected={currentSubPage}
        onSelect={(index) => {
            let pageLabel = subPagesNav[index].label.toLowerCase();
            if (pageLabel === 'general') {
                navigate(paths.minion.getPath(minionId));
            } else {
                navigate(paths.minion.getPath(minionId, pageLabel));
            }
        }}
    />
{/if}
