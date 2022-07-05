<script>
    import { onMount } from "svelte";
    import { Link } from "svelte-navigator";
    import paths from "../paths";
    import { theme } from "../stores";
    import { logout } from "../controller";

    onMount(async () => {
        logout();
    });
</script>

{#await logout()}
    <p class="fw-bold">Hang on a moment while we sign you out.</p>
{:then}
    <p class="fw-bold">You have been successfully signed out.</p>
    <br />
    <Link
        to={paths.login.path}
        class={`btn btn-${$theme.color} float-start px-5 fw-bold`}>Go back</Link
    >
{:catch error}
    An error occurred.
{/await}
