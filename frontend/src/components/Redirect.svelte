<script lang="ts">
    import { onMount } from 'svelte';
    import { useNavigate } from 'svelte-navigator';

    const navigate = useNavigate();

    export let to: string;

    onMount(() => {
        navigate(to);
    });
</script>
