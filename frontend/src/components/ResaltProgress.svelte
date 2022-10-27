<script lang="ts">
    import { Progress } from 'sveltestrap';
    import { theme } from '../stores';
</script>

<Progress animated color={null} barClassName={'bg-' + $theme.color} value={100}>
    Loading...
</Progress>
