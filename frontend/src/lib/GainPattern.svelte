<script lang="ts">
    interface Point {
        x: number;
        y: number;
    }
    interface Props {
        marker: number;
        marker_size: number;
        measurements;
        ref: number;
        show_marker: boolean;
    }
    let {
        marker = 0,
        marker_size = 15,
        measurements = [],
        ref = -100.0,
        show_marker = true,
    }: Props = $props();

    function dbfs2db(dbfs: number, ref: number): number {
        return (dbfs - ref) / 2;
    }
    function polar(r: number, angle: number): Point {
        const rad: number = (angle * Math.PI) / 180.0;
        const x: number = r * Math.cos(rad);
        const y: number = r * Math.sin(rad);

        return { x, y };
    }

    let markerPath = $derived.by(() => {
        const start = polar(40, marker - marker_size / 2 - 90);
        const end = polar(40, marker + marker_size / 2 - 90);

        return (
            `M ${start.x.toFixed(2)} ${start.y.toFixed(2)} ` +
            "A 40 40 0 0 1 " +
            `${end.x.toFixed(2)} ${end.y.toFixed(2)}`
        );
    });
    let maxDbfs = $derived.by(() => {
        return Math.max(...measurements.map(([, dbfs]) => dbfs), ref);
    });
    let minDbfs = $derived.by(() => {
        return Math.min(...measurements.map(([, dbfs]) => dbfs), ref);
    });
    let points = $derived.by(() => {
        return measurements.map(([angle, dbfs]) => {
            return [angle, dbfs2db(dbfs, ref) - dbfs2db(maxDbfs, ref)];
        });
    });
    let scale = $derived.by(() => {
        // Gets the smallest multiple of 4, that is still larger than the
        // difference between minimal and maximal value in db.
        return Math.max(Math.ceil(dbfs2db(maxDbfs, minDbfs) / 4) * 4, 1);
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
            x1={polar(40, i * 30).x}
            y1={polar(40, i * 30).y}
            x2={polar(40, i * 30 - 180).x}
            y2={polar(40, i * 30 - 180).y}
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
            filter="url(#blurred)"
            stroke-linecap="round"
            stroke-width="1.5"
        />
        <path
            class="marker"
            d={markerPath}
            stroke-linecap="round"
            stroke-width="1"
        />
    {/if}
    {#each { length: 12 }, i}
        <text
            alignment-baseline="middle"
            font-size="3pt"
            text-anchor="middle"
            x={polar(45, i * 30 - 90).x}
            y={polar(45, i * 30 - 90).y}
        >
            {i * 30 > 180 ? i * 30 - 360 : i * 30}&deg
        </text>
    {/each}
    {#each { length: 4 }, i}
        <text
            alignment-baseline="top"
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
    {#each points as [angle, db] (angle)}
        <circle
            cx={polar(((scale + db) / scale) * 40, angle - 90).x}
            cy={polar(((scale + db) / scale) * 40, angle - 90).y}
            fill="#000"
            r="1"
        />
    {/each}
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
    .marker {
        stroke: var(--color-amber-500);
    }
</style>
