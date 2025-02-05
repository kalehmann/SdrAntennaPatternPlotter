<script lang="ts">
    interface Props {
        current: number;
        total: number;
    }

    let { current = 1, total = 1 }: Props = $props();
</script>

<ul class="flex flex-row justify-center">
    {#each { length: total }, i}
        <li
            class="w-3 h-3 m-3 border-2 border-neutral-950 {i < current
                ? 'bg-violet-800'
                : ''} inline-block rounded-full"
        ></li>
    {/each}
</ul>
