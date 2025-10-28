export class ApuTuningConfig implements ApuTuningType {
    temperature_limit: number | null = null;
    skin_temperature_limit: number | null = null;
    stapm_power_limit: number | null = null;
    slow_power_limit: number | null = null;
    slow_boost_duration: number | null = null;
    fast_power_limit: number | null = null;
    fast_boost_duration: number | null = null;

    constructor(config?: Partial<ApuTuningConfig>) {
        Object.assign(this, config)
    }
}
