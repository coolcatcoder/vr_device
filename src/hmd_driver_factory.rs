use std::{
    ffi::{CStr, c_char, c_int, c_void},
    ptr::null_mut,
};

use autocxx::subclass::CppSubclassDefault;

use crate::{
    DeviceProvider,
    ffi::vr::{EVRInitError, IServerTrackedDeviceProvider_Version},
};

#[unsafe(no_mangle)]
extern "C" fn HmdDriverFactory(
    interface_name: *const c_char,
    return_code: *mut c_int,
) -> *mut c_void {
    // let mut driver_log = unsafe { UniquePtr::from_raw(ffi::vr::VRDriverLog()) };
    // let message = c"Testing!";
    // unsafe { driver_log.pin_mut().Log(message.as_ptr()) };
    // driver_log.into_raw();

    //panic!();

    let interface_name = unsafe { CStr::from_ptr(interface_name) };

    if CStr::from_bytes_with_nul(IServerTrackedDeviceProvider_Version).expect("Valid CStr")
        == interface_name
    {
        // This should leak the device provider, and make it valid forever.
        let device_provider = DeviceProvider::default_cpp_owned().into_raw();
        return device_provider.cast();
    }

    unsafe {
        *return_code = EVRInitError::VRInitError_Init_InterfaceNotFound as c_int;
        null_mut()
    }
}
