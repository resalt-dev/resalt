<script lang="ts">
    import { closeEvents, connectEvents } from '../controller';
    import { auth } from '../stores';
    import { get } from 'svelte/store';

    $: {
        if ($auth) {
            console.log('Connecting to SSE...');
            connectEvents();
        } else {
            closeEvents();
        }
    }
</script>
