<script>
    import { theme } from "../stores";

    function toggleDarkMode(event) {
        console.log("toggleDarkMode");
        theme.update((t) => ({ ...t, dark: !t.dark }));
    }
</script>

<div class="form-check form-switch">
    <input
        checked={$theme.dark}
        style="margin-left: -1em;"
        class="form-check-input fs-3 mt-0 mb-3"
        type="checkbox"
        id="toggleDarkModeSwitch"
        on:click={toggleDarkMode}
    />
</div>
