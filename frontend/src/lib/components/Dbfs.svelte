<script lang="ts">
    interface Props {
        value: (value: number) => void;
    }

    let { value = () => {} }: Props = $props();

    const evtSource: EventSource = new EventSource("/sse");
    let dbfs: string = $state("0.0");
    evtSource.onmessage = (event) => {
        value(parseFloat(event.data));
        dbfs = `${event.data}`;
    };
</script>

<span>{dbfs}dbFS</span>
