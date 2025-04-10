<script lang="ts">
    import { appState } from "$lib/state.svelte.ts";
    import { Compass } from "$lib/compass.svelte.ts";
    import { limit } from "$lib/common.ts";
    import { MeasuredValues } from "$lib/measured_values.svelte.ts";
    import Button from "$lib/components/Button.svelte";
    import Dbfs from "$lib/components/Dbfs.svelte";
    import GainPattern from "$lib/components/GainPattern.svelte";
    import MeasureButton from "$lib/components/MeasureButton.svelte";
    import Modal from "$lib/components/Modal.svelte";

    interface Props {
        next: () => void;
    }
    let { next = () => {} }: Props = $props();

    const compass: Compass = new Compass();
    const stepSize = 360 / appState.steps;
    const markerSize = limit(stepSize / 2, 5, 20);

    let compassDirection = $derived.by(() => {
        if (!compass.available || currentStep === 0) {
            return null;
        }

        return compass.direction;
    });
    let currentStep: number = $state(0.0);
    let currentValue: number = -100.0;
    let measuring: boolean = false;
    let nextAngle: number = $derived.by(() => {
        const angle = currentStep * stepSize;
        if (angle <= 180) {
            return angle;
        }

        return angle - 360;
    });
    let showModal: boolean = $state(true);

    function loadDummyData() {
        appState.values = new MeasuredValues();
        appState.values.setReference(-17.54);
        appState.values.addValue(0, -14.46);
        appState.values.addValue(30, -19.96);
        appState.values.addValue(60, -31.28);
        appState.values.addValue(90, -31.34);
        appState.values.addValue(120, -31.33);
        appState.values.addValue(150, -27.51);
        appState.values.addValue(180, -24.42);
        appState.values.addValue(210, -26.7);
        appState.values.addValue(240, -31.2);
        appState.values.addValue(270, -31.24);
        appState.values.addValue(300, -31.33);
        appState.values.addValue(330, -19.64);
        next();
    }
    function onMeasureDone() {
        const angle = currentStep * stepSize;
        appState.values.addValue(angle, currentValue);
        if (currentStep === 0) {
            compass.finishCalibration();
        }
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
        if (currentStep === 0) {
            compass.startCalibration();
        }
    }
    function onMeasureStop() {
        currentValue = -100.0;
        measuring = false;
        if (currentStep === 0) {
            compass.stopCalibration();
        }
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
        {nextAngle.toFixed(0)}&deg points torwards the receiver, press the
        measure button and hold your antenna steady.
    </p>
    <div class="flex m-5 flex-row justify-center text-xl">
        Current value: &nbsp
        <Dbfs reference={appState.values.referenceValue} value={onValue} />
    </div>
    <GainPattern
        compass={compassDirection}
        marker={currentStep * stepSize}
        marker_size={markerSize}
        values={appState.values}
    />
    <div class="flex flex-row justify-center">
        <MeasureButton
            done={onMeasureDone}
            start={onMeasureStart}
            stop={onMeasureStop}
        >
            Measure
        </MeasureButton>
        {#if import.meta.env.DEV}
            <Button onclick={loadDummyData}>Load dummy values</Button>
        {/if}
    </div>
</div>

{#if showModal}
    <Modal>
        <p>
            Now connect the signal source to the antenna that should be
            measured.
        </p>
        {#if compass.available}
            <p>The compass will be calibrated during the first measurement.</p>
        {/if}

        <Button
            onclick={() => {
                showModal = false;
            }}
        >
            Ok
        </Button>
    </Modal>
{/if}
