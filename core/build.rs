use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if cfg!(windows) {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let target_dir = out_dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap();

        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dll_dir = manifest_dir
            .join("..")
            .join("ryzenadj-ffi")
            .join("ryzenadj")
            .join("win32");

        let files_to_copy = ["inpoutx64.dll", "WinRing0x64.dll", "WinRing0x64.sys"];

        for file_name in files_to_copy.iter() {
            let src_path = dll_dir.join(file_name);
            let dest_path = target_dir.join(file_name);
            // Ignore errors in case the file already exists
            let _ = fs::copy(&src_path, &dest_path);
        }
    }
}
