use log::{info, warn};
use ryzenadj_ffi::{cleanup_ryzenadj, get_cpu_family, init_ryzenadj, init_table, ryzen_access};

pub struct RyzenAdj {
    ry: ryzen_access,
}

macro_rules! set_value {
    ($func_name:ident, $ffi_func:ident, $doc:expr) => {
        #[doc = $doc]
        pub fn $func_name(&self, value: u32) -> Result<(), i32> {
            let ret = unsafe { ryzenadj_ffi::$ffi_func(self.ry, value) };
            if ret == 0 { Ok(()) } else { Err(ret) }
        }
    };
}

impl RyzenAdj {
    pub fn new() -> Result<Self, ()> {
        let ry = unsafe { init_ryzenadj() };
        if ry.is_null() {
            warn!("Failed to initialize ryzenadj");
            return Err(());
        }

        let ret = unsafe { init_table(ry) };
        if ret != 0 {
            warn!("Failed to initialize PM table");
            unsafe { cleanup_ryzenadj(ry) };
            return Err(());
        }

        info!("Successfully initialized ryzenadj");
        Ok(Self { ry })
    }

    pub fn get_cpu_family(&self) -> i32 {
        unsafe { get_cpu_family(self.ry) }
    }

    set_value!(set_stapm_limit, set_stapm_limit, "Sustained Power Limit (STAPM) in mW");
    set_value!(set_fast_limit, set_fast_limit, "Fast PPT Limit in mW");
    set_value!(set_slow_limit, set_slow_limit, "Slow PPT Limit in mW");
    set_value!(set_slow_time, set_slow_time, "Slow PPT Constant Time in seconds");
    set_value!(set_stapm_time, set_stapm_time, "STAPM Constant Time in seconds");
    set_value!(set_tctl_temp, set_tctl_temp, "Tctl Temperature Limit in degree C");
    set_value!(set_vrm_current, set_vrm_current, "VRM Current Limit (TDC) in mA");
    set_value!(set_vrmsoc_current, set_vrmsoc_current, "VRM SoC Current Limit (TDC) in mA");
    set_value!(set_vrmmax_current, set_vrmmax_current, "VRM Maximum Current Limit (EDC) in mA");
    set_value!(set_vrmsocmax_current, set_vrmsocmax_current, "VRM SoC Maximum Current Limit (EDC) in mA");
    set_value!(set_psi0_current, set_psi0_current, "PSI0 VDD Current Limit in mA");
    set_value!(set_psi0soc_current, set_psi0soc_current, "PSI0 SoC Current Limit in mA");
    set_value!(set_max_socclk_freq, set_max_socclk_freq, "Maximum SoC Clock Frequency in MHz");
    set_value!(set_min_socclk_freq, set_min_socclk_freq, "Minimum SoC Clock Frequency in MHz");
    set_value!(set_max_fclk_freq, set_max_fclk_freq, "Maximum FCLK Frequency in MHz");
    set_value!(set_min_fclk_freq, set_min_fclk_freq, "Minimum FCLK Frequency in MHz");
    set_value!(set_max_vcn, set_max_vcn, "Maximum VCN Frequency in MHz");
    set_value!(set_min_vcn, set_min_vcn, "Minimum VCN Frequency in MHz");
    set_value!(set_max_lclk, set_max_lclk, "Maximum LCLK Frequency in MHz");
    set_value!(set_min_lclk, set_min_lclk, "Minimum LCLK Frequency in MHz");
    set_value!(set_max_gfxclk_freq, set_max_gfxclk_freq, "Maximum GFX Clock Frequency in MHz");
    set_value!(set_min_gfxclk_freq, set_min_gfxclk_freq, "Minimum GFX Clock Frequency in MHz");
    set_value!(
        set_prochot_deassertion_ramp,
        set_prochot_deassertion_ramp,
        "Prochot Deassertion Ramp Time"
    );
    set_value!(set_apu_skin_temp_limit, set_apu_skin_temp_limit, "APU Skin Temperature Limit (STT) in degree C");
    set_value!(set_dgpu_skin_temp_limit, set_dgpu_skin_temp_limit, "dGPU Skin Temperature Limit (STT) in degree C");
    set_value!(set_apu_slow_limit, set_apu_slow_limit, "APU Slow PPT Limit in mW");
    set_value!(set_skin_temp_power_limit, set_skin_temp_power_limit, "Skin Temperature Power Limit in mW");
}

impl Drop for RyzenAdj {
    fn drop(&mut self) {
        info!("Cleaning up ryzenadj");
        unsafe { cleanup_ryzenadj(self.ry) };
    }
}
