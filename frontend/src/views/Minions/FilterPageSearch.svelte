<script lang="ts">
    import { Row, Col, Button } from "sveltestrap";

    export let navigate;
    export let load_minions;
</script>

<Row>
    <Col class="mb-4">
        <label for="minionsSearch" class="form-label d-inline">ABC</label>
        <input
            id="minionsSearch"
            type="email"
            class="form-control ms-2 d-inline"
            style="width: 15rem;"
        />
    </Col>
</Row>

<!-- TEMP -->
<Button color="secondary" size="sm" on:click={() => load_minions(true)}>
    Force reload minions
</Button>
