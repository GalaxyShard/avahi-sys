use std::env;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    if cfg!(not(target_os = "linux")) {
        File::create(out_path).expect("Couldn't create placeholder bindings");
        return;
    }
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-lib=avahi-client");
    println!("cargo:rustc-link-lib=avahi-common");

    bindgen::Builder::default()
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
