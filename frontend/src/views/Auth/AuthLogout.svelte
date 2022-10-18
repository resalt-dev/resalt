<script lang="ts">
    import { onMount } from 'svelte';
    import { Link } from 'svelte-navigator';
    import paths from '../../paths';
    import { theme } from '../../stores';
    import { logout, showToast } from '../../controller';
    import { MessageType } from '../../models/MessageType';

    let loggingOut: boolean = true;

    onMount(() => {
        logout()
            .then(() => {
                loggingOut = false;
                showToast(
                    MessageType.SUCCESS,
                    'Logout Success',
                    'You have now been logged out.',
                );
            })
            .catch((err) => {
                showToast(MessageType.ERROR, 'Logout Error', err);
            });
    });
</script>

{#if loggingOut}
    <p class="fw-bold">Hang on a moment while we sign you out.</p>
{:else}
    <p class="fw-bold">You have been successfully signed out.</p>
    <br />
    <Link
        to={paths.login.path}
        class={`btn btn-${$theme.color} float-start px-5 fw-bold mb-3`}
    >
        Go back
    </Link>
    <br />
{/if}
