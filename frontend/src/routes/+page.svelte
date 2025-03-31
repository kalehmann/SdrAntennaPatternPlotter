<script lang="ts">
    import { appState } from "$lib/state.svelte.ts";
    import { MeasuredValues } from "$lib/measured_values.svelte.ts";
    import BasicSettings from "$lib/components/BasicSettings.svelte";
    import Calibration from "$lib/components/Calibration.svelte";
    import Evaluation from "$lib/components/Evaluation.svelte";
    import Measurement from "$lib/components/Measurement.svelte";
    import Progress from "$lib/components/Progress.svelte";

    let step: number = $state(1);

    function next() {
        step += 1;
    }

    function startOver() {
        appState.values = new MeasuredValues();
        step = 1;
    }
</script>

<main class="grow">
    {#if step === 1}
        <BasicSettings start={next} />
    {:else if step === 2}
        <Calibration {next} />
    {:else if step === 3}
        <Measurement {next} />
    {:else}
        <Evaluation {startOver} />
    {/if}
</main>
<footer class="flex-none mb-5">
    <Progress current={step} total={4} />
</footer>
