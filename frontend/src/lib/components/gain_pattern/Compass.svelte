<script lang="ts">
    import { polarToCartesian } from "$lib/common.ts";

    interface Props {
        direction: number;
        radius?: number;
    }
    let { direction, radius = 40 }: Props = $props();

    let compassPath = $derived.by(() => {
        const start = polarToCartesian(radius * 0.1, direction - 90);
        const end = polarToCartesian(radius * 0.9, direction - 90);

        return `M ${start.svgPath()} L ${end.svgPath()}`;
    });
</script>

<path class="compass" d={compassPath} stroke-linecap="round" stroke-width="1" />

<style>
    .compass {
        stroke: var(--color-violet-500);
    }
</style>
