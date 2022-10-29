<script lang="ts">
    import paths from '../../paths';
    import Tabs from '../../components/Tabs.svelte';
    import type { NavigateFn } from 'svelte-navigator';
    import type TabPage from '../../models/TabPage';

    import PreferencesTabTheme from './PreferencesTabTheme.svelte';

    // svelte-ignore unused-export-let
    export let location: Location;
    // svelte-ignore unused-export-let
    export let navigate: NavigateFn;
    export let subPage: string = '';

    let tabs: TabPage[] = [];
    $: tabs = [
        {
            key: 'theme',
            label: 'Theme',
            path: paths.preferences.getPath('theme'),
            component: PreferencesTabTheme,
        },
    ];
</script>

<h1>Preferences</h1>

<Tabs {tabs} current={subPage} />
