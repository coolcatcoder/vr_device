use std::{ffi::{CStr, c_char, c_int, c_void}, ptr::null_mut};

use autocxx::subclass::CppSubclassDefault;

use crate::{DeviceProvider, ffi::vr::{EVRInitError, IServerTrackedDeviceProvider_Version}};

#[unsafe(no_mangle)]
extern "C" fn HmdDriverFactory(interface_name: *const c_char, return_code: *mut c_int) -> *mut c_void
{
    let interface_name = unsafe { CStr::from_ptr(interface_name) };

    if CStr::from_bytes_with_nul(IServerTrackedDeviceProvider_Version).expect("Valid CStr") == interface_name {
        // This is foolish, and almost certainly going to behave strangely.
        let device_provider = DeviceProvider::default_rust_owned();
        // Leaking this should cause the rc to never be destroyed.
        let device_provider_leaker = device_provider.clone();
        std::mem::forget(device_provider_leaker);
        let device_provider_pointer = device_provider.as_ptr();

        return device_provider_pointer.cast();
    }
    
    unsafe {
        *return_code = EVRInitError::VRInitError_Init_InterfaceNotFound as c_int;
        null_mut()
    }
}