<script lang="ts">
    const evtSource: EventSource = new EventSource("/sse");
    export let dbfs: String = '0.0';
    evtSource.onmessage = (event) => {
        dbfs = `${event.data}`;
    };
</script>

<p>{dbfs}dbFS</p>
