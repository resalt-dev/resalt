<script lang="ts">
    import { showAlert, AlertType, getUserById } from "../../controller";
    import { theme } from "../../stores";
    import { writable } from "svelte/store";

    import { onMount } from "svelte";
    import {
        Card,
        CardBody,
        CardHeader,
        CardSubtitle,
        CardTitle,
        Col,
        Row,
    } from "sveltestrap";

    // export let navigate;
    // export let location;
    export let userId;

    const user = writable(null);

    onMount(() => {
        getUserById(userId)
            .then((data) => {
                user.set(data);
            })
            .catch((err) => {
                showAlert(
                    AlertType.ERROR,
                    "Failed fetching user: " + userId,
                    err
                );
            });
    });
</script>

{#if !$user}
    <h1>Loading...</h1>
{:else}
    <h1>User {$user.id}</h1>

    <Row>
        <Col>
            <Card class="mb-3 {$theme.dark ? 'bg-dark' : ''}">
                <CardHeader>
                    <CardTitle class="mb-0">Theme</CardTitle>
                </CardHeader>
                <CardBody>
                    <CardSubtitle class="mb-3">Color:</CardSubtitle>
                    AAA
                    <hr />
                    <CardSubtitle class="mb-3">Dark mode:</CardSubtitle>
                    BBB
                </CardBody>
            </Card>
        </Col>
    </Row>
{/if}
