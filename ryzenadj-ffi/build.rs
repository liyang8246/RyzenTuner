use std::env;
use std::path::PathBuf;

fn main() {
    // Build the C library
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .define("NDEBUG", None)
        .define("_LIBRYZENADJ_INTERNAL", None)
        .include("ryzenadj/lib")
        .include("ryzenadj/lib/win32")
        .file("ryzenadj/main.c")
        .file("ryzenadj/lib/api.c")
        .file("ryzenadj/lib/cpuid.c")
        .file("ryzenadj/lib/nb_smu_ops.c")
        .file("ryzenadj/lib/win32/osdep_win32.cpp");

    if cfg!(windows) {
        build.define("_WIN32", None);
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let lib_dir = PathBuf::from(manifest_dir).join("ryzenadj").join("win32");
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=inpoutx64");
        println!("cargo:rustc-link-lib=WinRing0x64");
    } else if cfg!(unix) {
        build
            .file("ryzenadj/lib/linux/osdep_linux.c")
            .file("ryzenadj/lib/linux/osdep_linux_mem.c")
            .file("ryzenadj/lib/linux/osdep_linux_smu_kernel_module.c");
    }

    build.compile("ryzenadj");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_file(".*ryzenadj.h")
        .merge_extern_blocks(true)
        .clang_arg("-D_LIBRYZENADJ_INTERNAL")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Rerun if changed
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=ryzenadj/");
}
