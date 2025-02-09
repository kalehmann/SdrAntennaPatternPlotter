<script lang="ts">
    import { polarToCartesian } from "$lib/common.ts";

    interface Props {
        direction: number;
        radius?: number;
        size?: number;
    }
    let { direction, radius = 40, size = 10 }: Props = $props();

    let markerPath = $derived.by(() => {
        const start = polarToCartesian(radius, direction - size / 2 - 90);
        const end = polarToCartesian(radius, direction + size / 2 - 90);
        const arc = `A ${radius} ${radius} 0 0 1 ${end.svgPath()}`;

        return `M ${start.svgPath()} ${arc}`;
    });
</script>

<defs>
    <filter id="blurred" width="20" height="20" x="-10" y="-10">
        <feGaussianBlur stdDeviation="0.5" />
    </filter>
</defs>
<path
    class="animate-pulse marker"
    d={markerPath}
    fill="none"
    filter="url(#blurred)"
    stroke-linecap="round"
    stroke-width="2.5"
/>
<path
    class="marker"
    d={markerPath}
    fill="none"
    stroke-linecap="round"
    stroke-width="1"
/>

<style>
    .marker {
        stroke: var(--color-amber-500);
    }
</style>
