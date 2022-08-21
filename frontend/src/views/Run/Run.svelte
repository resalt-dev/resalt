<script lang="ts">
    import Tabs from "../../components/Tabs.svelte";

    import RunTabLive from "./RunTabLive.svelte";
</script>

<h1>Run</h1>

<Tabs
    children={[
        {
            label: "Live Run",
            component: RunTabLive,
        },
    ]}
/>

Under development.
