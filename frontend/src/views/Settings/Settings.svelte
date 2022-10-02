<script lang="ts">
    import { useNavigate } from 'svelte-navigator';
    const navigate = useNavigate();

    import { currentUser, theme } from '../../stores';
    import { hasResaltPermission, P_ADMIN_GROUP } from '../../perms';
    import paths from '../../paths';
    import SettingsTabConfig from './SettingsTabConfig.svelte';
    import SettingsTabGroups from './SettingsTabGroups.svelte';
    import Tabs from '../../components/Tabs.svelte';
    import type { NavSubPage } from '../../utils';
    import type User from '../../models/User';

    function calcSubPagesNav(currentUser: User | null): NavSubPage[] {
        if (!currentUser) return [];

        let navs: NavSubPage[] = [
            {
                label: 'Config',
                component: SettingsTabConfig,
                class: $theme.dark ? 'bg-dark' : '',
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
        subPagesNav.findIndex(
            (page) =>
                page.label.toLowerCase() === location.pathname.split('/')[4],
        ),
        0,
    );
</script>

<h1>Settings</h1>

<Tabs
    children={subPagesNav}
    selected={currentSubPage}
    onSelect={(index) => {
        let pageLabel = subPagesNav[index].label.toLowerCase();
        if (pageLabel === 'config') {
            navigate(paths.settings.path);
        } else {
            navigate(paths.settings_page.getPath(pageLabel));
        }
    }}
/>
