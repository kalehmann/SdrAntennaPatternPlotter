<script lang="ts">
    import type { Snippet } from "svelte";

    interface Props {
        children: Snippet;
        isWarning?: boolean;
    }

    let { children, isWarning = false }: Props = $props();

    let cssClasses = $derived.by(() => {
        const classes = [
            "m-5",
            "p-5",
            "rounded-lg",
            "w-full",
            "h-max",
            "drop-shadow-lg",
        ];

        if (isWarning) {
            classes.push("bg-pink-900");
            classes.push("text-white");
        } else {
            classes.push("bg-white");
        }

        return classes.join(" ");
    });
</script>

<div
    class="w-screen
           fixed
           top-0
           left-0
           flex flex-row
           justify-center"
>
    <div class={cssClasses}>
        {@render children?.()}
    </div>
</div>
