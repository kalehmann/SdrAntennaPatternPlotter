<script lang="ts">
    import { appState } from "$lib/state.svelte.ts";
    import GainPattern from "$lib/components/GainPattern.svelte";

    function downloadCsv(
        filename: string,
        contents: Array<Array<number | string>>,
    ): void {
        const data = encodeURIComponent(
            contents
                .map((line: Array<number | string>): string => line.join(","))
                .join("\n"),
        );
        const el = document.createElement("a");
        el.setAttribute("download", filename);
        el.setAttribute("href", `data:text/plain;charset=utf-8,${data}`);
        el.style.display = "none";

        document.body.appendChild(el);
        el.click();
        document.body.removeChild(el);
    }

    function onDownloadCsv() {
        const data = [["Angle", "Gain (dbFS)"], ...appState.measurements];
        const date = new Date();
        const filename = `antenna-${date.toISOString().split("T")[0]}.csv`;

        downloadCsv(filename, data);
    }

    async function onDownloadPng() {
        const svg = document
            .getElementById("pattern-wrapper")
            ?.querySelector("svg");
        if (svg === null || svg === undefined) {
            return;
        }
        const date = new Date();
        const filename = `antenna-${date.toISOString().split("T")[0]}.png`;
        const height = svg.clientHeight;
        const width = svg.clientWidth;
        const xml = new XMLSerializer().serializeToString(svg);

        const img = document.createElement("img");
        img.src = `data:image/svg+xml;charset=utf-8,${encodeURIComponent(xml)}`;
        await new Promise((resolve, reject) => {
            img.onerror = reject;
            img.onload = resolve;
        });

        const canvas = document.createElement("canvas");
        canvas.height = height * 2;
        canvas.width = width * 2;
        const context = canvas.getContext("2d");
        if (context === null) {
            return;
        }
        context.drawImage(img, 0, 0, width * 2, height * 2);
        const dataUrl = await canvas.toDataURL("image/png", 1.0);

        const el = document.createElement("a");
        el.setAttribute("download", filename);
        el.setAttribute("href", dataUrl);
        el.style.display = "none";

        document.body.appendChild(el);
        el.click();
        document.body.removeChild(el);
    }
</script>

<div id="pattern-wrapper">
    <GainPattern
        interpolate={true}
        measurements={appState.measurements}
        ref={appState.reference_dbfs}
        show_marker={false}
    />
</div>

<div class="flex flex-row justify-center">
    <button
        class="mt-5
               auto-cols-min
               grid-flow-col
               gap-2
               text-neutral-100
               bg-violet-500
               p-3
               px-5
               rounded-md
               focus:bg-violet-600
               hover:bg-violet-600"
        onclick={onDownloadCsv}
    >
        Download CSV
    </button>
</div>
<div class="flex flex-row justify-center">
    <button
        class="mt-5
               auto-cols-min
               grid-flow-col
               gap-2
               text-neutral-100
               bg-violet-500
               p-3
               px-5
               rounded-md
               focus:bg-violet-600
               hover:bg-violet-600"
        onclick={onDownloadPng}
    >
        Download PNG
    </button>
</div>
