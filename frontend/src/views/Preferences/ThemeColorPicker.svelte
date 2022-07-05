<script>
    import Icon from "../../components/Icon.svelte";
    import constants from "../../constants";
    import { theme } from "../../stores";

    $: currentColor = $theme.color;
    $: console.log("currentColor", currentColor);

    function selectColor(event) {
        let color = [...event.target.classList]
            .filter((i) => i.startsWith("text"))[0]
            .split("-")[1];
        console.log("selectColor", color);
        theme.update((t) => ({ ...t, color }));
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
{/each}
