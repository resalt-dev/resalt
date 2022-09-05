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

<Row>
    <Col xs="12">
        <h5 class="mb-3">Color:</h5>
        <ThemeColorPicker />
        <hr />
        <h5 class="mb-3">Dark mode:</h5>
        <ThemeDarkToggle />
    </Col>
</Row>
