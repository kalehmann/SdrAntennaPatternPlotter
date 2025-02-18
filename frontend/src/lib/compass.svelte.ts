import { Vec2d } from "$lib/common.ts";

export class Compass {
    private calibrationData: Array<Vec2d>;

    private offset: number = $state(0.0);

    private value: number = $state(0.0);

    public available: boolean = $state(false);

    public direction: number = $derived.by(() => {
        return this.value - this.offset;
    });

    public constructor() {
        this.calibrationData = [];

        this.checkAvailability();
    }

    public finishCalibration = (): void => {
        if (!this.available) {
            return;
        }
        window.removeEventListener(
            "deviceorientation",
            this.calibrationHandler,
        );
        if (this.calibrationData.length > 0) {
            const sum = this.calibrationData.reduce(
                (a: Vec2d, b: Vec2d): Vec2d => new Vec2d(a.x + b.x, a.y + b.y),
                new Vec2d(0, 0),
            );
            this.value = this.calibrationData.pop()?.angle() ?? 0.0;
            this.offset = sum.angle();
            this.calibrationData = [];
        }
        window.addEventListener("deviceorientation", this.compassHandler);
    };

    public startCalibration = (): void => {
        if (!this.available) {
            return;
        }
        window.addEventListener("deviceorientation", this.calibrationHandler);
    };

    public stopCalibration = (): void => {
        if (!this.available) {
            return;
        }
        window.removeEventListener(
            "deviceorientation",
            this.calibrationHandler,
        );
        this.calibrationData = [];
    };

    private checkAvailability = (): void => {
        if (window.isSecureContext && "ondeviceorientation" in window) {
            const installOneshotHandler = () =>
                window.addEventListener(
                    "deviceorientation",
                    this.oneShotHandler,
                    { once: true },
                );
            if (
                window.DeviceOrientationEvent &&
                //@ts-expect-error Non standard method on mobile Safari
                window.DeviceOrientationEvent.requestPermission
            ) {
                //@ts-expect-error Non standard method on mobile Safari
                window.DeviceOrientationEvent.requestPermission().then(
                    (permissionState: string) => {
                        if (permissionState === "granted") {
                            installOneshotHandler();
                        }
                    },
                );
            } else {
                installOneshotHandler();
            }
        }
    };

    private calibrationHandler = (event: DeviceOrientationEvent): void => {
        if (event.alpha === null) {
            return;
        }
        // Getting the average of a set of angles is difficult as the average
        // of 1° and 359° is 0° and not 180°, so the values are converted to
        // vectors first.
        this.calibrationData.push(Vec2d.fromPolar(1, event.alpha));
    };

    private compassHandler = (event: DeviceOrientationEvent): void => {
        if (event.alpha === null) {
            return;
        }
        this.value = event.alpha;
    };

    private oneShotHandler = (event: DeviceOrientationEvent): void => {
        this.available = event.alpha !== null;
    };
}
