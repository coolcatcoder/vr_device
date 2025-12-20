fn main() -> miette::Result<()> {
    let include_path = std::path::PathBuf::from("src");

    // This assumes all your C++ bindings are in main.rs
    let mut b = autocxx_build::Builder::new("src/lib.rs", [&include_path]).build()?;
    b.flag_if_supported("-std=c++14")
     .compile("vr_device"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/lib.rs");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=/home/coolcatcoder/Documents/GitHub/vr_device");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=openvr_api");

    Ok(())
}