<script>
    export let type = "regular";
    export let name = "bx-rocket";
    export let size = "2";
    let cssClass = "";
    export { cssClass as class };
    export let style = "";

    $: full_name =
        (type === "solid" ? "bxs-" : type === "logo" ? "bxl-" : "bx-") + name;
</script>

<i
    on:click
    class="bx {full_name} {cssClass}"
    style="font-size: {size}rem;{style}"
/>
