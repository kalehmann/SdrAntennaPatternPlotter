<script lang="ts">
    import { Vec2d } from "$lib/common.ts";
    import { CubicInterpolation } from "$lib/cubic_interpolation.ts";
    import { MeasuredValues } from "$lib/measured_values.svelte.ts";

    interface Props {
        interpolate?: boolean;
        radius?: number;
        values: MeasuredValues;
    }
    let { interpolate = false, radius = 40, values }: Props = $props();

    let interpolationPath = $derived.by(() => {
        const interpolation = new CubicInterpolation(points, 0.2);

        return interpolation.svgPath();
    });
    let points: Array<Vec2d> = $derived.by(() => {
        return [...values.points(radius)];
    });
</script>

{#each { length: 4 }, i}
    <text
        fill="#000"
        filter="url(#solid)"
        font-size="2.5pt"
        text-anchor="middle"
        x={radius - 4 - (radius / 4) * i}
        y="4"
    >
        {0 - (i * values.scale) / 4}db
    </text>
{/each}
{#each points as point (point.angle())}
    <circle cx={point.x} cy={point.y} fill="#000" r="1" />
{/each}
{#if interpolate}
    <path
        id="interpolation"
        d={interpolationPath}
        fill="none"
        stroke="#000"
        stroke-width="0.2"
    />
{/if}
