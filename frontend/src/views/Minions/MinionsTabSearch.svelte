<script lang="ts">
    import { Button, Col, Input, Row } from "sveltestrap";
    import { refreshMinions } from "../../controller";
</script>

<Row>
    <Col class="mb-4">
        <label for="minionsSearch" class="form-label d-inline">ABC</label>
        <Input
            id="minionsSearch"
            type="text"
            placeholder="Search minions"
            class="form-control ms-2 d-inline"
            style="width: 15rem;"
        />
        <input
            id="minionsSearch"
            type="email"
            class="form-control ms-2 d-inline"
            style="width: 15rem;"
        />
    </Col>
</Row>

<Button color="secondary" size="sm" on:click={() => refreshMinions()}>
    Force reload minions
</Button>
