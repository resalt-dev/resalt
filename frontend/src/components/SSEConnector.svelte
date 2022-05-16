<script>
    import { onMount } from "svelte";
    import { connect_events } from "../controller";

    import { useNavigate } from "svelte-navigator";
    const navigate = useNavigate();

    onMount(() => {
        console.log("Connecting to SSE...");

        // Connect to SSE
        connect_events(navigate);
    });
</script>
