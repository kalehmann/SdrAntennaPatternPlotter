interface AppState {
    frequency: number;
    measurements: Array<[number, number]>;
    reference_dbfs: number;
    steps: number;
}

export const appState: AppState = $state({
    frequency: 145_000,
    measurements: [],
    reference_dbfs: -100.0,
    steps: 12,
});
