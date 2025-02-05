<script lang="ts">
    interface Props {
        start: () => void;
    }

    let { start = () => {} }: Props = $props();
    let freq_khz: number = $state(145000);
    let measurement_count: number = $state(12);

    const onSubmit = (event) => {
        event.preventDefault();
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
    <h1 class="font-bold mb-2 text-xl">Basic Settings</h1>

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
                bind:value={freq_khz}
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
                type="number"
                bind:value={measurement_count}
            />
        </label>
        <p>
            This will have a measurement at every {(
                360 / measurement_count
            ).toFixed(2)} &deg
        </p>
        <div class="flex flex-row justify-center">
            <input
                id="start"
                name="start"
                class="mt-5
                       inline
                       text-neutral-100
                       bg-violet-500
                       p-2
                       px-5
                       rounded-md
                       focus:bg-violet-600
                       hover:bg-violet-600"
                type="submit"
                value="Start"
            />
        </div>
    </form>
</div>
