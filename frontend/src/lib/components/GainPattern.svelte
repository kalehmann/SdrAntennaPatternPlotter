<script lang="ts">
    import { MeasuredValues } from "$lib/measured_values.svelte.ts";
    import Compass from "$lib/components/gain_pattern/Compass.svelte";
    import Marker from "$lib/components/gain_pattern/Marker.svelte";
    import PolarGrid from "$lib/components/gain_pattern/PolarGrid.svelte";
    import PolarPlot from "$lib/components/gain_pattern/PolarPlot.svelte";

    interface Props {
        compass?: number | null;
        interpolate?: boolean;
        marker?: number | null;
        marker_size?: number;
        values: MeasuredValues;
    }
    let {
        compass = null,
        interpolate = false,
        marker = null,
        marker_size = 15,
        values,
    }: Props = $props();

    const radius = 40;
</script>

<svg
    class="w-full"
    viewBox="-50 -50 100 110"
    xmlns="http://www.w3.org/2000/svg"
    xmlns:xlink="http://www.w3.org/1999/xlink"
>
    <PolarGrid {radius}>
        {#if marker !== null}
            <Marker direction={marker} {radius} size={marker_size} />
        {/if}
        {#if compass !== null}
            <Compass direction={compass} {radius} />
        {/if}
        <PolarPlot {interpolate} {radius} {values} />
    </PolarGrid>
    <text
        alignment-baseline="middle"
        font-size="3pt"
        text-anchor="end"
        x="47"
        y="50"
    >
        0 db = {values.maxDbRef.toFixed(1)} dbRef
    </text>
</svg>
