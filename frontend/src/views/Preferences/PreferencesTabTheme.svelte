<script lang="ts">
    import { Row, Col, FormGroup } from 'sveltestrap';
    import Icon from '../../components/Icon.svelte';
    import constants from '../../constants';
    import { config, theme } from '../../stores';

    function selectColor(event: any): void {
        let color = [...event.target.classList]
            .filter((i) => i.startsWith('text'))[0]
            .split('-')[1];
        console.log('selectColor', color);
        if (color === 'reset') {
            $theme.color = $config.defaultThemeColor;
        } else {
            $theme.color = color;
        }
    }

    function toggleDarkMode(_event: any): void {
        console.log('toggleDarkMode');
        theme.update((t) => ({ ...t, dark: !t.dark }));
    }
</script>

<Row>
    <Col xs="12">
        {#if $config.enableThemeSwitching}
            <h5 class="mb-3">Color:</h5>

            {#each constants.themeColors as color}
                <Icon
                    type={color === $theme.color ? 'solid' : 'regular'}
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

            <hr />
        {/if}

        <h5 class="mb-3">Dark mode:</h5>
        <FormGroup floating={true} class="form-switch ps-2">
            <input
                type="checkbox"
                class="form-check-input fs-3 ms-0 mt-0"
                checked={$theme.dark}
                on:click={toggleDarkMode}
            />
        </FormGroup>
    </Col>
</Row>
