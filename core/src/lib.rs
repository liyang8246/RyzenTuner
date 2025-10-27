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

    pub fn get_cpu_family(&self) -> &'static str {
        let family_id = unsafe { get_cpu_family(self.ry) };
        match family_id {
            0 => "Raven",
            1 => "Picasso",
            2 => "Renoir",
            3 => "Cezanne",
            4 => "Dali",
            5 => "Lucienne",
            6 => "Vangogh",
            7 => "Rembrandt",
            8 => "Mendocino",
            9 => "Phoenix",
            10 => "Hawkpoint",
            11 => "Dragonrange",
            12 => "Krackanpoint",
            13 => "Strixpoint",
            14 => "Strixhalo",
            15 => "Firerange",
            _ => "Unknown",
        }
    }

    set_value!(set_stapm_limit, set_stapm_limit, "Sustained Power Limit (STAPM) in mW");
    set_value!(set_fast_limit, set_fast_limit, "Fast PPT Limit in mW");
    set_value!(set_slow_limit, set_slow_limit, "Slow PPT Limit in mW");
    set_value!(set_slow_time, set_slow_time, "Slow PPT Constant Time in seconds");
    set_value!(set_stapm_time, set_stapm_time, "STAPM Constant Time in seconds");
    set_value!(set_tctl_temp, set_tctl_temp, "Tctl Temperature Limit in degree C");
    set_value!(set_vrm_current, set_vrm_current, "VRM Current Limit (TDC) in mA");
    set_value!(
        set_vrmsoc_current,
        set_vrmsoc_current,
        "VRM SoC Current Limit (TDC) in mA"
    );
    set_value!(
        set_vrmmax_current,
        set_vrmmax_current,
        "VRM Maximum Current Limit (EDC) in mA"
    );
    set_value!(
        set_vrmsocmax_current,
        set_vrmsocmax_current,
        "VRM SoC Maximum Current Limit (EDC) in mA"
    );
    set_value!(set_psi0_current, set_psi0_current, "PSI0 VDD Current Limit in mA");
    set_value!(set_psi0soc_current, set_psi0soc_current, "PSI0 SoC Current Limit in mA");
    set_value!(
        set_max_socclk_freq,
        set_max_socclk_freq,
        "Maximum SoC Clock Frequency in MHz"
    );
    set_value!(
        set_min_socclk_freq,
        set_min_socclk_freq,
        "Minimum SoC Clock Frequency in MHz"
    );
    set_value!(set_max_fclk_freq, set_max_fclk_freq, "Maximum FCLK Frequency in MHz");
    set_value!(set_min_fclk_freq, set_min_fclk_freq, "Minimum FCLK Frequency in MHz");
    set_value!(set_max_vcn, set_max_vcn, "Maximum VCN Frequency in MHz");
    set_value!(set_min_vcn, set_min_vcn, "Minimum VCN Frequency in MHz");
    set_value!(set_max_lclk, set_max_lclk, "Maximum LCLK Frequency in MHz");
    set_value!(set_min_lclk, set_min_lclk, "Minimum LCLK Frequency in MHz");
    set_value!(
        set_max_gfxclk_freq,
        set_max_gfxclk_freq,
        "Maximum GFX Clock Frequency in MHz"
    );
    set_value!(
        set_min_gfxclk_freq,
        set_min_gfxclk_freq,
        "Minimum GFX Clock Frequency in MHz"
    );
    set_value!(
        set_prochot_deassertion_ramp,
        set_prochot_deassertion_ramp,
        "Prochot Deassertion Ramp Time"
    );
    set_value!(
        set_apu_skin_temp_limit,
        set_apu_skin_temp_limit,
        "APU Skin Temperature Limit (STT) in degree C"
    );
    set_value!(
        set_dgpu_skin_temp_limit,
        set_dgpu_skin_temp_limit,
        "dGPU Skin Temperature Limit (STT) in degree C"
    );
    set_value!(set_apu_slow_limit, set_apu_slow_limit, "APU Slow PPT Limit in mW");
    set_value!(
        set_skin_temp_power_limit,
        set_skin_temp_power_limit,
        "Skin Temperature Power Limit in mW"
    );
}

macro_rules! get_value {
    ($func_name:ident, $ffi_func:ident, $doc:expr) => {
        #[doc = $doc]
        pub fn $func_name(&self) -> f32 {
            unsafe { ryzenadj_ffi::$ffi_func(self.ry) }
        }
    };
}

impl RyzenAdj {
    get_value!(get_stapm_limit, get_stapm_limit, "Sustained Power Limit (STAPM) in mW");
    get_value!(get_fast_limit, get_fast_limit, "Fast PPT Limit in mW");
    get_value!(get_slow_limit, get_slow_limit, "Slow PPT Limit in mW");
    get_value!(get_slow_time, get_slow_time, "Slow PPT Constant Time in seconds");
    get_value!(get_stapm_time, get_stapm_time, "STAPM Constant Time in seconds");
    get_value!(get_tctl_temp, get_tctl_temp, "Tctl Temperature Limit in degree C");
    get_value!(get_vrm_current, get_vrm_current, "VRM Current Limit (TDC) in mA");
    get_value!(
        get_vrmsoc_current,
        get_vrmsoc_current,
        "VRM SoC Current Limit (TDC) in mA"
    );
    get_value!(
        get_vrmmax_current,
        get_vrmmax_current,
        "VRM Maximum Current Limit (EDC) in mA"
    );
    get_value!(
        get_vrmsocmax_current,
        get_vrmsocmax_current,
        "VRM SoC Maximum Current Limit (EDC) in mA"
    );
    get_value!(get_psi0_current, get_psi0_current, "PSI0 VDD Current Limit in mA");
    get_value!(get_psi0soc_current, get_psi0soc_current, "PSI0 SoC Current Limit in mA");
    get_value!(
        get_apu_skin_temp_limit,
        get_apu_skin_temp_limit,
        "APU Skin Temperature Limit (STT) in degree C"
    );
    get_value!(
        get_dgpu_skin_temp_limit,
        get_dgpu_skin_temp_limit,
        "dGPU Skin Temperature Limit (STT) in degree C"
    );
    get_value!(get_apu_slow_limit, get_apu_slow_limit, "APU Slow PPT Limit in mW");

    get_value!(get_stapm_value, get_stapm_value, "Sustained Power Value (STAPM) in mW");
    get_value!(get_fast_value, get_fast_value, "Fast PPT Value in mW");
    get_value!(get_slow_value, get_slow_value, "Slow PPT Value in mW");
    get_value!(get_apu_slow_value, get_apu_slow_value, "APU Slow PPT Value in mW");
    get_value!(get_vrm_current_value, get_vrm_current_value, "VRM Current Value (TDC) in mA");
    get_value!(get_vrmsoc_current_value, get_vrmsoc_current_value, "VRM SoC Current Value (TDC) in mA");
    get_value!(get_vrmmax_current_value, get_vrmmax_current_value, "VRM Maximum Current Value (EDC) in mA");
    get_value!(get_vrmsocmax_current_value, get_vrmsocmax_current_value, "VRM SoC Maximum Current Value (EDC) in mA");
    get_value!(get_tctl_temp_value, get_tctl_temp_value, "Tctl Temperature Value in degree C");
    get_value!(get_apu_skin_temp_value, get_apu_skin_temp_value, "APU Skin Temperature Value (STT) in degree C");
    get_value!(get_dgpu_skin_temp_value, get_dgpu_skin_temp_value, "dGPU Skin Temperature Value (STT) in degree C");

    get_value!(get_cclk_setpoint, get_cclk_setpoint, "CCLK Boost Setpoint");
    get_value!(get_cclk_busy_value, get_cclk_busy_value, "CCLK Busy Value");
    get_value!(get_l3_clk, get_l3_clk, "L3 Clock in MHz");
    get_value!(get_l3_logic, get_l3_logic, "L3 Logic");
    get_value!(get_l3_vddm, get_l3_vddm, "L3 VDDM");
    get_value!(get_l3_temp, get_l3_temp, "L3 Temperature in degree C");
    get_value!(get_gfx_clk, get_gfx_clk, "GFX Clock in MHz");
    get_value!(get_gfx_temp, get_gfx_temp, "GFX Temperature in degree C");
    get_value!(get_gfx_volt, get_gfx_volt, "GFX Voltage in V");
    get_value!(get_mem_clk, get_mem_clk, "Memory Clock in MHz");
    get_value!(get_fclk, get_fclk, "FCLK in MHz");
    get_value!(get_soc_power, get_soc_power, "SoC Power in W");
    get_value!(get_soc_volt, get_soc_volt, "SoC Voltage in V");
    get_value!(get_socket_power, get_socket_power, "Socket Power in W");
}

macro_rules! get_core_value {
    ($func_name:ident, $ffi_func:ident, $doc:expr) => {
        #[doc = $doc]
        pub fn $func_name(&self, core: u32) -> f32 {
            unsafe { ryzenadj_ffi::$ffi_func(self.ry, core) }
        }
    };
}

impl RyzenAdj {
    get_core_value!(get_core_clk, get_core_clk, "Get Core Clock in MHz");
    get_core_value!(get_core_volt, get_core_volt, "Get Core Voltage in V");
    get_core_value!(get_core_power, get_core_power, "Get Core Power in W");
    get_core_value!(get_core_temp, get_core_temp, "Get Core Temperature in degree C");
}

impl Drop for RyzenAdj {
    fn drop(&mut self) {
        info!("Cleaning up ryzenadj");
        unsafe { cleanup_ryzenadj(self.ry) };
    }
}
