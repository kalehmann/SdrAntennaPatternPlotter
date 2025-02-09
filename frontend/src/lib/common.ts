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

export const dbfs2db = (dbfs: number, ref: number): number => {
    return (dbfs - ref) / 2;
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
