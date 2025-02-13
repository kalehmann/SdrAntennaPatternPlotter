<script lang="ts">
    import { appState } from "$lib/state.svelte.ts";
    import Button from "$lib/components/Button.svelte";

    interface Props {
        start: () => void;
    }

    let { start = () => {} }: Props = $props();

    const onSubmit = (event: SubmitEvent) => {
        event.preventDefault();
        if (
            event.target === null ||
            event.target instanceof HTMLFormElement === false
        ) {
            return;
        }
        const formData = new FormData(event.target);
        const frequency = formData.get("freq");
        fetch("/frequency", {
            body: `${frequency}`,
            method: "POST",
        });
        start();
    };
</script>

<div class="h-full p-3">
    <h1 class="font-bold mb-2 text-2xl">Basic Settings</h1>

    <form onsubmit={onSubmit}>
        <label class="block">
            Frequency (kHz):
            <input
                id="freq"
                name="freq"
                class="block
                       w-full
                       border-1
                       border-transparent
                       border-b-violet-800
                       focus:shadow-md
                       focus:shadow-violet-300
                       focus-visible:outline-none
                       focus-visible:bg-white
                       mb-2"
                type="number"
                bind:value={appState.frequency}
            />
        </label>
        <label class="block">
            Number of measurements:
            <input
                id="measurement_count"
                name="measurement_sound"
                class="block
                       w-full
                       border-1
                       border-transparent
                       border-b-violet-800
                       focus:shadow-md
                       focus:shadow-violet-300
                       focus-visible:outline-none
                       focus-visible:bg-white
                       mb-2"
                min="1"
                type="number"
                bind:value={appState.steps}
            />
        </label>
        <p>
            This will have a measurement at every {(
                360 / appState.steps
            ).toFixed(2)} &deg
        </p>
        <div class="flex flex-row justify-center">
            <Button>Start</Button>
        </div>
    </form>
</div>
