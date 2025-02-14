<script lang="ts">
    import type { Snippet } from "svelte";

    interface Props {
        children?: Snippet;
        onclick?: (event?: MouseEvent) => void;
    }
    let { children, onclick = () => {} }: Props = $props();
</script>

<button
    class="mt-5
           m-1
           inline
           text-neutral-100
           bg-violet-500
           p-2
           px-5
           rounded-md
           focus:bg-violet-600
           hover:bg-violet-600"
    {onclick}
>
    {@render children?.()}
</button>
