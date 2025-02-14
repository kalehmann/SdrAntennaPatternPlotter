<script lang="ts">
    import { appState } from "$lib/state.svelte.ts";
    import Button from "$lib/components/Button.svelte";
    import Dbfs from "$lib/components/Dbfs.svelte";
    import MeasureButton from "$lib/components/MeasureButton.svelte";
    import Modal from "$lib/components/Modal.svelte";

    interface Props {
        next: () => void;
    }

    let { next = () => {} }: Props = $props();
    let calibrating: boolean = $state(false);
    let result: number = $state(-100.0);
    let showModal: boolean = $state(true);

    function onValue(value: number) {
        if (calibrating) {
            if (value > result) {
                result = value;
            }
        }
    }
</script>

<div class="flex m-5 flex-col items-center">
    <p class="mb-2">
        Place the reference antenna in the same place the actual antenna is
        going to be measured.
    </p>
    <p>
        Current value: <Dbfs value={onValue} />
    </p>
</div>
<div class="flex flex-row justify-center">
    <MeasureButton
        done={() => {
            appState.values.setReference(result);
            next();
        }}
        start={() => {
            calibrating = true;
        }}
        stop={() => {
            calibrating = false;
            result = -100.0;
        }}
    >
        Calibrate
    </MeasureButton>
    <Button onclick={next}>Skip</Button>

    {#if showModal}
        <Modal>
            <p>Now connect the reference antenna to the signal source.</p>

            <Button
                onclick={() => {
                    showModal = false;
                }}
            >
                Ok
            </Button>
        </Modal>
    {/if}
</div>
