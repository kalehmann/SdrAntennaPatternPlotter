<script lang="ts">
    let freq_khz: number = 145000;

    const onSubmit = (event) => {
        const formData = new FormData(event.target);
        const frequency = formData.get("freq");
        fetch("/frequency", {
            body: `${frequency}`,
            method: "POST",
        });
    };
</script>

<form on:submit|preventDefault={onSubmit}>
    <label>
        Frequency (kHz):
        <input type="number" id="freq" name="freq" bind:value={freq_khz} />
    </label>
    <input type="submit" id="sub" value="Change frequency" />
</form>
