<script lang="ts">
    import {
        Card,
        CardHeader,
        CardTitle,
        CardSubtitle,
        CardBody,
        Row,
        Col,
    } from "sveltestrap";
    import { theme } from "../../stores";
    import ThemeColorPicker from "./ThemeColorPicker.svelte";
    import ThemeDarkToggle from "./ThemeDarkToggle.svelte";
</script>

<h1>Preferences</h1>

<Row>
    <Col xs="12">
        <Card class="mb-3 {$theme.dark ? 'bg-dark' : ''}">
            <CardHeader>
                <CardTitle class="mb-0">Theme</CardTitle>
            </CardHeader>
            <CardBody>
                <CardSubtitle class="mb-3">Color:</CardSubtitle>
                <ThemeColorPicker />
                <hr />
                <CardSubtitle class="mb-3">Dark mode:</CardSubtitle>
                <ThemeDarkToggle />
            </CardBody>
        </Card>
    </Col>
</Row>
