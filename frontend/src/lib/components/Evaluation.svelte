<script lang="ts">
    import { appState } from "$lib/state.svelte.ts";
    import Button from "$lib/components/Button.svelte";
    import GainPattern from "$lib/components/GainPattern.svelte";

    function download(filename: string, data: string): void {
        const el = document.createElement("a");
        el.setAttribute("download", filename);
        el.setAttribute("href", data);
        el.style.display = "none";

        document.body.appendChild(el);
        el.click();
        document.body.removeChild(el);
    }

    function loadSvg(): SVGElement | null {
        const svg = document
            .getElementById("pattern-wrapper")
            ?.querySelector("svg");
        if (svg === null || svg === undefined) {
            return null;
        }

        return svg;
    }

    function svgToDataUrl(svg: SVGElement): string {
        const xml = new XMLSerializer().serializeToString(svg);

        return `data:image/svg+xml;charset=utf-8,${encodeURIComponent(xml)}`;
    }

    function downloadCsv(
        filename: string,
        contents: Array<Array<number | string>>,
    ): void {
        const data = encodeURIComponent(
            contents
                .map((line: Array<number | string>): string => line.join(","))
                .join("\n"),
        );
        download(filename, `data:text/plain;charset=utf-8,${data}`);
    }

    function onDownloadCsv() {
        const data = appState.values.csvValues();
        const date = new Date();
        const filename = `antenna-${date.toISOString().split("T")[0]}.csv`;

        downloadCsv(filename, data);
    }

    async function onDownloadPng() {
        const date = new Date();
        const filename = `antenna-${date.toISOString().split("T")[0]}.png`;
        const svg = loadSvg();
        if (svg === null) {
            return;
        }
        const height = svg.clientHeight;
        const width = svg.clientWidth;
        const svgData = svgToDataUrl(svg);

        const img = document.createElement("img");
        img.src = svgData;
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

        download(filename, dataUrl);
    }

    function onDownloadSvg() {
        const date = new Date();
        const filename = `antenna-${date.toISOString().split("T")[0]}.svg`;
        const svg = loadSvg();
        if (svg === null) {
            return;
        }
        const svgData = svgToDataUrl(svg);

        download(filename, svgData);
    }
</script>

<div id="pattern-wrapper">
    <GainPattern interpolate={true} values={appState.values} withRef={true} />
</div>

<div class="flex flex-row justify-center">
    <Button onclick={onDownloadCsv}>Download CSV</Button>
</div>
<div class="flex flex-row justify-center">
    <Button onclick={onDownloadPng}>Download PNG</Button>
</div>
<div class="flex flex-row justify-center">
    <Button onclick={onDownloadSvg}>Download SVG</Button>
</div>
