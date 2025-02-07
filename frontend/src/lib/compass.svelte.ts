export class Compass {
    private calibrationData: Array<number>;

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
                (a: number, b: number): number => a + b,
                0,
            );
            this.offset = sum / this.calibrationData.length;
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
                window.DeviceOrientationEvent.requestPermission
            ) {
                window.DeviceOrientationEvent.requestPermission().then(
                    (permissionState) => {
                        if (permissionState === "granted") {
                            installOneshotHandler();
                        }
                    },
                );
            } else {
                installOneshotHandler();
            }
        }
    }

    private calibrationHandler = (event: DeviceOrientation): void => {
        this.calibrationData.push(event.alpha);
    };

    private compassHandler = (event: DeviceOrientation): void => {
        this.value = event.alpha;
    };

    private oneShotHandler = (event: DeviceOrientation): void => {
        this.available = event.alpha !== null;
    };
}
