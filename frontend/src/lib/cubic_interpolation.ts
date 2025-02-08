import { polarToCartesian, Vec2d } from "$lib/common.ts";

/*
 * Simple cubic spline interpolation.
 *
 * This class generates a closed SVG path of a cubic spline through all the
 * given points. Each of the given points has a tangent line - that is a line
 * passing throught the point, which is parallel to the line between the previous
 * and next points. The control points will be placed on that tangent line.
 */
export class CubicInterpolation {
    private points: Array<Vec2d>;
    private smoothing: number;
    private tangentLines: Array<Vec2d>;

    public constructor(points: Array<Vec2d>, smoothing: number = 0.2) {
        this.points = points;
        this.smoothing = smoothing;
        this.tangentLines = [];
        for (let i = 0; i < this.points.length; i++) {
            const prev = this.pointAtIndex(i - 1);
            const next = this.pointAtIndex(i + 1);
            this.tangentLines.push(new Vec2d(next.x - prev.x, next.y - prev.y));
        }
    }

    public svgPath = (): string => {
        let path = "";
        if (this.points.length === 0) {
            return path;
        }
        path += `M ${this.points[0].svgPath()}`;
        if (this.points.length === 1) {
            return path;
        }
        if (this.points.length === 2) {
            return `${path} L ${this.points[1].svgPath()}`;
        }
        const firstControlVector = polarToCartesian(
            this.tangentLines[0].distance() * this.smoothing,
            this.tangentLines[0].angle(),
        );
        const lastControlVector = polarToCartesian(
            this.tangentLines[0].distance() * this.smoothing,
            this.tangentLines[0].angle() + 180,
        );
        const secondControlVector = polarToCartesian(
            this.tangentLines[1].distance() * this.smoothing,
            this.tangentLines[1].angle() + 180,
        );
        const firstControlPoint = new Vec2d(
            this.points[0].x + firstControlVector.x,
            this.points[0].y + firstControlVector.y,
        );
        const secondControlPoint = new Vec2d(
            this.points[1].x + secondControlVector.x,
            this.points[1].y + secondControlVector.y,
        );
        const lastControlPoint = new Vec2d(
            this.points[0].x + lastControlVector.x,
            this.points[0].y + lastControlVector.y,
        );
        path += this.curveTo(
            firstControlPoint,
            secondControlPoint,
            this.points[1],
        );

        for (let i = 2; i < this.points.length; i++) {
            const controlVector = polarToCartesian(
                this.tangentLines[i].distance() * this.smoothing,
                this.tangentLines[i].angle() + 180,
            );
            const controlPoint = new Vec2d(
                this.points[i].x + controlVector.x,
                this.points[i].y + controlVector.y,
            );
            path += this.continueCurve(controlPoint, this.points[i]);
        }

        return path + this.continueCurve(lastControlPoint, this.points[0]);
    };

    private pointAtIndex = (i: number): Vec2d => {
        const n = this.points.length;
        while (i < 0) {
            i += n;
        }

        return this.points[i % n];
    };

    private curveTo = (
        firstControlPoint: Vec2d,
        secondControlPoint: Vec2d,
        destinationPoint: Vec2d,
    ): string => {
        return (
            ` C ${firstControlPoint.svgPath()}, ` +
            `${secondControlPoint.svgPath()} ` +
            `${destinationPoint.svgPath()}`
        );
    };

    private continueCurve = (
        controlPoint: Vec2d,
        destinationPoint: Vec2d,
    ): string => {
        return ` S ${controlPoint.svgPath()} ${destinationPoint.svgPath()}`;
    };
}
