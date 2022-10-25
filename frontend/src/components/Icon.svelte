<script lang="ts">
    export let type = 'regular';
    export let name = 'bx-rocket';
    export let size = '2';
    let cssClass = '';
    export { cssClass as class };
    export let htmlElement = null;
    export let style = '';

    $: full_name =
        (type === 'solid' ? 'bxs-' : type === 'logo' ? 'bxl-' : 'bx-') + name;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<i
    bind:this={htmlElement}
    class="bx {full_name} {cssClass.indexOf('align-') === -1
        ? 'align-middle'
        : ''} {cssClass}"
    style="font-size: {size}rem;{style}"
/>
