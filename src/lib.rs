//#![warn(clippy::pedantic)]

use std::{ffi::{CStr, c_char, c_int, c_void}, ptr::null_mut};

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
#[allow(clippy::pedantic)]
mod raw_bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[unsafe(no_mangle)]
extern "C" fn HmdDriverFactory(interface_name: *const c_char, return_code: *mut c_int) -> *mut c_void
{
    let interface_name = unsafe { CStr::from_ptr(interface_name) };
    if CStr::from_bytes_with_nul(raw_bindings::vr_IServerTrackedDeviceProvider_Version).expect("Valid CStr.") == interface_name {
        // let device_provider_vtable = Box::new(raw_bindings::vr_IServerTrackedDeviceProvider__bindgen_vtable {
        //     vr_IServerTrackedDeviceProvider_
        // });
        // let device_provider = raw_bindings::vr_IServerTrackedDeviceProvider {
        //     vtable_: vtable,
        // };

        todo!()
    }
    
    unsafe {
        *return_code = raw_bindings::vr_EVRInitError_VRInitError_Init_InterfaceNotFound as c_int;
        null_mut()
    }
}
