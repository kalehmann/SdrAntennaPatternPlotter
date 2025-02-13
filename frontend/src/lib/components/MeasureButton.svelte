<script lang="ts">
    import type { Snippet } from "svelte";
    import Button from "$lib/components/Button.svelte";

    interface Props {
        children: Snippet;
        done: () => void;
        measuring_period?: number;
        start: () => void;
        stop: () => void;
    }

    let {
        children,
        done = () => {},
        measuring_period = 10.0,
        start = () => {},
        stop = () => {},
    }: Props = $props();

    let duration: number = $state(10.0);
    let interval: ReturnType<typeof setInterval> | undefined = undefined;
    let running: boolean = $state(false);

    function onclick() {
        if (!running) {
            duration = measuring_period;
            interval = setInterval(function () {
                duration -= 0.1;
                if (duration < 0.1) {
                    done();
                    clearInterval(interval);
                    interval = undefined;
                    running = false;
                }
            }, 100);
            running = true;
            start();
        } else {
            if (interval !== undefined) {
                clearInterval(interval);
                interval = undefined;
            }
            running = false;
            stop();
            duration = measuring_period;
        }
    }
</script>

<Button {onclick}>
    <div class="grid auto-cols-min grid-flow-col gap-2">
        {#if running}
            <span class="relative flex size-7 mr-2">
                <span
                    class="absolute
                           inline-flex
                           w-full
                           h-full
                           flex-col
                           justify-center
                           animate-ping
                           bg-purple-800
                           rounded-full"
                ></span>
                <span
                    class="relative
                           inline-flex
                           h-full
                           flex-col
                           center-items
                           justify-center
                           rounded-full"
                >
                    {duration.toFixed(1)}
                </span>
            </span>
        {/if}
        <span class="flex flex-col justify-center">{@render children?.()}</span>
    </div>
</Button>
