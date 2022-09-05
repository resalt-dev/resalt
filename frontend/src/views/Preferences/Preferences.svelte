<script lang="ts">
    import Tabs from "../../components/Tabs.svelte";
    import PreferencesTabTheme from "./PreferencesTabTheme.svelte";
</script>

<h1>Preferences</h1>

<Tabs
    children={[
        {
            label: "Themeing",
            component: PreferencesTabTheme,
        },
    ]}
/>
