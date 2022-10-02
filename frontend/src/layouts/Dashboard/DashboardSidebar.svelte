<script lang="ts">
    import paths from '../../paths';
    import { sidebarCollapsed as collapsed, theme, config } from '../../stores';
    import Icon from '../../components/Icon.svelte';
    import Logo from '../../components/Logo.svelte';
    import SidebarItem from './DashboardSidebarItem.svelte';
    import constants from '../../constants';
    import {
        Button,
        Modal,
        ModalBody,
        ModalFooter,
        ModalHeader,
    } from 'sveltestrap';

    function handleClickCollapse(): void {
        collapsed.update((n) => !n);
    }

    let openUpdate = false;
    const toggleUpdate = () => (openUpdate = !openUpdate);
</script>

<div
    class="d-flex flex-column flex-shrink-0 bg-dark h-100 no-select"
    style="min-height: 100vh; overflow-y: auto; {$collapsed
        ? 'width: 4.5rem;'
        : 'width: 17.5rem;'}"
>
    <div
        on:click={handleClickCollapse}
        class="d-flex text-decoration-none mouse-pointer"
    >
        <div
            class="d-flex align-items-center py-4 {$collapsed
                ? 'mx-auto'
                : 'w-100'}"
            style="height: 80px"
        >
            <div class="px-5 py-3 {$collapsed ? 'd-none' : 'w-100'}">
                <Logo color={$theme.color} />
            </div>
            <Icon
                name="menu"
                class="mb-0 h3 text-white {!$collapsed && 'd-none'}"
            />
        </div>
    </div>

    <hr class="mt-0 mb-3" />

    <ul
        class="nav nav-pills flex-column mb-auto fs-5 {$collapsed
            ? 'nav-flush text-center'
            : 'mx-2'}"
    >
        {#each paths as route}
            {#if route.showInNav}
                {#if route.name === 'users'}
                    <li><hr /></li>
                {/if}
                <SidebarItem {route} collapsed={$collapsed} />
            {/if}
        {/each}
    </ul>

    <hr class="mb-0" />

    <div
        on:click={handleClickCollapse}
        class="btn-dark bg-dark border-0 pt-3 pb-3 px-3 fw-light mouse-pointer d-flex align-items-center"
        aria-current="page"
    >
        <Icon
            name={$collapsed ? 'right-arrow-alt' : 'left-arrow-alt'}
            class={$collapsed ? '' : 'me-3'}
            size="2.5"
            style="margin-bottom: -2px;"
        />
        {#if !$collapsed}
            <span class="fs-5">Collapse</span>
        {/if}
    </div>

    <hr class="mt-0 mb-0" />

    <!-- svelte-ignore a11y-invalid-attribute -->
    {#if $config.latestVersion === 'unknown'}
        <span
            class="text-center link-danger text-decoration-underline mouse-pointer"
            on:click={toggleUpdate}
        >
            {#if $collapsed}
                {$config.currentVersion}
            {:else}
                <Icon name="chevrons-up" />
                {constants.appName} - {$config.currentVersion}
                <Icon name="chevrons-up" />
            {/if}
        </span>
    {:else if $config.currentVersion !== $config.latestVersion}
        <span
            class="text-center link-warning text-decoration-underline mouse-pointer"
            on:click={toggleUpdate}
        >
            {#if $collapsed}
                {$config.currentVersion}
            {:else}
                <Icon name="chevrons-up" />
                {constants.appName} - {$config.currentVersion}
                <Icon name="chevrons-up" />
            {/if}
        </span>
    {:else}
        <span class="text-center text-secondary">
            {#if $collapsed}
                {$config.currentVersion}
            {:else}
                {constants.appName} - {$config.currentVersion}
            {/if}
        </span>
    {/if}
</div>

<div>
    <Modal
        isOpen={openUpdate}
        toggle={toggleUpdate}
        class={$theme.dark ? 'theme-dark' : ''}
    >
        <ModalHeader
            toggle={toggleUpdate}
            class={$config.latestVersion === 'unknown'
                ? 'bg-danger'
                : 'bg-warning text-dark'}
        >
            {#if $config.latestVersion === 'unknown'}
                Update Error!
            {:else}
                Update Warning
            {/if}
        </ModalHeader>
        <ModalBody>
            {#if $config.latestVersion === 'unknown'}
                <h1>
                    <span class="update-label">Current: </span>
                    <span class="badge bg-{$theme.color}"
                        >{$config.currentVersion}</span
                    >
                </h1>
                <br />
                There was a critical error while trying to check for updates. Especially
                in a software that interracts with SaltStack, it is
                <b>CRITICAL</b> to run the latest version for security reasons.
                <br />
                <br />
                Double-check that the Resalt container is able to access
                <code>raw.githubusercontent.com</code> and without a proxy. Please
                contact your administrator or the Resalt development team if this
                issue persists.
            {:else}
                <h1>
                    <span class="update-label">Current: </span>
                    <span class="badge bg-{$theme.color}"
                        >{$config.currentVersion}</span
                    >
                </h1>
                <h1>
                    <span class="update-label">Latest: </span>
                    <span class="badge bg-{$theme.color}"
                        >{$config.latestVersion}</span
                    >
                </h1>
                <br />
                By not upgrading, you risk compromising the security and integrity
                of your infrastructure by not taking use of the latest bug fixes
                and security patches.
                <br />
                <hr class="bg-light" />
                You can upgrade by increasing the version number of the Docker image
                in your compose/stack file to the latest version. If you have any
                questions, please reach out on GitHub:<a
                    target="_blank"
                    href={constants.githubUrl}
                >
                    {constants.githubUrl}</a
                >
            {/if}
        </ModalBody>
        <ModalFooter>
            <Button color="secondary" on:click={toggleUpdate}>Close</Button>
        </ModalFooter>
    </Modal>
</div>

<style lang="scss">
    .update-label {
        width: 150px;
        display: inline-block;
    }
</style>
