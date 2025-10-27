use ryzen_tuner::RyzenAdj;

fn main() {
    match RyzenAdj::new() {
        Ok(ryzenadj) => {
            println!("CPU Family: {}", ryzenadj.get_cpu_family());
            println!("STAPM Limit: {}", ryzenadj.get_stapm_limit());
            println!("Fast Limit: {}", ryzenadj.get_fast_limit());
            println!("Slow Limit: {}", ryzenadj.get_slow_limit());
            println!("Slow Time: {}", ryzenadj.get_slow_time());
            println!("STAPM Time: {}", ryzenadj.get_stapm_time());
            println!("Tctl Temp: {}", ryzenadj.get_tctl_temp());
            println!("VRM Current: {}", ryzenadj.get_vrm_current());
            println!("VRM SoC Current: {}", ryzenadj.get_vrmsoc_current());
            println!("VRM Max Current: {}", ryzenadj.get_vrmmax_current());
            println!("VRM SoC Max Current: {}", ryzenadj.get_vrmsocmax_current());
            println!("PSI0 Current: {}", ryzenadj.get_psi0_current());
            println!("PSI0 SoC Current: {}", ryzenadj.get_psi0soc_current());
            println!("APU Skin Temp Limit: {}", ryzenadj.get_apu_skin_temp_limit());
            println!("dGPU Skin Temp Limit: {}", ryzenadj.get_dgpu_skin_temp_limit());
            println!("APU Slow Limit: {}", ryzenadj.get_apu_slow_limit());
        }
        Err(_) => {
            eprintln!("Failed to initialize RyzenAdj");
        }
    }
}
