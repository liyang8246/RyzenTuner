/* eslint-disable @typescript-eslint/no-extraneous-class */
/* eslint-disable @typescript-eslint/no-empty-object-type */
/* eslint-disable @typescript-eslint/no-unsafe-declaration-merging */

const createNullConfig = (): ApuTuningType => {
    return Object.fromEntries(
        Object.keys({} as ApuTuningType).map(key => [key, null])
    ) as ApuTuningType;
};


export class ApuTuningConfig {
    constructor(config?: Partial<ApuTuningType>) {
        const defaults = createNullConfig();
        Object.assign(this, defaults, config);
    }
}

export interface ApuTuningConfig extends ApuTuningType { }
