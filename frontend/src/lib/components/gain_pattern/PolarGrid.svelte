<script lang="ts">
    import { polarToCartesian } from "$lib/common.ts";
    import type { Snippet } from "svelte";

    interface Props {
        children?: Snippet;
        radius?: number;
    }
    let { children, radius = 40 }: Props = $props();
</script>

<defs>
    <filter x="0" y="0" width="1" height="1" id="solid">
        <feFlood flood-color="#fff" result="bg" />
        <feMerge>
            <feMergeNode in="bg" />
            <feMergeNode in="SourceGraphic" />
        </feMerge>
    </filter>
</defs>
<circle cx="0" cy="0" fill="#fff" r={radius} />
<circle cx="0" cy="0" r={radius} fill="none" stroke="#000" stroke-width="0.5" />
{#each { length: 6 }, i}
    <line
        stroke="#222"
        stroke-width={i % 3 === 0 ? 0.2 : 0.1}
        x1={polarToCartesian(radius, i * 30).x}
        y1={polarToCartesian(radius, i * 30).y}
        x2={polarToCartesian(radius, i * 30 - 180).x}
        y2={polarToCartesian(radius, i * 30 - 180).y}
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
{#each { length: 12 }, i}
    <text
        alignment-baseline="middle"
        font-size="3pt"
        text-anchor="middle"
        x={polarToCartesian(radius + 5, i * 30 - 90).x}
        y={polarToCartesian(radius + 5, i * 30 - 90).y}
    >
        {i * 30 > 180 ? i * 30 - 360 : i * 30}&deg
    </text>
{/each}
{@render children?.()}
