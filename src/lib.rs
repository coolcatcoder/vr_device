//#![warn(clippy::pedantic)]

use std::{ffi::{CStr, c_char, c_int, c_void}, ptr::null_mut};

// bindgen wrapper.hpp -o ./src/new_bindings.rs --vtable-generation --allowlist-item "vr::IServerTrackedDeviceProvider" --allowlist-item "vr::IServerTrackedDeviceProvider_Version"
mod bindings;

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
        *return_code = bindings::VrInitError::InitInterfaceNotFound as c_int;
        null_mut()
    }
}

unsafe extern "C" fn init(
        this: *mut bindings::ServerTrackedDeviceProvider,
        context: *mut bindings::vr_IVRDriverContext,
    ) -> bindings::VrInitError {
        todo!()
    }

static DEVICE_PROVIDER_VTABLE: bindings::ServerTrackedDeviceProviderVtable = bindings::ServerTrackedDeviceProviderVtable {
    init,
    cleanup: todo!(),
    get_interface_versions: todo!(),
    run_frame: todo!(),
    should_block_standby_mode: todo!(),
    enter_standby: todo!(),
    leave_standby: todo!(),
};

// static device_provider = bindings::ServerTrackedDeviceProvider {
//     v
// }
