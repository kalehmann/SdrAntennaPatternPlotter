import { dbfs2db, Vec2d } from "$lib/common.ts";
import { SvelteMap } from "svelte/reactivity";

export class MeasuredValues {
    private scaleBase: number;

    private values: Map<number, number> = $state(new SvelteMap());

    public maxDbfs = $derived.by(() => {
        return Math.max(...this.values.values(), this.referenceValue);
    });

    public maxDbRef = $derived.by(() => {
        return dbfs2db(this.maxDbfs, this.referenceValue);
    });

    public minDbfs = $derived.by(() => {
        return Math.min(...this.values.values(), this.referenceValue);
    });

    public points = $derived.by(() => {
        return (radius: number) => {
            return this.values.entries().map(([angle, dbfs]) => {
                const a = angle - 90;
                const db = dbfs2db(dbfs, this.referenceValue);
                const maxDb = dbfs2db(this.maxDbfs, this.referenceValue);
                const r = ((this.scale + db - maxDb) / this.scale) * radius;

                return Vec2d.fromPolar(r, a);
            });
        };
    });

    public referenceRadius = $derived.by(() => {
        return (radius: number) => {
            const maxDb = dbfs2db(this.maxDbfs, this.referenceValue);

            return ((this.scale - maxDb) / this.scale) * radius;
        };
    });

    public referenceValue: number = $state(0.0);

    public scale = $derived.by(() => {
        // Gets the smallest multiple of scaleBase, that is still larger than
        // the difference between minimal and maximal value in db.
        const dbRange = dbfs2db(this.maxDbfs, this.minDbfs);

        return Math.max(
            Math.ceil(dbRange / this.scaleBase) * this.scaleBase,
            1,
        );
    });

    public constructor(referenceValue: number, scaleBase = 4) {
        this.referenceValue = referenceValue;
        this.scaleBase = scaleBase;
        this.values.clear();
    }

    public addValue = (angle: number, dbfs: number): void => {
        this.values.set(angle, dbfs);
    };

    public csvValues = (): Array<[string, string, string]> => {
        return [
            ["Angle", "Gain (dbFS)", "Gain (dbRef)"],
            ...this.values
                .entries()
                .map(([angle, dbfs]): [string, string, string] => [
                    angle.toFixed(0),
                    dbfs.toFixed(2),
                    dbfs2db(dbfs, this.referenceValue).toFixed(2),
                ]),
        ];
    };

    public setReference = (value: number): void => {
        this.referenceValue = value;
    };
}
