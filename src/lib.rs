//#![warn(clippy::pedantic)]

use std::{ffi::{CStr, CString, c_char}, fs::{File, OpenOptions}, io::Write, ptr::null, sync::LazyLock};

use autocxx::{moveit::MakeCppStorage, prelude::*, subclass::{CppSubclassDefault, subclass}};

use crate::{debugging::log, ffi::vr::EVRInitError};

mod hmd_driver_factory;
mod device_provider;
mod debugging;

include_cpp! {
    #include "openvr_driver.h"
    safety!(unsafe)
    generate!("vr::InitServerDriverContext")
    generate!("vr::IServerTrackedDeviceProvider_Version")
    generate!("vr::IServerTrackedDeviceProvider")
    generate!("vr::IVRDriverLog")
    generate!("vr::VRDriverLog")
    subclass!("vr::IServerTrackedDeviceProvider", DeviceProvider)
}

const k_InterfaceVersions: &[*const c_char; 13] = &[
    IVRSettings_Version.as_ptr(),
    ITrackedDeviceServerDriver_Version.as_ptr(),
    IVRDisplayComponent_Version.as_ptr(),
    IVRDriverDirectModeComponent_Version.as_ptr(),
    IVRCameraComponent_Version.as_ptr(),
    IServerTrackedDeviceProvider_Version.as_ptr(),
    IVRWatchdogProvider_Version.as_ptr(),
    IVRVirtualDisplay_Version.as_ptr(),
    IVRDriverManager_Version.as_ptr(),
    IVRResources_Version.as_ptr(),
    IVRCompositorPluginProvider_Version.as_ptr(),
    IVRIPCResourceManagerClient_Version.as_ptr(),
    null(),
];

const IVRSettings_Version: &CStr = c"IVRSettings_003";
const ITrackedDeviceServerDriver_Version: &CStr = c"ITrackedDeviceServerDriver_005";
const IVRDisplayComponent_Version: &CStr = c"IVRDisplayComponent_003";
const IVRDriverDirectModeComponent_Version: &CStr = c"IVRDriverDirectModeComponent_009";
const IVRCameraComponent_Version: &CStr = c"IVRCameraComponent_003";
const IServerTrackedDeviceProvider_Version: &CStr = c"IServerTrackedDeviceProvider_004";
const IVRWatchdogProvider_Version: &CStr = c"IVRWatchdogProvider_001";
const IVRVirtualDisplay_Version: &CStr = c"IVRVirtualDisplay_002";
const IVRDriverManager_Version: &CStr = c"IVRDriverManager_001";
const IVRResources_Version: &CStr = c"IVRResources_001";
const IVRCompositorPluginProvider_Version: &CStr = c"IVRCompositorPluginProvider_001";
const IVRIPCResourceManagerClient_Version: &CStr = c"IVRIPCResourceManagerClient_002";

#[subclass]
#[derive(Default)]
pub struct DeviceProvider;

impl ffi::vr::IServerTrackedDeviceProvider_methods for DeviceProvider {
    unsafe fn Init(&mut self, context: *mut ffi::vr::IVRDriverContext) -> ffi::vr::EVRInitError {
        let error = unsafe { ffi::vr::InitServerDriverContext(context) };
        if error != EVRInitError::VRInitError_None {
            return error
        }

        debugging::set_panic_hook();

        log(c"Oh no!");

        // Works up to here.
        return EVRInitError::VRInitError_Unknown;

        //let context = unsafe { &mut *context };

        // driver log
        // let driver_log = unsafe { ffi::vr::IVRDriverLog::allocate_uninitialized_cpp_storage() };
        // let mut driver_log = unsafe { UniquePtr::from_raw(driver_log) };
        // let message = c"Testing!";
        // unsafe { driver_log.as_mut().unwrap().Log(message.as_ptr()) };

        EVRInitError::VRInitError_None
    }

    // Secretly this returns a *const *const c_char
    fn GetInterfaceVersions(&mut self) ->  *const autocxx::c_void {
        k_InterfaceVersions.as_ptr().cast()
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
