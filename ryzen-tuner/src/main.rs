use ryzen_tuner::RyzenAdj;

fn main() {
    match RyzenAdj::new() {
        Ok(ryzenadj) => {
            let cpu_family = ryzenadj.get_cpu_family();
            println!("CPU Family: {}", cpu_family);
        }
        Err(_) => {
            eprintln!("Failed to initialize RyzenAdj");
        }
    }
}
