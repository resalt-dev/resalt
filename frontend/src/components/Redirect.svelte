<script>
    import { onMount } from "svelte";
    import { useNavigate } from "svelte-navigator";

    const navigate = useNavigate();

    export let to;

    onMount(() => {
        navigate(to);
    });
</script>
