<script lang="ts">
    import BasicSettings from "$lib/BasicSettings.svelte";
    import Calibration from "$lib/Calibration.svelte";
    import Evaluation from "$lib/Evaluation.svelte";
    import Measurement from "$lib/Measurement.svelte";
    import Progress from "$lib/Progress.svelte";

    let step: number = $state(1);

    function next() {
        step += 1;
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
        <Evaluation />
    {/if}
</main>
<footer class="flex-none mb-5">
    <Progress current={step} total="4" />
</footer>
