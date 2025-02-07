<script lang="ts">
    import { appState } from "$lib/state.svelte.ts";
    import Dbfs from "$lib/Dbfs.svelte";
    import MeasureButton from "$lib/MeasureButton.svelte";
    import Modal from "$lib/Modal.svelte";

    interface Props {
        next: (result: number) => void;
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

<div class="flex m-5 flex-row justify-center">
    Current value: <Dbfs value={onValue} />
</div>
<div class="flex flex-row justify-center">
    <MeasureButton
        done={() => {
            appState.reference_dbfs = result;
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

    {#if showModal}
        <Modal>
            <p>Now connect the reference antenna to the signal source.</p>

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
</div>
