/* eslint-disable @typescript-eslint/no-extraneous-class */
/* eslint-disable @typescript-eslint/no-empty-object-type */
/* eslint-disable @typescript-eslint/no-unsafe-declaration-merging */

const createNullConfig = (): ApuTuningConfig => {
    return Object.fromEntries(
        Object.keys({} as ApuTuningConfig).map(key => [key, null])
    ) as ApuTuningConfig;
};


export class ApuTuning {
    constructor(config?: Partial<ApuTuningConfig>) {
        const defaults = createNullConfig();
        Object.assign(this, defaults, config);
    }
}

export interface ApuTuning extends ApuTuningConfig { }
