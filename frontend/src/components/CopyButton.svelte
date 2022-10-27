<script lang="ts">
    import { Button } from 'sveltestrap';
    import Icon from '../components/Icon.svelte';
    import { showToast } from '../controller';
    import { MessageType } from '../models/MessageType';
    import { theme } from '../stores';

    export let name: string;
    export let value: string;
</script>

<Button
    color={$theme.color}
    size="sm"
    class="ms-2"
    style="margin-bottom: -0.15rem;"
    on:click={() => {
        navigator.clipboard.writeText(value);
        showToast(
            MessageType.SUCCESS,
            'Copied to Clipboard',
            `Copied ${name} "${value}" to clipboard.`,
        );
    }}
>
    <Icon name="clipboard" size="1" align="top" style="padding-top: 0.15rem;" />
</Button>
