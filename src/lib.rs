//#![warn(clippy::pedantic)]

use std::sync::LazyLock;

use autocxx::{moveit::MakeCppStorage, prelude::*, subclass::{CppSubclassDefault, subclass}};

use crate::ffi::vr::EVRInitError;

mod hmd_driver_factory;
mod device_provider;

include_cpp! {
    #include "openvr_driver.h"
    safety!(unsafe)
    generate!("vr::InitServerDriverContext")
    generate!("vr::IServerTrackedDeviceProvider_Version")
    generate!("vr::IServerTrackedDeviceProvider")
    generate!("vr::IVRDriverLog")
    subclass!("vr::IServerTrackedDeviceProvider", DeviceProvider)
}

// I'll do this by hand if it doesn't work like this.
unsafe extern "C" {
    #[link_name = "\u{1}_ZN2vrL19k_InterfaceVersionsE"]
    pub static k_InterfaceVersions: [*const ::std::os::raw::c_char; 13usize];
}

#[subclass]
#[derive(Default)]
pub struct DeviceProvider;

impl ffi::vr::IServerTrackedDeviceProvider_methods for DeviceProvider {
    unsafe fn Init(&mut self, context: *mut ffi::vr::IVRDriverContext) -> ffi::vr::EVRInitError {
        let error = unsafe { ffi::vr::InitServerDriverContext(context) };
        if error != EVRInitError::VRInitError_None {
            return error
        }

        // driver log
        let driver_log = unsafe { ffi::vr::IVRDriverLog::allocate_uninitialized_cpp_storage() };
        let mut driver_log = unsafe { UniquePtr::from_raw(driver_log) };
        let message = c"Testing!";
        unsafe { driver_log.as_mut().unwrap().Log(message.as_ptr()) };

        EVRInitError::VRInitError_None
    }

    // Secretly this returns a *const *const c_char
    fn GetInterfaceVersions(&mut self) ->  *const autocxx::c_void {
        unsafe { k_InterfaceVersions.as_ptr().cast() }
    }

    fn ShouldBlockStandbyMode(&mut self) -> bool {
        false
    }

    fn RunFrame(&mut self) {
        
    }

    fn EnterStandby(&mut self) {
        
    }

    fn LeaveStandby(&mut self) {
        
    }

    fn Cleanup(&mut self) {
        
    }
}

// static DEVICE_PROVIDER: ffi::vr::IServerTrackedDeviceProvider = ffi::vr::IServerTrackedDeviceProvider {
//     vtable_: null(),
// };

// unsafe extern "C" fn init(
//         this: *mut bindings::ServerTrackedDeviceProvider,
//         context: *mut bindings::vr_IVRDriverContext,
//     ) -> bindings::VrInitError {
//         todo!()
//     }

// static DEVICE_PROVIDER_VTABLE: bindings::ServerTrackedDeviceProviderVtable = bindings::ServerTrackedDeviceProviderVtable {
//     init,
//     cleanup: todo!(),
//     get_interface_versions: todo!(),
//     run_frame: todo!(),
//     should_block_standby_mode: todo!(),
//     enter_standby: todo!(),
//     leave_standby: todo!(),
// };

// static device_provider = bindings::ServerTrackedDeviceProvider {
//     v
// }
