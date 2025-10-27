use log::{info, warn};
use ryzenadj_ffi::{cleanup_ryzenadj, get_cpu_family, init_ryzenadj, ryzen_access};

pub struct RyzenAdj {
    ry: ryzen_access,
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
}

impl Drop for RyzenAdj {
    fn drop(&mut self) {
        info!("Cleaning up ryzenadj");
        unsafe { cleanup_ryzenadj(self.ry) };
    }
}