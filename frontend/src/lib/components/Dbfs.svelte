<script lang="ts">
    import { dbfs2db } from "$lib/common.ts";

    interface Props {
        reference?: number | null;
        value: (value: number) => void;
    }

    let { reference = null, value = () => {} }: Props = $props();

    const evtSource: EventSource = new EventSource("/sse");
    let dbfs: number = $state(-100.0);
    evtSource.onmessage = (event) => {
        dbfs = parseFloat(event.data);
        value(dbfs);
    };
</script>

<span>
    {dbfs.toFixed(2)} dbFS
    {#if reference !== null}
        ({dbfs2db(dbfs, reference).toFixed(2)} db<sub>Ref</sub>)
    {/if}
</span>
