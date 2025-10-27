use log::{info, warn};
use ryzenadj_ffi::{cleanup_ryzenadj, get_cpu_family, init_ryzenadj, ryzen_access};

pub struct RyzenAdj {
    ry: ryzen_access,
}

macro_rules! set_value {
    ($func_name:ident, $ffi_func:ident) => {
        pub fn $func_name(&self, value: u32) -> Result<(), i32> {
            let ret = unsafe { ryzenadj_ffi::$ffi_func(self.ry, value) };
            if ret == 0 {
                Ok(())
            } else {
                Err(ret)
            }
        }
    };
    ($func_name:ident, $ffi_func:ident, no_arg) => {
        pub fn $func_name(&self) -> Result<(), i32> {
            let ret = unsafe { ryzenadj_ffi::$ffi_func(self.ry) };
            if ret == 0 {
                Ok(())
            } else {
                Err(ret)
            }
        }
    };
}

impl RyzenAdj {
    pub fn new() -> Result<Self, ()> {
        let ry = unsafe { init_ryzenadj() };
        if ry.is_null() {
            warn!("Failed to initialize ryzenadj");
            Err(())
        } else {
            info!("Successfully initialized ryzenadj");
            Ok(Self { ry })
        }
    }

    pub fn get_cpu_family(&self) -> i32 {
        unsafe { get_cpu_family(self.ry) }
    }

    set_value!(set_stapm_limit, set_stapm_limit);
    set_value!(set_fast_limit, set_fast_limit);
    set_value!(set_slow_limit, set_slow_limit);
    set_value!(set_slow_time, set_slow_time);
    set_value!(set_stapm_time, set_stapm_time);
    set_value!(set_tctl_temp, set_tctl_temp);
    set_value!(set_vrm_current, set_vrm_current);
    set_value!(set_vrmsoc_current, set_vrmsoc_current);
    set_value!(set_vrmmax_current, set_vrmmax_current);
    set_value!(set_vrmsocmax_current, set_vrmsocmax_current);
    set_value!(set_psi0_current, set_psi0_current);
    set_value!(set_psi0soc_current, set_psi0soc_current);
    set_value!(set_max_socclk_freq, set_max_socclk_freq);
    set_value!(set_min_socclk_freq, set_min_socclk_freq);
    set_value!(set_max_fclk_freq, set_max_fclk_freq);
    set_value!(set_min_fclk_freq, set_min_fclk_freq);
    set_value!(set_max_vcn, set_max_vcn);
    set_value!(set_min_vcn, set_min_vcn);
    set_value!(set_max_lclk, set_max_lclk);
    set_value!(set_min_lclk, set_min_lclk);
    set_value!(set_max_gfxclk_freq, set_max_gfxclk_freq);
    set_value!(set_min_gfxclk_freq, set_min_gfxclk_freq);
    set_value!(set_prochot_deassertion_ramp, set_prochot_deassertion_ramp);
    set_value!(set_apu_skin_temp_limit, set_apu_skin_temp_limit);
    set_value!(set_dgpu_skin_temp_limit, set_dgpu_skin_temp_limit);
    set_value!(set_apu_slow_limit, set_apu_slow_limit);
    set_value!(set_skin_temp_power_limit, set_skin_temp_power_limit);
    set_value!(set_power_saving, set_power_saving, no_arg);
    set_value!(set_max_performance, set_max_performance, no_arg);
}

impl Drop for RyzenAdj {
    fn drop(&mut self) {
        info!("Cleaning up ryzenadj");
        unsafe { cleanup_ryzenadj(self.ry) };
    }
}
