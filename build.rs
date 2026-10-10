fn main() {
    let manifest_dir =
        std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let include_dir = manifest_dir.join("vst3_c_api");
    let header = include_dir.join("vst3_c_api.h");

    println!("cargo:rerun-if-changed={}", header.display());

    let bindings = bindgen::Builder::default()
        .header(header.to_string_lossy())
        .dynamic_library_name("Vst3")
        .clang_arg(format!("-I{}", include_dir.display()))
        .generate()
        .expect("bindgen failed");

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join("vst3_bindings.rs"))
        .expect("failed to write bindings");
}
