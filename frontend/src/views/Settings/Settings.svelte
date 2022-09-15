<script lang="ts">
    import { useNavigate } from "svelte-navigator";
    import { Label } from "sveltestrap";
    const navigate = useNavigate();

    import Tabs from "../../components/Tabs.svelte";
    import paths from "../../paths";
    import SettingsTabConfig from "./SettingsTabConfig.svelte";
    import SettingsTabGroups from "./SettingsTabGroups.svelte";
    import { theme } from "../../stores";

    $: subPagesNav = [
        {
            label: "Config",
            component: SettingsTabConfig,
            class: $theme.dark ? "bg-dark" : "",
        },
        {
            label: "Groups",
            component: SettingsTabGroups,
        },
    ];

    $: subPage = location.pathname.split("/")[4];
    // $: console.log("location", location, subPage);

    // Find index of subPage in subPagesNav, or 0 otherwise.
    $: currentSubPage = Math.max(
        subPagesNav.findIndex((page) => page.label.toLowerCase() === subPage),
        0
    );
</script>

<h1>Settings</h1>

<Tabs
    children={subPagesNav}
    selected={currentSubPage}
    onSelect={(index) => {
        let pageLabel = subPagesNav[index].label.toLowerCase();
        if (pageLabel === "config") {
            navigate(paths.settings.path);
        } else {
            navigate(paths.settings_page.getPath(pageLabel));
        }
    }}
/>
