use std::{ffi::{CStr, c_char, c_int, c_void}, ptr::null_mut};
use crate::bindings::{self, VrInitError};

#[unsafe(no_mangle)]
extern "C" fn HmdDriverFactory(interface_name: *const c_char, return_code: *mut c_int) -> *mut c_void
{
    let interface_name = unsafe { CStr::from_ptr(interface_name) };

    if bindings::SERVER_TRACKED_DEVICE_PROVIDER_VERSION == interface_name {
        // let device_provider_vtable = Box::new(raw_bindings::vr_IServerTrackedDeviceProvider__bindgen_vtable {
        //     vr_IServerTrackedDeviceProvider_
        // });
        // let device_provider = raw_bindings::vr_IServerTrackedDeviceProvider {
        //     vtable_: vtable,
        // };

        todo!()
    }
    
    unsafe {
        *return_code = VrInitError::InitInterfaceNotFound as c_int;
        null_mut()
    }
}