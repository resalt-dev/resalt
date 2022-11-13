<script lang="ts">
    import { onMount } from 'svelte';
    import { Link, type NavigateFn } from 'svelte-navigator';
    import paths from '../../paths';
    import { theme, toasts } from '../../stores';
    import { logout } from '../../api';
    import { MessageType } from '../../models/MessageType';

    // svelte-ignore unused-export-let
    export let location: Location;
    // svelte-ignore unused-export-let
    export let navigate: NavigateFn;

    let loggingOut: boolean = true;

    onMount(() => {
        logout()
            .then(() => {
                loggingOut = false;
                toasts.add(
                    MessageType.SUCCESS,
                    'Logout Success',
                    'You have now been logged out.',
                );
            })
            .catch((err) => {
                toasts.add(MessageType.ERROR, 'Logout Error', err);
            });
    });
</script>

{#if loggingOut}
    <p class="fw-bold">Hang on a moment while we sign you out.</p>
{:else}
    <p class="fw-bold">You have been successfully signed out.</p>
    <br />
    <Link
        to={paths.login.getPath()}
        class={`btn btn-${$theme.color} float-start px-5 fw-bold mb-3`}
    >
        Go back
    </Link>
    <br />
{/if}
