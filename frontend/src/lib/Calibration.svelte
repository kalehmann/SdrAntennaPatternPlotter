<script lang="ts">
    import Dbfs from "$lib/Dbfs.svelte";
    interface Props {
        next: () => void;
    }

    let { next = () => {} }: Props = $props();
</script>

<Dbfs />
<button onclick={next}>Calibrate</button>
