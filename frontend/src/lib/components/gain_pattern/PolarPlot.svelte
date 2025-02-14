<script lang="ts">
    import { Vec2d } from "$lib/common.ts";
    import { CubicInterpolation } from "$lib/cubic_interpolation.ts";
    import { MeasuredValues } from "$lib/measured_values.svelte.ts";

    interface Props {
        interpolate?: boolean;
        radius?: number;
        values: MeasuredValues;
        withRef?: boolean;
    }
    let {
        interpolate = false,
        radius = 40,
        values,
        withRef = false,
    }: Props = $props();

    // Need to load the color from the variable here, as styles are not loaded
    // when the SVG is rendered as PNG or downloaded.
    const styles = getComputedStyle(document.documentElement);
    const refColor = styles.getPropertyValue("--color-emerald-700");

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
{#if withRef}
    <circle
        class="reference"
        fill="none"
        r={values.referenceRadius(radius)}
        stroke={refColor}
        stroke-width="0.5"
    />
{/if}
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
