<script lang="ts">
    import { onMount } from 'svelte';
    import { useNavigate } from 'svelte-navigator';
    import { currentUser } from '../../stores';
    import { loadCurrentUser } from '../../controller';
    import paths from '../../paths';

    const navigate = useNavigate();

    onMount(() => {
        loadCurrentUser()
            .then((data) => {
                currentUser.set(data);
            })
            .catch((err) => {
                console.error(err);
                navigate(paths.logout.path);
            });
    });
</script>

<div />
