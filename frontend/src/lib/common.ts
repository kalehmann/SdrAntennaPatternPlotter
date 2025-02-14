export class Vec2d {
    public x: number;
    public y: number;

    public constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }

    public static fromPolar = (r: number, angle: number): Vec2d => {
        return polarToCartesian(r, angle);
    };

    public angle = (): number => {
        return (Math.atan2(this.y, this.x) * 180) / Math.PI;
    };

    public distance = (): number => {
        return Math.sqrt(Math.pow(this.x, 2) + Math.pow(this.y, 2));
    };

    public svgPath = (): string => {
        return `${this.x.toFixed(2)} ${this.y.toFixed(2)}`;
    };
}

const styles = getComputedStyle(document.documentElement);

export const colors = {
    reference: styles.getPropertyValue("--color-emerald-700"),
};

export const dbfs2db = (dbfs: number, ref: number): number => {
    // dbFS is assumed to be linear to the received power.
    // See https://groups.google.com/g/gqrx/c/hlUl0CEswvE/m/wvsXqNWABQAJ
    return dbfs - ref;
};

export const limit = (value: number, min: number, max: number): number => {
    return Math.max(Math.min(value, max), min);
};

export const polarToCartesian = (r: number, angle: number): Vec2d => {
    const rad: number = (angle * Math.PI) / 180.0;
    const x: number = r * Math.cos(rad);
    const y: number = r * Math.sin(rad);

    return new Vec2d(x, y);
};
