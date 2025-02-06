<script lang="ts">
    interface Props {
        next: () => void;
        ref: number;
    }

    let { next = () => {}, ref = -100.0 }: Props = $props();
</script>

<button onclick={next}>Hello, world! {ref}</button>
