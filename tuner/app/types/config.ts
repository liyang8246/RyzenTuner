export class ApuTuningConfig implements ApuTuningConfig {
    temperatureLimit: number | null = null;
    skinTemperatureLimit: number | null = null;
    stapmPowerLimit: number | null = null;
    slowPowerLimit: number | null = null;
    slowBoostDuration: number | null = null;
    fastPowerLimit: number | null = null;
    fastBoostDuration: number | null = null;

    constructor(config?: Partial<ApuTuningConfig>) {
        Object.assign(this, config)
    }
}
