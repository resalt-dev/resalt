<script lang="ts">
    import Tabs from "../../components/Tabs.svelte";
    import SettingsTabConfig from "./SettingsTabConfig.svelte";
</script>

<h1>Settings</h1>

<Tabs
    children={[
        {
            label: "Config",
            component: SettingsTabConfig,
        },
    ]}
/>
