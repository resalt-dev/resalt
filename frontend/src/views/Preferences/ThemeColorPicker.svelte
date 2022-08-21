<script lang="ts">
    import Icon from "../../components/Icon.svelte";
    import constants from "../../constants";
    import { config, theme } from "../../stores";

    $: currentColor = $theme.color;
    $: console.log("currentColor", currentColor);

    function selectColor(event) {
        let color = [...event.target.classList]
            .filter((i) => i.startsWith("text"))[0]
            .split("-")[1];
        console.log("selectColor", color);
        if (color === "reset") {
            $theme.color = $config.themeColor;
        } else {
            $theme.color = color;
        }
    }
</script>

{#each constants.themeColors as color}
    <Icon
        type={color === currentColor ? "solid" : "regular"}
        size="2"
        name="check-circle"
        class="mouse-pointer text-{color}"
        on:click={selectColor}
    />
{/each}<Icon
    type="regular"
    size="2"
    name="reset"
    class="mouse-pointer text-reset"
    on:click={selectColor}
/>
