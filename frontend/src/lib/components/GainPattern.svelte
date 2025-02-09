<script lang="ts">
    import { polarToCartesian, Vec2d } from "$lib/common.ts";
    import { CubicInterpolation } from "$lib/cubic_interpolation.ts";

    interface Props {
        compass?: number;
        interpolate?: boolean;
        marker?: number;
        marker_size?: number;
        measurements: Array<[number, number]>;
        ref: number;
        show_compass?: boolean;
        show_marker?: boolean;
    }
    let {
        compass = 0.0,
        interpolate = false,
        marker = 0.0,
        marker_size = 15,
        measurements = [],
        ref = -100.0,
        show_compass = false,
        show_marker = true,
    }: Props = $props();

    function dbfs2db(dbfs: number, ref: number): number {
        return (dbfs - ref) / 2;
    }

    let compassPath = $derived.by(() => {
        const start = polarToCartesian(3, compass - 90);
        const end = polarToCartesian(37, compass - 90);

        return `M ${start.svgPath()} L ${end.svgPath()}`;
    });

    let markerPath = $derived.by(() => {
        const start = polarToCartesian(40, marker - marker_size / 2 - 90);
        const end = polarToCartesian(40, marker + marker_size / 2 - 90);

        return `M ${start.svgPath()} A 40 40 0 0 1 ${end.svgPath()}`;
    });
    let maxDbfs = $derived.by(() => {
        return Math.max(...measurements.map(([, dbfs]) => dbfs), ref);
    });
    let minDbfs = $derived.by(() => {
        return Math.min(...measurements.map(([, dbfs]) => dbfs), ref);
    });
    let points: Array<Vec2d> = $derived.by(() => {
        return measurements.map(([angle, dbfs]) => {
            const a = angle - 90;
            const db = dbfs2db(dbfs, ref) - dbfs2db(maxDbfs, ref);
            const r = ((scale + db) / scale) * 40;

            return Vec2d.fromPolar(r, a);
        });
    });
    let scale = $derived.by(() => {
        // Gets the smallest multiple of 4, that is still larger than the
        // difference between minimal and maximal value in db.
        return Math.max(Math.ceil(dbfs2db(maxDbfs, minDbfs) / 4) * 4, 1);
    });

    let interpolationPath = $derived.by(() => {
        const interpolation = new CubicInterpolation(points, 0.2);

        return interpolation.svgPath();
    });
</script>

<svg
    class="w-full"
    viewBox="-50 -50 100 110"
    xmlns="http://www.w3.org/2000/svg"
    xmlns:xlink="http://www.w3.org/1999/xlink"
>
    <defs>
        <filter x="0" y="0" width="1" height="1" id="solid">
            <feFlood flood-color="#fff" result="bg" />
            <feMerge>
                <feMergeNode in="bg" />
                <feMergeNode in="SourceGraphic" />
            </feMerge>
        </filter>
        <filter id="blurred" width="20" height="20" x="-10" y="-10">
            <feGaussianBlur stdDeviation="0.5" />
        </filter>
    </defs>
    <circle cx="0" cy="0" fill="#fff" r="40" />
    <circle cx="0" cy="0" r="40" fill="none" stroke="#000" stroke-width="0.5" />
    {#each { length: 6 }, i}
        <line
            stroke="#222"
            stroke-width={i % 3 === 0 ? 0.2 : 0.1}
            x1={polarToCartesian(40, i * 30).x}
            y1={polarToCartesian(40, i * 30).y}
            x2={polarToCartesian(40, i * 30 - 180).x}
            y2={polarToCartesian(40, i * 30 - 180).y}
        />
    {/each}
    {#each { length: 3 }, i}
        <circle
            cx="0"
            cy="0"
            r={i * 10 + 10}
            fill="none"
            stroke="#000"
            stroke-width="0.2"
        />
    {/each}
    {#if show_marker}
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
    {/if}
    {#if show_compass}
        <path
            class="compass"
            d={compassPath}
            stroke-linecap="round"
            stroke-width="1"
        />
    {/if}
    {#each { length: 12 }, i}
        <text
            alignment-baseline="middle"
            font-size="3pt"
            text-anchor="middle"
            x={polarToCartesian(45, i * 30 - 90).x}
            y={polarToCartesian(45, i * 30 - 90).y}
        >
            {i * 30 > 180 ? i * 30 - 360 : i * 30}&deg
        </text>
    {/each}
    {#each { length: 4 }, i}
        <text
            fill="#000"
            filter="url(#solid)"
            font-size="2.5pt"
            text-anchor="middle"
            x={36 - 10 * i}
            y="4"
        >
            {0 - (i * scale) / 4}db
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
    <text
        alignment-baseline="middle"
        font-size="3pt"
        text-anchor="end"
        x="47"
        y="50"
    >
        0 db = {dbfs2db(maxDbfs, ref).toFixed(1)} dbRef
    </text>
</svg>

<style>
    .compass {
        stroke: var(--color-violet-500);
    }

    .marker {
        stroke: var(--color-amber-500);
    }
</style>
