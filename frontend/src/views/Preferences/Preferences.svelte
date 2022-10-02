<script lang="ts">
    import { useNavigate } from 'svelte-navigator';
    const navigate = useNavigate();

    import paths from '../../paths';
    import PreferencesTabTheme from './PreferencesTabTheme.svelte';
    import Tabs from '../../components/Tabs.svelte';
    import type { NavSubPage } from '../../utils';

    function calcSubPagesNav(): NavSubPage[] {
        let navs: NavSubPage[] = [];

        navs.push({
            label: 'Color theme',
            component: PreferencesTabTheme,
        });

        return navs;
    }

    $: subPagesNav = calcSubPagesNav();

    // Find index of subPage in subPagesNav, or 0 otherwise.
    $: currentSubPage = Math.max(
        subPagesNav.findIndex(
            (page) =>
                page.label.toLowerCase() === location.pathname.split('/')[4],
        ),
        0,
    );
</script>

<h1>Preferences</h1>

<Tabs
    children={subPagesNav}
    selected={currentSubPage}
    onSelect={(index) => {
        let pageLabel = subPagesNav[index].label.toLowerCase();
        if (pageLabel === 'config') {
            navigate(paths.preferences.path);
        } else {
            navigate(paths.preferences_page.getPath(pageLabel));
        }
    }}
/>
