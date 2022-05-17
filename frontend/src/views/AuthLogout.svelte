<script>
    import { onMount } from "svelte";
    import { Link } from "svelte-navigator";
    import paths from "../paths";
    import { logout } from "../controller";
    import constants from "../constants";

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
        class={`btn btn-${constants.mainColor} float-end`}>Go to login</Link
    >
{:catch error}
    An error occurred.
{/await}
