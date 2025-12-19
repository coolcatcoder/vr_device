#![warn(clippy::pedantic)]

use std::ffi::{c_char, c_int, c_void};

use crate::ffi::weird_HmdDriverFactory;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vr_device/src/bridge/hmd_driver_factory.h");

        type c_void;
        type c_int;

        unsafe fn weird_HmdDriverFactory(interface_name: *const c_char, return_code: *mut c_int) -> *mut c_void;
    }
	extern "Rust" {
        fn init();
    }
}

#[unsafe(no_mangle)]
extern "C" fn HmdDriverFactory(interface_name: *const c_char, return_code: *mut c_int) -> *mut c_void
{
	unsafe { weird_HmdDriverFactory(interface_name, return_code.cast()).cast() }
}

fn init() {

}