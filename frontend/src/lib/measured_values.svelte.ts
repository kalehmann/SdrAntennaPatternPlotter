import { dbfs2db, Vec2d } from "$lib/common.ts";
import { SvelteMap } from "svelte/reactivity";

export class MeasuredValues {
    private scaleBase: number;

    private values: Map<number, number> = $state(new SvelteMap());

    public maxDbfs = $derived.by(() => {
        if (this.referenceValue !== null) {
            return Math.max(...this.values.values(), this.referenceValue);
        }

        return Math.max(...this.values.values());
    });

    public maxDbRef = $derived.by(() => {
        if (this.referenceValue !== null) {
            return dbfs2db(this.maxDbfs, this.referenceValue);
        }

        return 0.0;
    });

    public minDbfs = $derived.by(() => {
        if (this.referenceValue !== null) {
            return Math.min(...this.values.values(), this.referenceValue);
        }

        return Math.min(...this.values.values());
    });

    public points = $derived.by(() => {
        return (radius: number) => {
            const reference =
                this.referenceValue !== null
                    ? this.referenceValue
                    : this.maxDbfs;
            return this.values.entries().map(([angle, dbfs]) => {
                const a = angle - 90;
                const db = dbfs2db(dbfs, reference);
                const maxDb = dbfs2db(this.maxDbfs, reference);
                const r = ((this.scale + db - maxDb) / this.scale) * radius;

                return Vec2d.fromPolar(r, a);
            });
        };
    });

    public referenceRadius = $derived.by(() => {
        return (radius: number) => {
            if (this.referenceValue === null) {
                return 0;
            }

            const maxDb = dbfs2db(this.maxDbfs, this.referenceValue);

            return ((this.scale - maxDb) / this.scale) * radius;
        };
    });

    public referenceValue: number | null = $state(null);

    public scale = $derived.by(() => {
        // Gets the smallest multiple of scaleBase, that is still larger than
        // the difference between minimal and maximal value in db.
        const dbRange = dbfs2db(this.maxDbfs, this.minDbfs);

        return Math.max(
            Math.ceil(dbRange / this.scaleBase) * this.scaleBase,
            1,
        );
    });

    public constructor(scaleBase = 4) {
        this.referenceValue = null;
        this.scaleBase = scaleBase;
        this.values.clear();
    }

    public addValue = (angle: number, dbfs: number): void => {
        this.values.set(angle, dbfs);
    };

    public csvValues = (): Array<Array<string>> => {
        const reference = this.referenceValue;

        if (reference !== null) {
            return [
                ["Angle", "Gain (dbFS)", "dbFS - reference"],
                ...this.values
                    .entries()
                    .map(([angle, dbfs]): [string, string, string] => [
                        angle.toFixed(0),
                        dbfs.toFixed(2),
                        dbfs2db(dbfs, reference).toFixed(2),
                    ]),
            ];
        }

        return [
            ["Angle", "Gain (dbFS)"],
            ...this.values
                .entries()
                .map(([angle, dbfs]): [string, string] => [
                    angle.toFixed(0),
                    dbfs.toFixed(2),
                ]),
        ];
    };

    public setReference = (value: number | null): void => {
        this.referenceValue = value;
    };
}
