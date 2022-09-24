<script lang="ts">
    import { theme } from '../../stores';

    function toggleDarkMode(): void {
        console.log('toggleDarkMode');
        theme.update((t) => ({ ...t, dark: !t.dark }));
    }
</script>

<div class="form-check form-switch">
    <input
        id="toggleDarkModeSwitch"
        type="checkbox"
        style="margin-left: -1em;"
        class="form-check-input fs-3 mt-0 mb-3 mouse-pointer"
        checked={$theme.dark}
        on:click={toggleDarkMode}
    />
</div>
