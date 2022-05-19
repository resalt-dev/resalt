<script>
    import { close_events, connect_events } from "../controller";
    import { auth } from "../stores";
    import { get } from "svelte/store";

    import { useNavigate } from "svelte-navigator";
    const navigate = useNavigate();

    $: {
        console.log("hi");
        console.log(get(auth));
        if ($auth) {
            console.log("Connecting to SSE...");
            connect_events(navigate);
        } else {
            close_events();
        }
    }
</script>
