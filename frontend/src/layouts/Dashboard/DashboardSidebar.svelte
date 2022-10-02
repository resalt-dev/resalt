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
                class="mb-0 h3 {$theme.dark
                    ? 'text-light'
                    : 'text-white'} {!$collapsed && 'd-none'}"
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
        class="{$theme.dark
            ? 'text-light'
            : 'text-white'} btn-dark bg-dark border-0 pt-3 pb-3 px-3 fw-light mouse-pointer d-flex align-items-center"
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
    <span
        class="text-center {$config.latestVersion === 'unknown'
            ? 'link-danger text-decoration-underline mouse-pointer'
            : $config.currentVersion !== $config.latestVersion
            ? 'link-warning text-decoration-underline mouse-pointer'
            : 'text-secondary'}"
        on:click={$config.currentVersion !== $config.latestVersion
            ? toggleUpdate
            : null}
    >
        {#if $collapsed}
            {$config.currentVersion}
        {:else}
            {constants.appName} - {$config.currentVersion}
        {/if}
    </span>
</div>

<div>
    <Modal
        isOpen={openUpdate}
        toggle={toggleUpdate}
        contentClassName={$theme.dark ? 'bg-secondary text-white' : ''}
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
                Current version: <code>"{$config.currentVersion}"</code>
                <br />
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
                Current version: <code>"{$config.currentVersion}"</code>
                <br />
                Latest version: <code>"{$config.latestVersion}"</code>
                <br />
                <br />
                Upgrading to the latest version is
                <b>CRITICAL FOR SECURITY</b>.
                <br />
                <br />
                By not updating, you risk compromising the security and integrity
                of your infrastructure by not taking use of the latest bug fixes
                and security patches.
                <br />
                <hr class="bg-light" />
                You can upgrade by increasing the version number of the Docker image
                in your compose/stack file to the latest version. If you have any
                questions, please reach out on GitHub (<a
                    target="_blank"
                    href={constants.githubUrl}
                >
                    {constants.githubUrl}</a
                >).
            {/if}
        </ModalBody>
        <ModalFooter>
            <Button color="secondary" on:click={toggleUpdate}>Close</Button>
        </ModalFooter>
    </Modal>
</div>
