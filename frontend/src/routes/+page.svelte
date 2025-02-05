<script lang="ts">
    import BasicSettings from "$lib/BasicSettings.svelte";
    import Calibration from "$lib/Calibration.svelte";
    import Progress from "$lib/Progress.svelte";

    let step: number = 1;
</script>

<div class="bg-neutral-300 flex flex-col h-screen">
    <main class="grow">
        {#if step === 1}
            <BasicSettings start={() => (step += 1)} />
        {:else if step === 2}
            <Calibration next={() => (step += 1)} />
        {:else if step === 3}{:else}{/if}
    </main>
    <footer class="flex-none mb-5">
        <Progress current={step} total="4" />
    </footer>
</div>
