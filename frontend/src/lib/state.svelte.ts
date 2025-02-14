import { MeasuredValues } from "$lib/measured_values.svelte.ts";

interface AppState {
    frequency: number;
    steps: number;
    values: MeasuredValues;
}

export const appState: AppState = $state({
    frequency: 145_000,
    steps: 12,
    values: new MeasuredValues(),
});
