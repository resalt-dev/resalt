<script lang="ts">
    import paths from '../../paths';
    import PreferencesTabTheme from './PreferencesTabTheme.svelte';
    import Tabs from '../../components/Tabs.svelte';
    import type { NavSubPage } from '../../utils';
    import type { NavigateFn } from 'svelte-navigator';

    // svelte-ignore unused-export-let
    export let location: Location;
    export let navigate: NavigateFn;
    export let subPage: string = 'theme';

    function calcSubPagesNav(): NavSubPage[] {
        let navs: NavSubPage[] = [];

        navs.push({
            label: 'Theme',
            component: PreferencesTabTheme,
        });

        return navs;
    }

    $: subPagesNav = calcSubPagesNav();

    // Find index of subPage in subPagesNav, or 0 otherwise.
    $: currentSubPage = Math.max(
        subPagesNav.findIndex((page) => page.label.toLowerCase() === subPage),
        0,
    );
</script>

<h1>Preferences</h1>

<Tabs
    children={subPagesNav}
    selected={currentSubPage}
    onSelect={(index) => {
        let pageLabel = subPagesNav[index].label.toLowerCase();
        navigate(paths.preferences_page.getPath(pageLabel));
    }}
/>
