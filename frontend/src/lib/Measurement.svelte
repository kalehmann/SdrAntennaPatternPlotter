<script lang="ts">
    import { appState } from "$lib/state.svelte.ts";
    import Dbfs from "$lib/Dbfs.svelte";
    import GainPattern from "$lib/GainPattern.svelte";
    import MeasureButton from "$lib/MeasureButton.svelte";
    import Modal from "$lib/Modal.svelte";

    interface Props {
        next: () => void;
    }

    let { next = () => {} }: Props = $props();
    let currentStep = $state(0.0);
    let currentValue = -100.0;
    let measuring: boolean = false;
    let showModal: boolean = $state(true);
    const stepSize = 360 / appState.steps;

    function onMeasureDone() {
        const angle = currentStep * stepSize;
        appState.measurements.push([angle, currentValue]);
        currentStep += 1;
        if (currentStep === appState.steps) {
            next();
        }
        currentValue = -100.0;
        measuring = false;
    }
    function onMeasureStart() {
        currentValue = -100.0;
        measuring = true;
    }
    function onMeasureStop() {
        currentValue = -100.0;
        measuring = false;
    }
    function onValue(value: number) {
        if (measuring && value > currentValue) {
            currentValue = value;
        }
    }
</script>

<div class="flex flex-col">
    <p class="m-3 text-xl">
        Turn your device and antenna so that
        {(currentStep * stepSize).toFixed(0)}&deg points torwards the receiver,
        press the measure button and hold your antenna steady.
    </p>
    <div class="flex m-5 flex-row justify-center text-xl">
        Current value: &nbsp <Dbfs value={onValue} />
    </div>
    <GainPattern
        marker={currentStep * stepSize}
        marker_size={stepSize / 2}
        measurements={appState.measurements}
        ref={appState.reference_dbfs}
    />
    <div class="flex flex-row justify-center">
        <MeasureButton
            done={onMeasureDone}
            measuring_period={5.0}
            start={onMeasureStart}
            stop={onMeasureStop}
        >
            Measure
        </MeasureButton>
    </div>
</div>

{#if showModal}
    <Modal>
        <p>
            Now connect the signal source to the antenna that should be
            measured.
        </p>

        <button
            class="mt-5
                   inline
                   text-neutral-100
                   bg-violet-500
                   p-2
                   px-5
                   rounded-md
                   focus:bg-violet-600
                   hover:bg-violet-600"
            onclick={() => {
                showModal = false;
            }}
        >
            Ok
        </button>
    </Modal>
{/if}
