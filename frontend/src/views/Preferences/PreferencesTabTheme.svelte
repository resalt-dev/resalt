<script lang="ts">
    import {
        Row,
        Col,
        FormGroup,
        Button,
        Card,
        CardHeader,
        CardBody,
    } from 'sveltestrap';
    import Icon from '../../components/Icon.svelte';
    import constants from '../../constants';
    import { showToast } from '../../controller';
    import { AlertType } from '../../models/AlertType';
    import { config, theme } from '../../stores';

    function selectColor(color: string): void {
        console.log('selectColor', color);
        if (color === 'reset') {
            $theme.color = $config.defaultThemeColor;
        } else {
            $theme.color = color;
        }
    }

    function setDarkMode(dark: boolean): void {
        console.log('toggleDarkMode');
        theme.update((t) => ({ ...t, dark: dark }));
    }
</script>

{#if $config.enableThemeSwitching}
    <Card class="mb-3">
        <CardHeader>Color</CardHeader>
        <CardBody>
            <Row>
                {#each constants.themeColors as color}
                    <Col xs="auto">
                        <div
                            class="theme-box mouse-pointer bg-{color} mb-4 border-{$theme.dark
                                ? 'secondary'
                                : 'light'}"
                            on:click={() => selectColor(color)}
                        >
                            {#if $theme.color === color}
                                <Icon
                                    name="check"
                                    style="color: {color === 'yellow'
                                        ? 'black'
                                        : 'white'} !important;"
                                    size="3"
                                />
                            {/if}
                        </div>
                    </Col>
                {/each}
            </Row>

            <Button
                color={null}
                class="btn-{$theme.color}"
                on:click={() => selectColor('reset')}
            >
                Reset
            </Button>
        </CardBody>
    </Card>
{/if}

<Card class="mb-3">
    <CardHeader>Dark mode</CardHeader>
    <CardBody>
        <Row>
            <Col xs="auto">
                <div
                    class="theme-box mouse-pointer mb-4 border-{$theme.dark
                        ? 'secondary'
                        : 'light'}"
                    style="background-color: #fff;"
                    on:click={() => setDarkMode(false)}
                >
                    {#if $theme.dark === false}
                        <Icon name="check" class="text-black" size="3" />
                    {/if}
                </div>
            </Col>
            <Col xs="auto">
                <div
                    class="theme-box mouse-pointer bg-dark mb-4 border-{$theme.dark
                        ? 'secondary'
                        : 'light'}"
                    on:click={() => setDarkMode(true)}
                >
                    {#if $theme.dark === true}
                        <Icon name="check" class="text-white" size="3" />
                    {/if}
                </div>
            </Col>
        </Row>

        <Button
            color={null}
            class="btn-{$theme.color}"
            on:click={() => setDarkMode(false)}
        >
            Reset
        </Button>

        <Button
            color="warning"
            on:click={() => {
                showToast(
                    AlertType.INFO,
                    'Testing toast!',
                    "This is a test toast message. It's a bit longer than the others, but that's okay.",
                );
            }}
        >
            Show toast
        </Button>
    </CardBody>
</Card>

<style lang="scss">
    $theme-box-size: 75px;
    .theme-box {
        width: $theme-box-size;
        height: $theme-box-size;
        border-radius: 10px;
        border: 6px solid;

        display: flex;
        justify-content: center; /* align horizontal */
        align-items: center; /* align vertical */
    }
</style>
