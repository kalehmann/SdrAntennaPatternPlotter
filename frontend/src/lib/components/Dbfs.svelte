<script lang="ts">
    import { dbfs2db } from "$lib/common.ts";
    import Notification from "$lib/components/Notification.svelte";

    interface Props {
        reference?: number | null;
        value: (value: number) => void;
    }

    let { reference = null, value = () => {} }: Props = $props();

    const evtSource: EventSource = new EventSource("/sse");
    let dbfs: number = $state(-100.0);
    let lastValueAt = $state(Date.now());
    let now = $state(Date.now());

    evtSource.onmessage = (event) => {
        dbfs = parseFloat(event.data);
        value(dbfs);
        lastValueAt = Date.now();
    };

    setInterval(() => {
        now = Date.now();
    }, 100);
</script>

<span>
    {dbfs.toFixed(1)} dbFS
    {#if reference !== null}
        ({dbfs2db(dbfs, reference).toFixed(1)} db<sub>Ref</sub>)
    {/if}
</span>

{#if now - lastValueAt > 2000}
    <Notification isWarning>
        Lost connection to server for
        {((now - lastValueAt) / 1000).toFixed(1)}
        seconds
    </Notification>
{/if}
