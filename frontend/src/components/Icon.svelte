<script>
    import feather from 'feather-icons';

    export let name = "home";
    let cssClass = "";
    export { cssClass as class };
    export let size = 24;
</script>

{@html feather.icons[name].toSvg({
    class: cssClass,
    width: size,
    height: size,
})}