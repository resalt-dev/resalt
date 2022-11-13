<script lang="ts">
    import { onMount } from 'svelte';
    import { useNavigate } from 'svelte-navigator';
    import { currentUser } from '../../stores';
    import { getCurrentUser } from '../../api';
    import paths from '../../paths';

    const navigate = useNavigate();

    onMount(() => {
        getCurrentUser()
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
