fn main() {
    cxx_build::bridge("src/lib.rs")
        //.file("src/bridge")
        .file("src/bridge/hmd_driver_factory.cpp")
        //.std("c++14")
        .compile("vr_device");

    println!("cargo:rerun-if-changed=src");
    println!("cargo:rustc-link-search=/home/coolcatcoder/Documents/GitHub/vr_device/libopenvr_api.so");
    println!("cargo:rustc-link-lib=libopenvr_api.so");
}