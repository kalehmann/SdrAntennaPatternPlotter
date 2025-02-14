<script lang="ts">
    import { appState } from "$lib/state.svelte.ts";
    import { colors } from "$lib/common.ts";
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
        withRef?: boolean;
    }
    let {
        compass = null,
        interpolate = false,
        marker = null,
        marker_size = 15,
        values,
        withRef = false,
    }: Props = $props();

    const radius = 40;
</script>

<svg
    class="w-full"
    viewBox="-50 -60 100 120"
    xmlns="http://www.w3.org/2000/svg"
    xmlns:xlink="http://www.w3.org/1999/xlink"
>
    <text
        alignment-baseline="middle"
        font-size="3pt"
        text-anchor="start"
        x="-45"
        y="-53"
    >
        Frequency: {(appState.frequency / 1000.0).toFixed(3)} MHz
    </text>
    <PolarGrid {radius}>
        {#if marker !== null}
            <Marker direction={marker} {radius} size={marker_size} />
        {/if}
        {#if compass !== null}
            <Compass direction={compass} {radius} />
        {/if}
        <PolarPlot {interpolate} {radius} {values} {withRef} />
    </PolarGrid>
    <text
        alignment-baseline="middle"
        font-size="3pt"
        text-anchor="start"
        x="-45"
        y="52"
    >
        <tspan>Reference antenna: </tspan>
        <tspan fill={colors.reference}>
            {appState.values.referenceValue.toFixed(1)} dbFS
        </tspan>
        <tspan x="47" text-anchor="end">
            0 db = {values.maxDbfs.toFixed(1)} dbFS
        </tspan>
    </text>
</svg>
