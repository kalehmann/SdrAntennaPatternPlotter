<script lang="ts">
    import type { Snippet } from "svelte";

    interface Props {
        children: Snippet;
    }

    let { children }: Props = $props();
</script>

<div
    class="h-screen
           w-screen
           fixed
           top-0
           left-0
           backdrop-blur-xs
           flex flex-row
           justify-center"
>
    <div class="m-15 p-10 rounded-lg w-full h-max bg-white">
        {@render children?.()}
    </div>
</div>
