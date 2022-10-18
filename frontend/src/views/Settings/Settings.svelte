<script lang="ts">
    import { useNavigate } from 'svelte-navigator';
    const navigate = useNavigate();

    import { currentUser } from '../../stores';
    import { hasResaltPermission, P_ADMIN_GROUP } from '../../perms';
    import paths from '../../paths';
    import SettingsTabConfig from './SettingsTabConfig.svelte';
    import SettingsTabGroups from './SettingsTabGroups.svelte';
    import Tabs from '../../components/Tabs.svelte';
    import type { NavSubPage } from '../../utils';
    import type User from '../../models/User';

    export let subPage: string = 'theme';

    function calcSubPagesNav(currentUser: User | null): NavSubPage[] {
        if (!currentUser) return [];

        let navs: NavSubPage[] = [
            {
                label: 'Config',
                component: SettingsTabConfig,
            },
        ];

        if (hasResaltPermission(currentUser.perms, P_ADMIN_GROUP)) {
            navs.push({
                label: 'Groups',
                component: SettingsTabGroups,
            });
        }

        return navs;
    }

    $: subPagesNav = calcSubPagesNav($currentUser);

    // Find index of subPage in subPagesNav, or 0 otherwise.
    $: currentSubPage = Math.max(
        subPagesNav.findIndex((page) => page.label.toLowerCase() === subPage),
        0,
    );
</script>

<h1>Settings</h1>

<Tabs
    children={subPagesNav}
    selected={currentSubPage}
    onSelect={(index) => {
        let pageLabel = subPagesNav[index].label.toLowerCase();
        navigate(paths.settings_page.getPath(pageLabel));
    }}
/>
